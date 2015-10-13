mod joins;

use expression::{Expression, SelectableExpression, NonAggregate};
use query_builder::AsQuery;
pub use self::joins::{InnerJoinSource, LeftOuterJoinSource};
use types::{FromSqlRow, NativeSqlType};

pub use self::joins::JoinTo;

pub trait Queriable<ST: NativeSqlType> {
    type Row: FromSqlRow<ST>;

    fn build(row: Self::Row) -> Self;
}

pub trait Column {
    type Table: Table;
    type SqlType: NativeSqlType;

    fn name(&self) -> String;

    fn qualified_name(&self) -> String;
}

impl<C: Column> Expression for C {
    type SqlType = <Self as Column>::SqlType;

    fn to_sql(&self) -> String {
        self.qualified_name()
    }
}

impl<C: Column> SelectableExpression<C::Table> for C {
}

impl<C: Column> NonAggregate for C {
}

pub trait Table: AsQuery + Sized {
    type PrimaryKey: Column<Table=Self>;
    type Star: Column<Table=Self>;
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
