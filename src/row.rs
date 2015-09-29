use db_result::DbResult;

pub trait Row {
    fn take(&mut self) -> &[u8];
    fn next_is_null(&self) -> bool;
}

pub struct DbRow<'a> {
    db_result: &'a DbResult,
    row_idx: usize,
    col_idx: usize,
}

impl<'a> DbRow<'a> {
    pub fn new(db_result: &'a DbResult, row_idx: usize) -> Self {
        DbRow {
            db_result: db_result,
            row_idx: row_idx,
            col_idx: 0,
        }
    }
}

impl<'a> Row for DbRow<'a> {
    fn take(&mut self) -> &[u8] {
        let current_idx = self.col_idx;
        self.col_idx += 1;
        self.db_result.get(self.row_idx, current_idx)
    }

    fn next_is_null(&self) -> bool {
        self.db_result.is_null(self.row_idx, self.col_idx)
    }
}

pub trait Rows<'a> {
    type Item: Row;

    fn next(&'a mut self) -> Option<Self::Item>;
}

pub struct DbRows {
    db_result: DbResult,
    row_idx: usize,
}

impl DbRows {
    pub fn new(db_result: DbResult) -> Self {
        DbRows {
            db_result: db_result,
            row_idx: 0,
        }
    }
}

impl<'a> Rows<'a> for DbRows {
    type Item = DbRow<'a>;

    fn next(&'a mut self) -> Option<Self::Item> {
        if self.row_idx >= self.db_result.num_rows() {
            None
        } else {
            let row = self.db_result.get_row(self.row_idx);
            self.row_idx += 1;
            Some(row)
        }
    }
}
