use db_result::DbResult;
use std::rc::Rc;

pub trait Row {
    fn take(&mut self) -> &[u8];
    fn next_is_null(&self) -> bool;
}

pub struct DbRow {
    db_result: Rc<DbResult>,
    row_idx: usize,
    col_idx: usize,
}

impl DbRow {
    pub fn new(db_result: Rc<DbResult>, row_idx: usize) -> Self {
        DbRow {
            db_result: db_result,
            row_idx: row_idx,
            col_idx: 0,
        }
    }
}

impl Row for DbRow {
    fn take(&mut self) -> &[u8] {
        let current_idx = self.col_idx;
        self.col_idx += 1;
        self.db_result.get(self.row_idx, current_idx)
    }

    fn next_is_null(&self) -> bool {
        self.db_result.is_null(self.row_idx, self.col_idx)
    }
}

pub struct DbRows {
    db_result: Rc<DbResult>,
    row_idx: usize,
}

impl DbRows {
    pub fn new(db_result: DbResult) -> Self {
        DbRows {
            db_result: Rc::new(db_result),
            row_idx: 0,
        }
    }
}

impl Iterator for DbRows {
    type Item = DbRow;

    fn next(&mut self) -> Option<Self::Item> {
        if self.row_idx >= self.db_result.num_rows() {
            None
        } else {
            let row = DbRow::new(self.db_result.clone(), self.row_idx);
            self.row_idx += 1;
            Some(row)
        }
    }
}
