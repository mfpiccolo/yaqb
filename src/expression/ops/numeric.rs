use expression::{Expression, SelectableExpression, NonAggregate};
use query_builder::{QueryBuilder, BuildQueryResult};
use types;

macro_rules! numeric_operation {
    ($name:ident, $op:expr) => {
        pub struct $name<Lhs, Rhs> {
            lhs: Lhs,
            rhs: Rhs,
        }

        impl<Lhs, Rhs> $name<Lhs, Rhs> {
            pub fn new(left: Lhs, right: Rhs) -> Self {
                $name {
                    lhs: left,
                    rhs: right,
                }
            }
        }

        impl<Lhs, Rhs> Expression for $name<Lhs, Rhs> where
            Lhs: Expression,
            Lhs::SqlType: types::ops::$name,
            Rhs: Expression,
        {
            type SqlType = <Lhs::SqlType as types::ops::$name>::Output;

            fn to_sql<T: QueryBuilder>(&self, out: &mut T) -> BuildQueryResult {
                try!(self.lhs.to_sql(out));
                out.push_sql($op);
                self.rhs.to_sql(out)
            }
        }

        impl<Lhs, Rhs, QS> SelectableExpression<QS> for $name<Lhs, Rhs> where
            Lhs: SelectableExpression<QS>,
            Rhs: SelectableExpression<QS>,
            $name<Lhs, Rhs>: Expression,
        {
        }

        impl<Lhs, Rhs> NonAggregate for $name<Lhs, Rhs> where
            Lhs: NonAggregate,
            Rhs: NonAggregate,
            $name<Lhs, Rhs>: Expression,
        {
        }

        generic_numeric_expr!($name, A, B);
    }
}

numeric_operation!(Add, " + ");
numeric_operation!(Sub, " - ");
numeric_operation!(Mul, " * ");
numeric_operation!(Div, " / ");
