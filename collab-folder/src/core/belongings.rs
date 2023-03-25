use crate::core::Belongings;
use collab::preclude::{Array, ArrayRef, ArrayRefWrapper, MapRefWrapper, ReadTxn, TransactionMut};

pub struct BelongingMap {
    container: MapRefWrapper,
}

impl BelongingMap {
    pub fn new(container: MapRefWrapper) -> Self {
        Self { container }
    }

    pub fn move_belonging(&self, bid: &str, from: u32, to: u32) {
        if let Some(belonging_array) = self.get_belongings(bid) {
            self.container.with_transact_mut(|txn| {
                belonging_array.move_belonging_with_txn(txn, from, to);
            })
        }
    }

    pub fn get_belongings(&self, bid: &str) -> Option<BelongingsArray> {
        let array = self.container.get_array_ref(bid)?;
        Some(BelongingsArray::from_array(array))
    }

    pub fn insert_belongings(&self, bid: &str, belongings: Belongings) -> BelongingsArray {
        let array_ref = self.container.with_transact_mut(|txn| {
            self.container
                .get_array_ref_with_txn(txn, bid)
                .unwrap_or_else(|| {
                    self.container
                        .insert_array_with_txn(txn, bid, belongings.into_inner())
                })
        });
        BelongingsArray::from_array(array_ref)
    }

    pub fn delete_belongings(&self, bid: &str, index: u32) {
        if let Some(belonging_array) = self.get_belongings(bid) {
            self.container.with_transact_mut(|txn| {
                belonging_array.remove_belonging_with_txn(txn, index);
            })
        }
    }
}

const BELONGINGS: &str = "belongings";
#[derive(Clone)]
pub struct BelongingsArray {
    container: ArrayRefWrapper,
}

impl BelongingsArray {
    pub fn get_or_create_with_txn(txn: &mut TransactionMut, container: &MapRefWrapper) -> Self {
        let belongings_container = container
            .get_array_ref_with_txn(txn, BELONGINGS)
            .unwrap_or_else(|| {
                container.insert_array_with_txn(
                    txn,
                    BELONGINGS,
                    Belongings::new(vec![]).into_inner(),
                )
            });
        Self {
            container: belongings_container,
        }
    }

    pub fn from_array(belongings: ArrayRefWrapper) -> Self {
        Self {
            container: belongings,
        }
    }

    pub fn get_belongings(&self) -> Belongings {
        let txn = self.container.transact();
        self.get_belongings_with_txn(&txn)
    }

    pub fn get_belongings_with_txn<T: ReadTxn>(&self, txn: &T) -> Belongings {
        belongings_from_array_ref(txn, &self.container)
    }

    pub fn move_belonging_with_txn(&self, txn: &mut TransactionMut, from: u32, to: u32) {
        self.container.move_to(txn, from, to)
    }

    pub fn remove_belonging_with_txn(&self, txn: &mut TransactionMut, index: u32) {
        self.container.remove_with_txn(txn, index);
    }

    pub fn add_belonging_with_txn(&self, txn: &mut TransactionMut, view_id: &str) {
        self.container.push_with_txn(txn, view_id)
    }
}

pub fn belongings_from_array_ref<T: ReadTxn>(txn: &T, array_ref: &ArrayRef) -> Belongings {
    let mut belongings = Belongings::new(vec![]);
    for value in array_ref.iter(txn) {
        belongings.view_ids.push(value.to_string(txn));
    }
    belongings
}