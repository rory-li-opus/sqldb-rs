use crate::error::Result;
use super::{parser::Parser, plan::Plan, schema::Table, types::Row, executor::ResultSet};

mod kv;

pub trait Engine: Clone {
    type Transaction: Transaction;

    fn begin(&self) -> Result<Self::Transaction>;

    fn session(&self) -> Result<Session<Self>> {
        Ok(Session {
            engine: self.clone(),
        })
    }
}

pub trait Transaction {
    fn commit(&self) -> Result<()>;
    fn rollback(&self) -> Result<()>;
    fn create_row(&mut self, table: String, row: Row) -> Result<()>;
    fn scan_table(&mut self, table: String) -> Result<Vec<Row>>;
    fn create_table(&mut self, table: String) -> Result<()>;
    fn get_table(&mut self, table: String) -> Result<Option<Table>>;
}


pub struct Session<E: Engine> {
    engine: E,
}

impl<E: Engine> Session<E> {
    pub fn execute(&mut self, sql: &str) -> Result<ResultSet> {
        match Parser::new(sql).parse()? {
            stmt => {
                let mut txn = self.engine.begin()?;
                match Plan::build(stmt).execute(&mut txn) {
                    Ok(result) => {
                        txn.commit()?;
                        Ok(result)
                    },
                    Err(e) => {
                        txn.rollback()?;
                        Err(e)
                    }
                }
            }
        }
    }
}