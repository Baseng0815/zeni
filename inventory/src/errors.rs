#[derive(Debug, thiserror::Error)]
pub enum InventoryError {
    #[error("could not encode the receipt image")]
    ImageEncode(#[from] image::ImageError),
    #[error("could not write the receipt image into the media path")]
    MediaWrite(#[from] std::io::Error),
    #[error("the extractor request failed")]
    Request(#[from] reqwest::Error),
    #[error("the extractor returned no choices")]
    EmptyResponse,
    #[error("the extractor returned a receipt that does not match the schema")]
    MalformedReceipt(#[from] serde_json::Error),
    #[error("unsupported currency code `{0}`")]
    UnsupportedCurrency(String),
    #[error("could not read `{0}` as an amount")]
    MalformedAmount(String),
    #[error("could not read `{value}` as a {kind}")]
    MalformedCivil { kind: &'static str, value: String },
    #[error("quantity is of kind `{0}` but carries no value")]
    MissingQuantity(&'static str),
}

pub type InventoryResult<T = ()> = Result<T, InventoryError>;
