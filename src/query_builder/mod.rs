use expression::count::CountStar;
use expression::*;
use types::{Bool, NativeSqlType};

pub trait Query {
}

pub trait AsQuery {
    type Query: Query;

    fn as_query(self) -> Self::Query;
}

pub struct SelectStatement<Select, From, Where> {
    select: Select,
    from: From,
    where_clause: Where,
}

impl<Select, From, Where> Query for SelectStatement<Select, From, Where> {
}

pub trait FilterDsl<Predicate> {
    type Output: Query;

    fn filter(self, predicate: Predicate) -> Self::Output;
}

impl<Select, From, Where, Pred> FilterDsl<Pred>
    for SelectStatement<Select, From, Where> where
    Pred: SelectableExpression<From, Bool>,
{
    type Output = SelectStatement<Select, From, Pred>;

    fn filter(self, predicate: Pred) -> Self::Output {
        SelectStatement {
            select: self.select,
            from: self.from,
            where_clause: predicate,
        }
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

impl<S, F, W, Selection> SelectDsl<Selection> for SelectStatement<S, F, W> where
    Selection: SelectableExpression<F>,
{
    type Output = SelectStatement<Selection, F, W>;

    fn select(self, selection: Selection) -> Self::Output {
        SelectStatement {
            select: selection,
            from: self.from,
            where_clause: self.where_clause,
        }
    }
}
