use schema::*;
use diesel::*;

#[test]
fn test_debug_count_output() {
    use schema::users::dsl::*;
    let mut query_builder = ::diesel::query_builder::debug::DebugQueryBuilder::new();
    let command = users.count();
    command.to_sql(&mut query_builder).unwrap();
    assert!(query_builder.sql.starts_with("SELECT COUNT(*) FROM"));
}

// // TODO: no method named `set` found for type `diesel::query_source::filter::FilteredQuerySource<schema::users::table, diesel::expression::predicates::Eq<schema::users::columns::id, diesel::expression::bound::Bound<diesel::types::Integer, i32>>>` in the current scope
// #[test]
// fn test_debug_output() {
//     use schema::users::dsl::*;
//     let mut query_builder = ::diesel::query_builder::debug::DebugQueryBuilder::new();
//     let command = users.filter(id.eq(1)).set(name.eq("Jim"));
//     command.to_sql(&mut query_builder).unwrap();
//     assert!(query_builder.sql.starts_with("SELECT COUNT(*) FROM"));
// }
