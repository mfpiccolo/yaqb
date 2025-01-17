use yaqb::*;

#[derive(PartialEq, Eq, Debug, Clone)]
pub struct User {
    pub id: i32,
    pub name: String,
    pub hair_color: Option<String>,
}

impl User {
    pub fn new(id: i32, name: &str) -> Self {
        User { id: id, name: name.to_string(), hair_color: None }
    }

    pub fn with_hair_color(id: i32, name: &str, hair_color: &str) -> Self {
        User {
            id: id,
            name: name.to_string(),
            hair_color: Some(hair_color.to_string()),
        }
    }

    pub fn new_post(&self, title: &str, body: Option<&str>) -> NewPost {
        NewPost::new(self.id, title, body)
    }
}

#[derive(PartialEq, Eq, Debug)]
pub struct Post {
    pub id: i32,
    pub user_id: i32,
    pub title: String,
    pub body: Option<String>,
}

impl Post {
    pub fn new(id: i32, user_id: i32, title: &str, body: Option<&str>) -> Self {
        Post {
            id: id,
            user_id: user_id,
            title: title.to_string(),
            body: body.map(|s| s.to_string()),
        }
    }
}

// Compiler plugin will automatically invoke this based on schema
table! {
    users {
        id -> Serial,
        name -> VarChar,
        hair_color -> Nullable<VarChar>,
    }
}
numeric_expr!(users::id);

table! {
    posts {
        id -> Serial,
        user_id -> Integer,
        title -> VarChar,
        body -> Nullable<Text>,
    }
}

// Compiler plugin will replace this with #[derive(Queriable)]
queriable! {
    User {
        id -> i32,
        name -> String,
        hair_color -> Option<String>,
    }
}

queriable! {
    Post {
        id -> i32,
        user_id -> i32,
        title -> String,
        body -> Option<String>,
    }
}

select_column_workaround!(users -> posts (id, name, hair_color));
select_column_workaround!(posts -> users (id, user_id, title, body));

one_to_many!(users (User) -> posts (Post) on (user_id = id));

#[derive(Debug, PartialEq, Eq)]
pub struct NewUser {
    pub name: String,
    pub hair_color: Option<String>,
}

impl NewUser {
    pub fn new(name: &str, hair_color: Option<&str>) -> Self {
        NewUser {
            name: name.to_string(),
            hair_color: hair_color.map(|s| s.to_string()),
        }
    }
}

insertable! {
    NewUser => users {
        name -> String,
        hair_color -> Option<String>,
    }
}

changeset! {
    NewUser => users {
        name -> String,
        hair_color -> Option<String>,
    }
}

queriable! {
    NewUser {
        name -> String,
        hair_color -> Option<String>,
    }
}

pub struct NewPost {
    user_id: i32,
    title: String,
    body: Option<String>,
}

impl NewPost {
    pub fn new(user_id: i32, title: &str, body: Option<&str>) -> Self {
        NewPost {
            user_id: user_id,
            title: title.into(),
            body: body.map(|b| b.into()),
        }
    }
}

insertable! {
    NewPost => posts {
        user_id -> i32,
        title -> String,
        body -> Option<String>,
    }
}

pub fn setup_users_table(connection: &Connection) {
    connection.execute("CREATE TABLE users (
        id SERIAL PRIMARY KEY,
        name VARCHAR NOT NULL,
        hair_color VARCHAR
    )").unwrap();
}

pub fn setup_posts_table(connection: &Connection) {
    connection.execute("CREATE TABLE posts (
        id SERIAL PRIMARY KEY,
        user_id INTEGER NOT NULL,
        title VARCHAR NOT NULL,
        body TEXT
    )").unwrap();
}

pub fn connection() -> Connection {
    let result = connection_without_transaction();
    result.begin_test_transaction().unwrap();
    result
}

pub fn connection_without_transaction() -> Connection {
    let connection_url = ::std::env::var("DATABASE_URL").ok()
        .expect("DATABASE_URL must be set in order to run tests");
    Connection::establish(&connection_url).unwrap()
}

pub fn connection_with_sean_and_tess_in_users_table() -> Connection {
    let connection = connection();
    setup_users_table(&connection);
    let data: &[_] = &[NewUser::new("Sean", None), NewUser::new("Tess", None)];
    connection.insert_returning_count(&users::table, data).unwrap();
    connection
}
