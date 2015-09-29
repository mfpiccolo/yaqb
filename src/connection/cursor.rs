use Queriable;
use db_result::DbResult;
use row::DbRows;
use types::{NativeSqlType, FromSqlResult};

use std::marker::PhantomData;

pub struct Cursor<ST, T> {
    rows: DbRows,
    _marker: PhantomData<(ST, T)>,
}

impl<ST, T> Cursor<ST, T> {
    pub fn new(db_result: DbResult) -> Self {
        Cursor {
            rows: DbRows::new(db_result),
            _marker: PhantomData,
        }
    }
}

impl<ST, T> Iterator for Cursor<ST, T> where
    ST: NativeSqlType,
    T: Queriable<ST>,
{
    type Item = T;

    fn next(&mut self) -> Option<T> {
        match T::Row::build_from_rows(&mut self.rows) {
            Some(Ok(value)) => Some(T::build(value)),
            Some(Err(reason)) => panic!("Error reading values {}", reason.description()),
            None => None,
        }
    }
}
