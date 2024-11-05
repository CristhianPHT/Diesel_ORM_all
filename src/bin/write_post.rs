use diesel::*;
use std::io::{stdin, Read};

use probardiesel::{establish_connection, create_post}; // probardiesel es el nombre de paquete para archivos.
// cargo new probardiesel

fn main() {
  let connection = &mut establish_connection();

  let mut title = String::new();
  let mut body = String::new();

  println!("What would you like your title to be?");
  stdin().read_line(&mut title).unwrap();
  let title = title.trim_end(); // Remove the trailing newline

  println!("\nOk! Let's write {title} (Press {EOF} when finished)\n",);
  stdin().read_to_string(&mut body).unwrap();

  let post = create_post(connection, title.to_string(), &body);
  println!("\nSaved draft {title} with id {}", post.id);
}

#[cfg(not(windows))]
const EOF: &str = "CTRL+D";

#[cfg(windows)]
const EOF: &str = "CTRL+Z";