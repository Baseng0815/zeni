use std::path::PathBuf;

use image::ImageReader;
use zeni_inventory::receipt::extractors::ReceiptExtractor;
use zeni_inventory::receipt::extractors::llama_cpp::LlamaCppExtractor;
use zeni_inventory::warehouse::Warehouse;
use zeni_inventory::warehouse::store::InMemoryWarehouseStore;

#[tokio::main]
async fn main() {
    let image_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("examples")
        .join("resources")
        .join("casa-del-mondo.webp");
    let image = ImageReader::open(image_path).unwrap().decode().unwrap();

    let Result::Ok(endpoint) = std::env::var("LLAMA_CPP_ENDPOINT") else {
        return;
    };

    let mut extractor = LlamaCppExtractor::new(endpoint);
    let extracted_receipt = extractor.extract_image(&image).await.unwrap();
    eprintln!("extracted_receipt = {:#?}", extracted_receipt);

    let warehouse_store = InMemoryWarehouseStore::default();
    let mut warehouse = Warehouse::new(warehouse_store);
    warehouse.create_receipt(extracted_receipt).await;
}
