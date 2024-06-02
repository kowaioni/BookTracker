#[macro_use] extern crate rocket;

use rocket::serde::{json::Json, Deserialize, Serialize};

#[derive(FromForm, Serialize, Deserialize)]
struct Book {
    id: Option<i32>,
    title: String,
    author: String,
    year: i32,
}

#[post("/books", data = "<new_book>")]
fn add_new_book(new_book: Json<Book>) -> Json<Book> {
   new_book
}

#[get("/books/<book_id>")]
fn fetch_book_by_id(book_id: i32) -> Option<Json<Book>> {
    None
}

#[put("/books/<book_id>", data = "<updated_book>")]
fn modify_existing_book(book_id: i32, updated_book: Json<Book>) -> Option<Json<Book>> {
    Some(updated_book)
}

#[delete("/books/<book_id>")]
fn remove_book(book_id: i32) -> Option<Json<()>> {
    Some(Json(()))
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![add_new_book, fetch_book_by_id, modify_existing_book, remove_book])
}