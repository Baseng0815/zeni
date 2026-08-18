use jiff::Timestamp;
use uuid::Uuid;

use crate::errors::InventoryResult;
use crate::item::{
    Item,
    ItemId,
    ItemType,
};
use crate::receipt::ReceiptId;
use crate::receipt::extractors::ExtractedReceipt;
use crate::warehouse::store::WarehouseStore;

pub mod store;

pub struct Warehouse<S> {
    store: S,
}

impl<S: WarehouseStore> Warehouse<S> {
    pub fn new(store: S) -> Self {
        Self { store }
    }

    pub async fn create_item(
        &mut self,
        description: String,
        r#type: ItemType,
    ) -> InventoryResult<Item> {
        let created_at = Timestamp::now();
        let id = ItemId::from(Uuid::new_v7(uuid_timestamp(created_at)));

        let item = Item {
            id,
            created_at,
            description,
            r#type,
        };

        self.store.insert_item(item.clone()).await?;
        Ok(item)
    }

    pub async fn create_receipt(
        &mut self,
        extracted_receipt: ExtractedReceipt,
    ) -> InventoryResult {
        let created_at = Timestamp::now();
        let id = ReceiptId::from(Uuid::new_v7(uuid_timestamp(created_at)));

        todo!()
    }
}

pub(crate) fn uuid_timestamp(timestamp: Timestamp) -> uuid::Timestamp {
    let seconds = u64::try_from(timestamp.as_second()).unwrap_or_default();
    let nanos = u32::try_from(timestamp.subsec_nanosecond()).unwrap_or_default();

    uuid::Timestamp::from_unix(uuid::NoContext, seconds, nanos)
}
