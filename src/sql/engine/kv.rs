use crate::error::Result;
use crate::sql::engine::Transaction;
use crate::sql::schema::Table;
use crate::sql::types::Row;
use crate::storage;
use super::Engine;

pub struct KvEngine {
    pub kv: storage::Mvcc,
}

impl Clone for KvEngine {
    fn clone(&self) -> Self {
        Self { kv: self.kv.clone() }
    }
}

impl Engine for KvEngine {
    type Transaction = KvTransaction;

    fn begin(&self) -> Result<Self::Transaction> {
        Ok(Self::Transaction::new(self.kv.begin()?))
    }
}

pub struct KvTransaction {
    txn: storage::MvccTransaction,
}

impl KvTransaction {
    pub fn new(txn: storage::MvccTransaction) -> Self {
        Self { txn }
    }
}

impl Transaction for KvTransaction {
    fn commit(&self) -> Result<()> {
        todo!()
    }

    fn rollback(&self) -> Result<()> {
        todo!()
    }

    fn create_row(&mut self, table: String, row: Row) -> Result<()> {
        todo!()
    }

    fn scan_table(&mut self, table: String) -> Result<Vec<Row>> {

        todo!()
    }

    fn create_table(&mut self, table: String) -> Result<()> {
        todo!()
    }

    fn get_table(&mut self, table: String) -> Result<Option<Table>> {
        todo!()
    }
}