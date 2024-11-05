use self::models::Post;
use diesel::prelude::*;
use probardiesel::*;

fn main() {
    use self::schema::posts::dsl::{posts, published};

    let id = 1;
    // args()
    //     .nth(1)
    //     .expect("publish_post requires a post id")
    //     .parse::<i32>()
    //     .expect("Invalid ID");
    let connection = &mut establish_connection();

        let post = diesel::update(posts.find(id))
            .set(published.eq(false))
            .returning(Post::as_returning())
            .get_result(connection)
            .unwrap();
    println!("Published post {}", post.title);
}