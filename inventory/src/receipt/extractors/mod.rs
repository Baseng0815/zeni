use image::DynamicImage;
use zeni_finance::money::Money;

use crate::errors::InventoryResult;
use crate::receipt::{
    Quantity,
    ReceiptHeader,
};

pub mod anthropic;
pub mod llama_cpp;
mod raw;

#[derive(Debug)]
pub struct ExtractedReceipt {
    pub header: ReceiptHeader,
    pub articles: Vec<ExtractedArticle>,
}

/// An [ExtractedArticle] has its cross-reference to the underlying item not yet resolved.
#[derive(Debug)]
pub struct ExtractedArticle {
    pub description: String,
    pub unit_price: Money,
    pub quantity: Quantity,
    pub total: Money,
}

pub trait ReceiptExtractor {
    fn extract_image(
        &mut self,
        image: &DynamicImage,
    ) -> impl Future<Output = InventoryResult<ExtractedReceipt>> + Send;
}
