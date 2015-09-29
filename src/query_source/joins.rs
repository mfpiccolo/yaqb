use {QuerySource, Table};
use types::Many;

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

impl<Left, Right> QuerySource for InnerJoinSource<Left, Right> where
    Left: Table + JoinTo<Right>,
    Right: Table,
{
    type SqlType = (Left::SqlType, Right::SqlType);

    fn select_clause(&self) -> String {
        format!("{}, {}", self.left.select_clause(), self.right.select_clause())
    }

    fn from_clause(&self) -> String {
        format!("{} INNER JOIN {} ON {}",
            self.left.name(), self.right.name(), self.left.join_sql())
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

impl<Left, Right> QuerySource for LeftOuterJoinSource<Left, Right> where
    Left: Table + JoinTo<Right>,
    Right: Table,
{
    type SqlType = (Left::SqlType, Many<Right::SqlType>);

    fn select_clause(&self) -> String {
        format!("{}, {}", self.left.select_clause(), self.right.select_clause())
    }

    fn from_clause(&self) -> String {
        format!("{} LEFT OUTER JOIN {} ON {}",
            self.left.name(), self.right.name(), self.left.join_sql())
    }
}

pub trait JoinTo<T: Table>: Table {
    fn join_sql(&self) -> String;
}
