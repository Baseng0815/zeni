use zeni_finance::ledger::store::LedgerStoreError;

use crate::item::{
    Item,
    ItemId,
};
use crate::receipt::Receipt;

#[derive(Debug, thiserror::Error)]
pub enum WarehouseStoreError {
    #[error("duplicate item id: {0}")]
    DuplicateItemId(ItemId),
    #[error("no such item: {0}")]
    NoSuchItem(ItemId),
    #[error("domain error: {0}")]
    Domain(String),
}

pub type WarehouseStoreResult<T = ()> = Result<T, WarehouseStoreError>;

pub trait WarehouseStore {
    async fn insert_item(
        &mut self,
        item: Item,
    ) -> WarehouseStoreResult;

    async fn get_item(
        &self,
        id: ItemId,
    ) -> WarehouseStoreResult<Item>;
}

#[derive(Default)]
pub struct InMemoryWarehouseStore {
    items: Vec<Item>,
    receipts: Vec<Receipt>,
}

impl WarehouseStore for InMemoryWarehouseStore {
    async fn insert_item(
        &mut self,
        item: Item,
    ) -> WarehouseStoreResult {
        if self.get_item(item.id()).await.is_ok() {
            Err(WarehouseStoreError::DuplicateItemId(item.id()))?;
        }

        self.items.push(item);
        Ok(())
    }

    async fn get_item(
        &self,
        id: ItemId,
    ) -> WarehouseStoreResult<Item> {
        self.items
            .iter()
            .find(|item| item.id() == id)
            .cloned()
            .ok_or_else(|| WarehouseStoreError::NoSuchItem(id))
    }
}
