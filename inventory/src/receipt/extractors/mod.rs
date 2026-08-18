#[cfg(feature = "extractors")]
use image::DynamicImage;
use serde::{
    Deserialize,
    Serialize,
};
use zeni_finance::money::Money;

#[cfg(feature = "extractors")]
use crate::errors::InventoryResult;
use crate::receipt::{
    Quantity,
    ReceiptHeader,
};

#[cfg(feature = "extractors")]
pub mod anthropic;
#[cfg(feature = "extractors")]
pub mod llama_cpp;
#[cfg(feature = "extractors")]
mod raw;

#[derive(Debug, Serialize, Deserialize)]
pub struct ExtractedReceipt {
    pub header: ReceiptHeader,
    pub articles: Vec<ExtractedArticle>,
}

/// An [ExtractedArticle] has its cross-reference to the underlying item not yet resolved.
#[derive(Debug, Serialize, Deserialize)]
pub struct ExtractedArticle {
    pub description: String,
    pub unit_price: Money,
    pub quantity: Quantity,
    pub total: Money,
}

#[cfg(feature = "extractors")]
pub trait ReceiptExtractor {
    fn extract_image(
        &mut self,
        image: &DynamicImage,
    ) -> impl Future<Output = InventoryResult<ExtractedReceipt>> + Send;
}
