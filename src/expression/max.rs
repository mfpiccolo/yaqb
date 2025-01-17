use query_builder::{QueryBuilder, BuildQueryResult};
use super::{Expression, SelectableExpression};
use types::{SqlOrd, NativeSqlType};

pub fn max<ST, T>(t: T) -> Max<T> where
    ST: NativeSqlType + SqlOrd,
    T: Expression<SqlType=ST>,
{
    Max {
        target: t,
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Max<T: Expression> {
    target: T,
}

impl<T: Expression> Expression for Max<T> {
    type SqlType = T::SqlType;

    fn to_sql<B: QueryBuilder>(&self, out: &mut B) -> BuildQueryResult {
        out.push_sql("MAX(");
        try!(self.target.to_sql(out));
        out.push_sql(")");
        Ok(())
    }
}

impl<T: Expression, QS> SelectableExpression<QS> for Max<T> {
}
