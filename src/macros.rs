#[macro_export]
macro_rules! table {
    (
        $name:ident {
            $($column_name:ident -> $Type:ty,)+
        }
    ) => {
        table! {
            $name (id) {
                $($column_name -> $Type,)+
            }
        }
    };
    (
        $name:ident ($pk:ident) {
            $($column_name:ident -> $Type:ty,)+
        }
    ) => {
        pub mod $name {
            use $crate::*;
            use $crate::query_builder::*;
            use $crate::types::*;
            pub use self::columns::*;

            pub mod dsl {
                pub use super::columns::{$($column_name),+};
                pub use super::table as $name;
            }

            #[allow(non_upper_case_globals, dead_code)]
            pub const all_columns: ($($column_name),+) = ($($column_name),+);

            #[allow(non_camel_case_types)]
            #[derive(Clone, Copy)]
            pub struct table;

            impl table {
                #[allow(dead_code)]
                pub fn star(&self) -> star {
                    star
                }
            }

            pub type SqlType = ($($Type),+);

            impl QuerySource for table {
                fn from_clause<T: QueryBuilder>(&self, out: &mut T) -> BuildQueryResult {
                    out.push_identifier(stringify!($name))
                }
            }

            impl AsQuery for table {
                type SqlType = SqlType;
                type Query = SelectStatement<SqlType, ($($column_name),+), Self>;

                fn as_query(self) -> Self::Query {
                    SelectStatement::simple(all_columns, self)
                }
            }

            impl Table for table {
                type PrimaryKey = columns::$pk;
                type AllColumns = ($($column_name),+);

                fn name() -> &'static str {
                    stringify!($name)
                }

                fn primary_key(&self) -> Self::PrimaryKey {
                    columns::$pk
                }

                fn all_columns() -> Self::AllColumns {
                    ($($column_name),+)
                }
            }

            pub mod columns {
                use super::table;
                use $crate::{Table, Column, Expression, SelectableExpression};
                use $crate::expression::NonAggregate;
                use $crate::query_builder::{QueryBuilder, BuildQueryResult};
                use $crate::types::*;

                #[allow(non_camel_case_types, dead_code)]
                #[derive(Debug, Clone, Copy)]
                pub struct star;

                impl Expression for star {
                    type SqlType = ();

                    fn to_sql<T: QueryBuilder>(&self, out: &mut T) -> BuildQueryResult {
                        try!(out.push_identifier(table::name()));
                        out.push_sql(".*");
                        Ok(())
                    }
                }

                impl SelectableExpression<table> for star {}

                $(#[allow(non_camel_case_types, dead_code)]
                #[derive(Debug, Clone, Copy)]
                pub struct $column_name;

                impl Expression for $column_name {
                    type SqlType = $Type;

                    fn to_sql<T: QueryBuilder>(&self, out: &mut T) -> BuildQueryResult {
                        try!(out.push_identifier(table::name()));
                        out.push_sql(".");
                        out.push_identifier(stringify!($column_name))
                    }
                }

                impl SelectableExpression<table> for $column_name {}

                impl NonAggregate for $column_name {}

                impl Column for $column_name {
                    type Table = table;

                    fn name() -> &'static str {
                        stringify!($column_name)
                    }
                }
                )+
            }
        }
    }
}

#[macro_export]
macro_rules! queriable {
    (
        $Struct:ident {
            $($field_name:ident -> $Type:ty,)+
        }
    ) => {
        impl<ST> $crate::Queriable<ST> for $Struct where
            ST: $crate::types::NativeSqlType,
            ($($Type),+): $crate::types::FromSqlRow<ST>,
        {
            type Row = ($($Type),+);

            fn build(row: Self::Row) -> Self {
                let ($($field_name),+) = row;
                $Struct {
                    $($field_name: $field_name),+
                }
            }
        }
    }
}

#[macro_export]
macro_rules! insertable {
    (
        $Struct:ty => $table_mod:ident {
            $($field_name:ident -> $Type:ty,)+
        }
    ) => {
        insertable! {
            $Struct => $table_mod {
                $($table_mod, $field_name -> $Type,)+
            }
        }
    };
    (
        $Struct:ty => $table_mod:ident {
            $($field_table_name:ident, $field_name:ident -> $Type:ty,)+
        }
    ) => {
        impl<'a: 'insert, 'insert> $crate::persistable::Insertable<$table_mod::table>
            for &'insert $Struct
        {
            type Columns = ($($table_mod::$field_name),+);
            type Values = $crate::expression::grouped::Grouped<($(
                $crate::expression::helper_types::AsExpr<&'insert $Type, $table_mod::$field_name>
            ),+)>;

            fn columns() -> Self::Columns {
                ($($table_mod::$field_name),+)
            }

            fn values(self) -> Self::Values {
                use $crate::expression::AsExpression;
                use $crate::expression::grouped::Grouped;
                Grouped(($(AsExpression::<
                   <$table_mod::$field_name as $crate::expression::Expression>::SqlType>
                   ::as_expression(&self.$field_name)
               ),+))
            }
        }
    };
}

