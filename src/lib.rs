
use self::models::{NewPost, Post};

pub mod models;
pub mod schema;

use diesel::prelude::*;
use dotenvy::dotenv;
use std::env;

pub fn establish_connection() -> PgConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}
pub fn create_post(conn: &mut PgConnection, title: &str, body: &str) -> Post {
    use crate::schema::posts;

    let new_post = NewPost { title, body };

    diesel::insert_into(posts::table)
        .values(&new_post)
        .returning(Post::as_returning())
        .get_result(conn)
        .expect("Error saving new post")
}

// ------------------------------ UPDATE

// use diesel::prelude::*; // Import the necessary Diesel prelude
use crate::schema::posts::dsl::*; // Import your posts table
// use crate::posts::dsl::*;
// use crate::posts::draft;

// Assuming `conn` is your database connection
pub fn actualizar(conn: &mut PgConnection){
    // let result = diesel::update(posts)
    // .set(body.eq("false".to_string()))
    // .execute(conn);
    // let id :i64= 1;
    let post = diesel::update(posts.find(id))
    .set(published.eq(true))
    .returning(Post::as_returning())
    .get_result(conn)
    .unwrap();
    println!("{:?}", &post);
}

