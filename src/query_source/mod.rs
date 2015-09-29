mod joins;
mod select;

use types::{FromSqlResult, NativeSqlType, Many};
use std::convert::Into;
pub use self::joins::{InnerJoinSource, LeftOuterJoinSource};
use self::select::SelectSqlQuerySource;

pub use self::joins::JoinTo;

pub trait Queriable<ST: NativeSqlType> {
    type Row: FromSqlResult<ST>;

    fn build(row: Self::Row) -> Self;
}

// impl<ST, T> Queriable<Many<ST>> for Vec<T> {
//     type 

pub trait QuerySource: Sized {
    type SqlType: NativeSqlType;

    fn select_clause(&self) -> String;
    fn from_clause(&self) -> String;

    fn select<C, T>(self, column: C) -> SelectSqlQuerySource<C::SqlType, Self> where
        C: SelectableColumn<T, Self>,
    {
        self.select_sql_inner(column.qualified_name())
    }

    fn select_sql<A: NativeSqlType>(self, columns: &str)
        -> SelectSqlQuerySource<A, Self>
    {
        self.select_sql_inner(columns)
    }

    fn select_sql_inner<A, S>(self, columns: S)
        -> SelectSqlQuerySource<A, Self> where
        A: NativeSqlType,
        S: Into<String>
    {
        SelectSqlQuerySource::new(columns.into(), self)
    }
}

pub trait Column<Table> {
    type SqlType: NativeSqlType;

    fn name(&self) -> String;

    fn qualified_name(&self) -> String;
}

pub trait Table: QuerySource {
    type PrimaryKey: Column<Self>;
    fn name(&self) -> &str;
    fn primary_key(&self) -> Self::PrimaryKey;

    fn inner_join<T>(self, other: T) -> InnerJoinSource<Self, T> where
        T: Table,
        Self: JoinTo<T>,
    {
        InnerJoinSource::new(self, other)
    }

    fn left_outer_join<T>(self, other: T) -> LeftOuterJoinSource<Self, T> where
        T: Table,
        Self: JoinTo<T>,
    {
        LeftOuterJoinSource::new(self, other)
    }
}

pub trait SelectableColumn<T, QS: QuerySource>: Column<T> {}

impl<T, C> SelectableColumn<T, T> for C where
    T: Table,
    C: Column<T>,
{}
