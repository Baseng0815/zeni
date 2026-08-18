use dioxus::prelude::*;
use inventory::receipt::ReceiptId;

#[get("/api/inventory/receipts")]
async fn get_receipts() -> ServerFnResult<Vec<ReceiptId>> {
    Ok(Vec::default())
}
