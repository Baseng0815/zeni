use crate::receipt::extractors::{
    ExtractedReceipt,
    ReceiptExtractor,
};

pub struct AnthropicExtractor {
    api_key: String,
}

impl AnthropicExtractor {
    pub fn new(api_key: String) -> Self {
        Self { api_key }
    }
}

impl ReceiptExtractor for AnthropicExtractor {
    async fn extract_image(
        &mut self,
        image: &image::DynamicImage,
    ) -> crate::errors::InventoryResult<ExtractedReceipt> {
        todo!()
    }
}
