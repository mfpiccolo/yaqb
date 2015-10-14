use expression::count::CountStar;
use expression::*;
use query_source::{Table, QuerySource};
use types::{Bool, NativeSqlType};

pub trait QueryBuilder {
    fn push_sql(&mut self, sql: &str);
    fn push_identifier(&mut self, identifier: &str);
    fn output(self) -> (String, Vec<Option<Vec<u8>>>);
}

pub trait Query {
    type SqlType: NativeSqlType;

    fn to_sql<T: QueryBuilder>(&self, out: &mut T);
}

pub trait AsQuery {
    type SqlType: NativeSqlType;
    type Query: Query<SqlType=Self::SqlType>;

    fn as_query(self) -> Self::Query;
}

impl<T: Query> AsQuery for T {
    type SqlType = <Self as Query>::SqlType;
    type Query = Self;

    fn as_query(self) -> Self::Query {
        self
    }
}

use std::marker::PhantomData;

#[derive(Debug, Clone, Copy)]
pub struct SelectStatement<SqlType, Select, From, Where = Bound<Bool, bool>> {
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

impl<ST, S, F> SelectStatement<ST, S, F> {
    pub fn simple(select: S, from: F) -> Self {
        SelectStatement::new(select, from, Bound::new(true))
    }
}

impl<Type, Select, From, Where> Query for SelectStatement<Type, Select, From, Where> where
    Type: NativeSqlType,
    From: QuerySource,
    Select: SelectableExpression<From, Type>,
    Where: SelectableExpression<From, Bool>,
{
    type SqlType = Type;

    fn to_sql<T: QueryBuilder>(&self, out: &mut T) {
        out.push_sql("SELECT ");
        self.select.to_sql(out);
        out.push_sql(" FROM ");
        self.from.from_clause(out);
        out.push_sql(" WHERE ");
        self.where_clause.to_sql(out);
    }
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
    T: Table,
    T::Query: FilterDsl<Pred>,
{
    type Output = <T::Query as FilterDsl<Pred>>::Output;

    fn filter(self, predicate: Pred) -> Self::Output {
        self.as_query().filter(predicate)
    }
}

pub trait SelectDsl<
    Selection: Expression,
    Type: NativeSqlType = <Selection as Expression>::SqlType,
> {
    type Output: Query<SqlType=Type>;

    fn select(self, selection: Selection) -> Self::Output;
}

pub trait CountDsl: SelectDsl<CountStar> + Sized {
    fn count(self) -> <Self as SelectDsl<CountStar>>::Output {
        self.select(count_star())
    }
}

impl<T> CountDsl for T where T: SelectDsl<CountStar> {
}

pub trait SelectSqlDsl: Sized {
    fn select_sql<A>(self, columns: &str)
        -> <Self as SelectDsl<SqlLiteral<A>>>::Output where
        A: NativeSqlType,
        Self: SelectDsl<SqlLiteral<A>>,
    {
        self.select_sql_inner(columns)
    }

    fn select_sql_inner<A, S>(self, columns: S)
        -> <Self as SelectDsl<SqlLiteral<A>>>::Output where
        A: NativeSqlType,
        S: Into<String>,
        Self: SelectDsl<SqlLiteral<A>>,
    {
        self.select(SqlLiteral::new(columns.into()))
    }
}

impl<T> SelectSqlDsl for T where T: SelectDsl<CountStar> {
}

impl<ST, S, F, W, Selection, Type> SelectDsl<Selection, Type>
    for SelectStatement<ST, S, F, W> where
    Type: NativeSqlType,
    Selection: Expression,
    SelectStatement<Type, Selection, F, W>: Query<SqlType=Type>,
{
    type Output = SelectStatement<Type, Selection, F, W>;

    fn select(self, selection: Selection) -> Self::Output {
        SelectStatement::new(selection, self.from, self.where_clause)
    }
}

// FIXME: This can be made generic on AsQuery after Specialization lands
impl<Selection, T> SelectDsl<Selection> for T where
    Selection: Expression,
    T: Table,
    T::Query: SelectDsl<Selection>,
{
    type Output = <T::Query as SelectDsl<Selection>>::Output;

    fn select(self, selection: Selection) -> Self::Output {
        self.as_query().select(selection)
    }
}
