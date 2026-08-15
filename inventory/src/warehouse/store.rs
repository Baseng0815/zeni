use crate::{item::{Item, ItemId}, receipt::Receipt};

#[derive(Debug, thiserror::Error)]
pub enum WarehouseStoreError<E = ()> {
    #[error("domain error: {}")]
    Domain(E),
}

pub type WarehouseStoreResult<T = (), E = ()> = Result<T, WarehouseStoreError<E>>;

pub trait WarehouseStore {
    type DomainError;

    async fn insert_item(
        &mut self,
        item: Item,
    ) -> WarehouseStoreResult<(), Self::DomainError>;

    async fn get_item(
        &mut self,
        id: ItemId,
    ) -> WarehouseStoreResult<(), Self::DomainError>;
}

pub struct InMemoryWarehouseStore {
    items: Vec<Item>,
    receipts: Vec<Receipt>,
}