#[macro_export]
macro_rules! changeset {
    (
        $Struct:ty => $table_mod:ident {
            $($field_name:ident -> $Type:ty,)+
        }
    ) => {
        impl<'a: 'update, 'update> $crate::query_builder::AsChangeset
            for &'update $Struct
        {
            type Changeset = ($(
                $crate::expression::predicates::Eq<
                    $table_mod::$field_name,
                    $crate::expression::bound::Bound<
                        <$table_mod::$field_name as $crate::expression::Expression>::SqlType,
                        &'update $Type,
                    >,
                >
            ),+);

            fn as_changeset(self) -> Self::Changeset {
                use $crate::expression::Expression;

                ($(
                    $table_mod::$field_name.eq(&self.$field_name)
                ),+)
            }
        }
    };
}

#[macro_export]
macro_rules! joinable {
    ($child:ident -> $parent:ident ($source:ident = $target:ident)) => {
        joinable_inner!($child -> $parent ($source = $target));
        joinable_inner!($parent -> $child ($target = $source));
    }
}

#[macro_export]
macro_rules! joinable_inner {
    ($child:ident -> $parent:ident ($source:ident = $target:ident)) => {
        impl $crate::JoinTo<$parent::table> for $child::table {
            type Predicate = $crate::expression::predicates::Eq<$child::$source, $parent::$target>;

            fn join_expression(&self) -> Self::Predicate {
                use $crate::Expression;
                $child::$source.eq($parent::$target)
            }
        }
    }
}

#[macro_export]
macro_rules! select_column_workaround {
    ($parent:ident -> $child:ident ($($column_name:ident),+)) => {
        $(select_column_inner!($parent -> $child $column_name);)+
        select_column_inner!($parent -> $child star);
    }
}

#[macro_export]
macro_rules! one_to_many {
    (
        $parent_table:ident ($parent_struct:ty) ->
        $child_table:ident ($child_struct:ty) on
        ($foreign_key:ident = $primary_key:ident)
    ) => {
        one_to_many!($child_table: $parent_table ($parent_struct) ->
                     $child_table ($child_struct) on ($foreign_key = $primary_key));
    };
    (
        $association_name:ident -> $association_type:ident :
        $parent_table:ident ($parent_struct:ty) ->
        $child_table:ident ($child_struct:ty) on
        ($foreign_key:ident = $primary_key:ident)
    ) => {
        pub type $association_type = $crate::helper_types::FindBy<
            $child_table::table,
            $child_table::$foreign_key,
            i32,
        >;
        one_to_many!($association_name: $parent_table ($parent_struct) ->
                     $child_table ($child_struct) on ($foreign_key = $primary_key));
    };
    (
        $association_name:ident :
        $parent_table:ident ($parent_struct:ty) ->
        $child_table:ident ($child_struct:ty) on
        ($foreign_key:ident = $primary_key:ident)
    ) => {
        impl $parent_struct {
            pub fn $association_name(&self) -> $crate::helper_types::FindBy<
                $child_table::table,
                $child_table::$foreign_key,
                i32,
            > {
                $child_table::table.filter($child_table::$foreign_key.eq(self.$primary_key))
            }
        }

        joinable!($child_table -> $parent_table ($foreign_key = $primary_key));
    };
}

#[macro_export]
macro_rules! select_column_inner {
    ($parent:ident -> $child:ident $column_name:ident) => {
        impl $crate::expression::SelectableExpression<
            $crate::query_source::InnerJoinSource<$child::table, $parent::table>,
        > for $parent::$column_name
        {
        }

        impl $crate::expression::SelectableExpression<
            $crate::query_source::InnerJoinSource<$parent::table, $child::table>,
        > for $parent::$column_name
        {
        }

        impl $crate::expression::SelectableExpression<
            $crate::query_source::LeftOuterJoinSource<$child::table, $parent::table>,
            $crate::types::Nullable<
                <$parent::$column_name as $crate::Expression>::SqlType>,
        > for $parent::$column_name
        {
        }

        impl $crate::expression::SelectableExpression<
            $crate::query_source::LeftOuterJoinSource<$parent::table, $child::table>,
        > for $parent::$column_name
        {
        }
    }
}
