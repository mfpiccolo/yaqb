use Queriable;
use db_result::DbResult;
use types::{NativeSqlType, FromSqlRow};

use std::marker::PhantomData;

pub struct Cursor<ST, T, Hack = ()> {
    current_row: usize,
    db_result: DbResult,
    _marker: PhantomData<(ST, T, Hack)>,
}

impl<ST, T, Hack> Cursor<ST, T, Hack> {
    pub fn new(db_result: DbResult) -> Self {
        Cursor {
            current_row: 0,
            db_result: db_result,
            _marker: PhantomData,
        }
    }
}

impl<ST, T, Hack> Iterator for Cursor<ST, T, Hack> where
    ST: NativeSqlType,
    T: Queriable<ST, Hack>,
{
    type Item = T;

    fn next(&mut self) -> Option<T> {
        if self.current_row >= self.db_result.num_rows() {
            None
        } else {
            let mut row = self.db_result.get_row(self.current_row);
            self.current_row += 1;
            let values = match T::Row::build_from_row(&mut row) {
                Ok(value) => value,
                Err(reason) => panic!("Error reading values {}", reason.description()),
            };
            let result = T::build(values);
            Some(result)
        }
    }
}
