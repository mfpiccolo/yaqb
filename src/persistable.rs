use std::marker::PhantomData;

use expression::Expression;
use query_builder::{QueryBuilder, BuildQueryResult};
use query_source::{Table, Column};
use types::NativeSqlType;

pub trait Insertable<T: Table> {
    type Columns: InsertableColumns<T>;
    type Values: Expression<SqlType=<Self::Columns as InsertableColumns<T>>::SqlType>;

    fn columns() -> Self::Columns;

    fn values(self) -> Self::Values;
}

pub trait InsertableColumns<T: Table> {
    type SqlType: NativeSqlType;

    fn names(&self) -> String;
}

impl<'a, T, U> Insertable<T> for &'a [U] where
    T: Table,
    &'a U: Insertable<T>,
{
    type Columns = <&'a U as Insertable<T>>::Columns;
    type Values = InsertValues<'a, T, U>;

    fn columns() -> Self::Columns {
        <&'a U>::columns()
    }

    fn values(self) -> Self::Values {
        InsertValues {
            values: self,
            _marker: PhantomData,
        }
    }
}

impl<'a, T, U> Insertable<T> for &'a Vec<U> where
    T: Table,
    &'a U: Insertable<T>,
{
    type Columns = <&'a U as Insertable<T>>::Columns;
    type Values = InsertValues<'a, T, U>;

    fn columns() -> Self::Columns {
        <&'a U>::columns()
    }

    fn values(self) -> Self::Values {
        InsertValues {
            values: &*self,
            _marker: PhantomData,
        }
    }
}


pub struct InsertValues<'a, T, U: 'a> {
    values: &'a [U],
    _marker: PhantomData<T>,
}

impl<'a, T, U> Expression for InsertValues<'a, T, U> where
    T: Table,
    &'a U: Insertable<T>,
{
    type SqlType = <<&'a U as Insertable<T>>::Columns as InsertableColumns<T>>::SqlType;

    fn to_sql<B: QueryBuilder>(&self, out: &mut B) -> BuildQueryResult {
        self.to_insert_sql(out)
    }

    fn to_insert_sql<B: QueryBuilder>(&self, out: &mut B) -> BuildQueryResult {
        for (i, record) in self.values.into_iter().enumerate() {
            if i != 0 {
                out.push_sql(", ");
            }
            try!(record.values().to_insert_sql(out));
        }
        Ok(())
    }
}

impl<C: Column<Table=T>, T: Table> InsertableColumns<T> for C {
    type SqlType = <Self as Expression>::SqlType;

    fn names(&self) -> String {
        Self::name().to_string()
    }
}
