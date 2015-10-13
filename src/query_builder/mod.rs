use expression::count::CountStar;
use expression::*;
use types::{Bool, NativeSqlType};

pub trait Query {
    type SqlType;
}

pub trait AsQuery {
    type Query: Query;

    fn as_query(self) -> Self::Query;
}

use std::marker::PhantomData;

pub struct SelectStatement<SqlType, Select, From, Where> {
    select: Select,
    from: From,
    where_clause: Where,
    _marker: PhantomData<SqlType>,
}

impl<ST, S, F, W> SelectStatement<ST, S, F, W> {
    pub fn new(select: S, from: F, where_clause: W) -> Self {
        SelectStatement {
            select: select,
            from: from,
            where_clause: where_clause,
            _marker: PhantomData,
        }
    }
}

impl<Type, Select, From, Where> Query for SelectStatement<Type, Select, From, Where> where
    Type: NativeSqlType,
    Select: SelectableExpression<From, Type>,
    Where: SelectableExpression<From, Bool>,
{
    type SqlType = Type;
}

pub trait FilterDsl<Predicate> {
    type Output: Query;

    fn filter(self, predicate: Predicate) -> Self::Output;
}

impl<ST, Select, From, Where, Pred> FilterDsl<Pred>
    for SelectStatement<ST, Select, From, Where> where
    SelectStatement<ST, Select, From, Pred>: Query,
{
    type Output = SelectStatement<ST, Select, From, Pred>;

    fn filter(self, predicate: Pred) -> Self::Output {
        SelectStatement::new(self.select, self.from, predicate)
    }
}

impl<Pred, T> FilterDsl<Pred> for T where
    T: AsQuery,
    T::Query: FilterDsl<Pred>,
{
    type Output = <T::Query as FilterDsl<Pred>>::Output;

    fn filter(self, predicate: Pred) -> Self::Output {
        self.as_query().filter(predicate)
    }
}


pub trait SelectDsl<Selection> {
    type Output: Query;

    fn select(self, selection: Selection) -> Self::Output;

    fn count(self) -> <Self as SelectDsl<CountStar>>::Output where
        Self: SelectDsl<CountStar> + Sized,
    {
        self.select(count_star())
    }

    fn select_sql<A>(self, columns: &str)
        -> <Self as SelectDsl<SqlLiteral<A>>>::Output where
        A: NativeSqlType,
        Self: SelectDsl<SqlLiteral<A>> + Sized,
    {
        <Self as SelectDsl<SqlLiteral<A>>>::select_sql_inner(self, columns)
    }

    fn select_sql_inner<A, S>(self, columns: S)
        -> <Self as SelectDsl<SqlLiteral<A>>>::Output where
        A: NativeSqlType,
        S: Into<String>,
        Self: SelectDsl<SqlLiteral<A>> + Sized,
    {
        self.select(SqlLiteral::new(columns.into()))
    }
}

impl<ST, S, F, W, Selection> SelectDsl<Selection>
    for SelectStatement<ST, S, F, W> where
    SelectStatement<ST, Selection, F, W>: Query,
{
    type Output = SelectStatement<ST, Selection, F, W>;

    fn select(self, selection: Selection) -> Self::Output {
        SelectStatement::new(selection, self.from, self.where_clause)
    }
}
