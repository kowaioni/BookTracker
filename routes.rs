#[macro_use] extern crate rocket;

use rocket::serde::{json::Json, Deserialize, Serialize};

#[derive(FromForm, Serialize, Deserialize)]
struct Book {
    id: Option<i32>,
    title: String,
    author: String,
    publication_year: i32,
}

#[post("/books", data = "<book>")]
fn add_book(book: Json<Book>) -> Json<Book> {
   book
}

#[get("/books/<id>")]
fn get_book_by_id(id: i32) -> Option<Json<Book>> {
    None
}

#[put("/books/<id>", data = "<book_update>")]
fn update_book(id: i32, book_update: Json<Book>) -> Option<Json<Book>> {
    Some(book_update)
}

#[delete("/books/<id>")]
fn delete_book(id: i32) -> Option<Json<()>> {
    Some(Json(()))
}

#[launch]
fn launch_rocket() -> _ {
    rocket::build()
        .mount("/", routes![add_book, get_book_by_id, update_book, delete_book])
}