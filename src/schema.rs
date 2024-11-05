// @generated automatically by Diesel CLI.

diesel::table! {
  posts (id) {
      id -> Int4,
      title -> Varchar,
      body -> Text,
      published -> Bool,
  }
}
// diesel::table!{
//   posts {
//     id -> BigInt,
//     title -> Text,
//     body -> Text,
//     draft -> Bool,
//     publish_at -> Timestamp,
//     visit_count -> Integer,
// }
// }