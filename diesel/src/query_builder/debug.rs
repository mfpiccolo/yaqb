use connection::*;
// use connection::pq_sys::*;
use super::{QueryBuilder, Binds, BuildQueryResult};
use types::NativeSqlType;

#[allow(dead_code)]
pub struct DebugQueryBuilder {
    pub sql: String,
    pub binds: Binds,
    pub bind_types: Vec<u32>,
    bind_idx: u32,
}

#[allow(dead_code)]
impl DebugQueryBuilder {
    pub fn new() -> Self {
        DebugQueryBuilder {
            sql: String::new(),
            binds: Vec::new(),
            bind_types: Vec::new(),
            bind_idx: 0,
        }
    }
}

#[allow(dead_code)]
impl QueryBuilder for DebugQueryBuilder {
    fn push_sql(&mut self, sql: &str) {
        self.sql.push_str(sql);
    }

    fn push_identifier(&mut self, identifier: &str) -> BuildQueryResult {
        let conn = Connection::establish("postgresql://localhost/yaqb_test").unwrap();
        let escaped_identifier = try!(conn.escape_identifier(identifier));
        Ok(self.push_sql(&escaped_identifier))
    }

    fn push_bound_value(&mut self, tpe: &NativeSqlType, bind: Option<Vec<u8>>) {
        self.bind_idx += 1;
        let sql = format!("${}", self.bind_idx);
        self.push_sql(&sql);
        self.binds.push(bind);
        self.bind_types.push(tpe.oid());
    }
}
