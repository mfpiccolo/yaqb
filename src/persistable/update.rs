use query_source::QuerySource;
use expression::Expression;

pub trait UpdateSet<Source: QuerySource>: Expression {
}
