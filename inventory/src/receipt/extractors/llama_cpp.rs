use std::path::PathBuf;

use image::DynamicImage;
use image::codecs::jpeg::JpegEncoder;
use jiff::Timestamp;
use serde::{
    Deserialize,
    Serialize,
};
use uuid::Uuid;

use crate::errors::{
    InventoryError,
    InventoryResult,
};
use crate::receipt::extractors::raw::RawReceipt;
use crate::receipt::extractors::{
    ExtractedReceipt,
    ReceiptExtractor,
};

const EXTRACTION_PROMPT: &str = "You are a receipt scanner. Transcribe the receipt in the image \
    into the requested schema. Copy amounts exactly as printed, without recomputing them, and put \
    the full unmodified text into `raw_text`. Give `currency_code` as an ISO-4217 code, dates as \
    YYYY-MM-DD and times as HH:MM:SS. Omit fields the receipt does not show.";

const JPEG_QUALITY: u8 = 90;

/// Must match the `--media-path` the server was started with.
const MEDIA_PATH: &str = "/tmp";

pub struct LlamaCppExtractor {
    endpoint: String,
    client: reqwest::Client,
}

#[derive(Serialize)]
struct RequestShape {
    messages: Vec<Message>,
    temperature: f32,
    response_format: ResponseFormat,
}

#[derive(Serialize)]
#[serde(tag = "role", rename_all = "snake_case")]
enum Message {
    System { content: String },
    User { content: Vec<ContentPart> },
}

#[derive(Serialize)]
#[serde(tag = "type", rename_all = "snake_case")]
enum ContentPart {
    Text { text: String },
    ImageUrl { image_url: ImageUrl },
}

#[derive(Serialize)]
struct ImageUrl {
    url: String,
}

#[derive(Serialize)]
#[serde(tag = "type", rename_all = "snake_case")]
enum ResponseFormat {
    JsonSchema { json_schema: SchemaSpec },
}

#[derive(Serialize)]
struct SchemaSpec {
    name: String,
    schema: schemars::Schema,
}

#[derive(Deserialize)]
struct ResponseShape {
    choices: Vec<Choice>,
}

#[derive(Deserialize)]
struct Choice {
    message: ResponseMessage,
}

#[derive(Deserialize)]
struct ResponseMessage {
    content: String,
}

impl RequestShape {
    fn for_receipt(image_url: String) -> Self {
        Self {
            messages: vec![
                Message::System {
                    content: EXTRACTION_PROMPT.to_owned(),
                },
                Message::User {
                    content: vec![
                        ContentPart::Text {
                            text: "Extract this receipt.".to_owned(),
                        },
                        ContentPart::ImageUrl {
                            image_url: ImageUrl { url: image_url },
                        },
                    ],
                },
            ],
            temperature: 0.0,
            response_format: ResponseFormat::JsonSchema {
                json_schema: SchemaSpec {
                    name: "receipt".to_owned(),
                    schema: schemars::schema_for!(RawReceipt),
                },
            },
        }
    }
}

/// An image parked in the server's media path, removed again once dropped.
struct MediaFile {
    path: PathBuf,
    name: String,
}

impl MediaFile {
    fn write(image: &DynamicImage) -> InventoryResult<Self> {
        let rgb = image.to_rgb8();
        let mut jpeg = Vec::new();
        JpegEncoder::new_with_quality(&mut jpeg, JPEG_QUALITY).encode_image(&rgb)?;

        let name = format!("zeni-receipt-{}.jpg", Uuid::new_v7(zeni_finance::uuid_timestamp(Timestamp::now())));
        let path = PathBuf::from(MEDIA_PATH).join(&name);
        std::fs::write(&path, jpeg)?;

        Ok(Self { path, name })
    }

    fn url(&self) -> String {
        format!("file://{}", self.name)
    }
}

impl Drop for MediaFile {
    fn drop(&mut self) {
        let _ = std::fs::remove_file(&self.path);
    }
}

impl LlamaCppExtractor {
    pub fn new(endpoint: String) -> Self {
        Self {
            endpoint,
            client: reqwest::Client::new(),
        }
    }

    fn completions_url(&self) -> String {
        format!("{}/v1/chat/completions", self.endpoint.trim_end_matches('/'))
    }
}

impl ReceiptExtractor for LlamaCppExtractor {
    async fn extract_image(
        &mut self,
        image: &DynamicImage,
    ) -> InventoryResult<ExtractedReceipt> {
        let media_file = MediaFile::write(image)?;

        let response = self
            .client
            .post(self.completions_url())
            .json(&RequestShape::for_receipt(media_file.url()))
            .send()
            .await?
            .error_for_status()?
            .json::<ResponseShape>()
            .await?;

        let content = response
            .choices
            .into_iter()
            .next()
            .ok_or(InventoryError::EmptyResponse)?
            .message
            .content;

        eprintln!("content = {:#?}", content);

        serde_json::from_str::<RawReceipt>(&content)?.into_extraction()
    }
}
