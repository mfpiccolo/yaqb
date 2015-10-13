use expression::{Expression, SelectableExpression};
use query_builder::*;
use super::Table;
use types::{NativeSqlType, Nullable, Bool};

#[derive(Clone, Copy)]
pub struct InnerJoinSource<Left, Right> {
    left: Left,
    right: Right,
}

impl<Left, Right> InnerJoinSource<Left, Right> {
    pub fn new(left: Left, right: Right) -> Self {
        InnerJoinSource {
            left: left,
            right: right,
        }
    }
}

impl<Left, Right> AsQuery for InnerJoinSource<Left, Right> where
    Left: Table + JoinTo<Right>,
    Right: Table,
    (Left::Star, Right::Star): SelectableExpression<
                               InnerJoinSource<Left, Right>,
                               (Left::SqlType, Right::SqlType),
                               >,
{
    type SqlType = (Left::SqlType, Right::SqlType);
    type Query = SelectStatement<
        (Left::SqlType, Right::SqlType),
        (Left::Star, Right::Star),
        Self,
    >;

    fn as_query(self) -> Self::Query {
        unimplemented!()
    }
}

// FIXME: This can be made generic on AsQuery after Specialization lands
impl<Selection, Type, Left, Right> SelectDsl<Selection, Type>
    for InnerJoinSource<Left, Right> where
    Type: NativeSqlType,
    Selection: Expression,
    InnerJoinSource<Left, Right>: AsQuery,
    <InnerJoinSource<Left, Right> as AsQuery>::Query: SelectDsl<Selection, Type>,
{
    type Output = <<Self as AsQuery>::Query as SelectDsl<Selection, Type>>::Output;

    fn select(self, selection: Selection) -> Self::Output {
        self.as_query().select(selection)
    }
}

// FIXME: This can be made generic on AsQuery after Specialization lands
impl<Pred, Left, Right> FilterDsl<Pred> for InnerJoinSource<Left, Right> where
    Pred: SelectableExpression<InnerJoinSource<Left, Right>, Bool>,
    InnerJoinSource<Left, Right>: AsQuery,
    <InnerJoinSource<Left, Right> as AsQuery>::Query: FilterDsl<Pred>,
{
    type Output = <<Self as AsQuery>::Query as FilterDsl<Pred>>::Output;

    fn filter(self, predicate: Pred) -> Self::Output {
        self.as_query().filter(predicate)
    }
}

#[derive(Clone, Copy)]
pub struct LeftOuterJoinSource<Left, Right> {
    left: Left,
    right: Right,
}

impl<Left, Right> LeftOuterJoinSource<Left, Right> {
    pub fn new(left: Left, right: Right) -> Self {
        LeftOuterJoinSource {
            left: left,
            right: right,
        }
    }
}

impl<Left, Right> AsQuery for LeftOuterJoinSource<Left, Right> where
    Left: Table + JoinTo<Right>,
    Right: Table,
    (Left::Star, Right::Star): SelectableExpression<
                               LeftOuterJoinSource<Left, Right>,
                               (Left::SqlType, Nullable<Right::SqlType>),
                               >,
{
    type SqlType = (Left::SqlType, Nullable<Right::SqlType>);
    type Query = SelectStatement<
        (Left::SqlType, Nullable<Right::SqlType>),
        (Left::Star, Right::Star),
        Self,
    >;

    fn as_query(self) -> Self::Query {
        unimplemented!()
    }
}

// FIXME: This can be made generic on AsQuery after Specialization lands
impl<Selection, Type, Left, Right> SelectDsl<Selection, Type>
    for LeftOuterJoinSource<Left, Right> where
    Type: NativeSqlType,
    Selection: SelectableExpression<LeftOuterJoinSource<Left, Right>, Type>,
    LeftOuterJoinSource<Left, Right>: AsQuery,
    <LeftOuterJoinSource<Left, Right> as AsQuery>::Query: SelectDsl<Selection, Type>,
{
    type Output = <<Self as AsQuery>::Query as SelectDsl<Selection, Type>>::Output;

    fn select(self, selection: Selection) -> Self::Output {
        self.as_query().select(selection)
    }
}

// FIXME: This can be made generic on AsQuery after Specialization lands
impl<Pred, Left, Right> FilterDsl<Pred> for LeftOuterJoinSource<Left, Right> where
    Pred: SelectableExpression<LeftOuterJoinSource<Left, Right>, Bool>,
    LeftOuterJoinSource<Left, Right>: AsQuery,
    <LeftOuterJoinSource<Left, Right> as AsQuery>::Query: FilterDsl<Pred>,
{
    type Output = <<Self as AsQuery>::Query as FilterDsl<Pred>>::Output;

    fn filter(self, predicate: Pred) -> Self::Output {
        self.as_query().filter(predicate)
    }
}

pub trait JoinTo<T: Table>: Table {
    fn join_sql(&self) -> String;
}
