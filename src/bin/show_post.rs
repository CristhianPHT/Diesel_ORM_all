// use self::models::*;
use diesel::prelude::*;
use probardiesel::{establish_connection};
use probardiesel::schema::posts::dsl::*;
// use probardiesel::schema::posts::dsl::posts;
use probardiesel::schema::posts::published;
use probardiesel::models::*;

fn main() {

    let connection = &mut establish_connection();
    let results = posts
        .filter(published.eq(true))
        .limit(5)
        .select(Post::as_select())
        .load(connection)
        .expect("Error loading posts");

    println!("Displaying {} posts", results.len());
    for post in results {
        println!("{}", post.title);
        println!("-----------\n");
        println!("{}", post.body);
    }
}