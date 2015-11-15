pub mod pg;

mod limit_clause;
mod order_clause;
mod select_statement;
mod where_clause;

pub use self::select_statement::SelectStatement;

use std::error::Error;
use query_source::QuerySource;
use types::NativeSqlType;

pub type Binds = Vec<Option<Vec<u8>>>;
pub type BuildQueryResult = Result<(), Box<Error>>;

pub trait QueryBuilder {
    fn push_sql(&mut self, sql: &str);
    fn push_identifier(&mut self, identifier: &str) -> BuildQueryResult;
    fn push_bound_value<T: NativeSqlType>(&mut self, binds: Option<Vec<u8>>);
}

pub trait Query: QueryFragment {
    type SqlType: NativeSqlType;
}

pub trait UpdateQuery: QueryFragment {
    type Source: QuerySource;
}

pub trait QueryFragment {
    fn to_sql<T: QueryBuilder>(&self, out: &mut T) -> BuildQueryResult;
}

pub trait AsQuery {
    type SqlType: NativeSqlType;
    type Query: Query<SqlType=Self::SqlType>;

    fn as_query(self) -> Self::Query;
}

pub trait AsUpdateQuery<Updates> {
    type UpdateQuery: UpdateQuery;

    fn as_update_query(self, updates: Updates) -> Self::UpdateQuery;
}

impl<T: Query> AsQuery for T {
    type SqlType = <Self as Query>::SqlType;
    type Query = Self;

    fn as_query(self) -> Self::Query {
        self
    }
}
