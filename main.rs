#[macro_use] extern crate rocket;

use rocket::serde::{json::Json, Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Mutex;
use lazy_static::lazy_static;

lazy_static! {
    static ref BOOKS: Mutex<HashMap<String, Book>> = {
        let m = HashMap::new();
        Mutex::new(m)
    };
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
struct Book {
    id: String,
    title: String,
    author: String,
}

#[post("/books", format = "json", data = "<book>")]
fn create(book: Json<Book>) -> Json<Book> {
    let mut books = BOOKS.lock().unwrap();
    books.insert(book.id.clone(), book.0.clone());
    book
}

#[get("/books/<id>")]
fn retrieve(id: String) -> Option<Json<Book>> {
    let books = BOOKS.lock().unwrap();
    books.get(&id).cloned().map(Json)
}

#[put("/books/<id>", format = "json", data = "<book>")]
fn update(id: String, book: Json<Book>) -> Option<Json<Book>> {
    let mut books = BOOKS.lock().unwrap();
    if books.contains_key(&id) {
        books.insert(id, book.0.clone());
        Some(book)
    } else {
        None
    }
}

#[delete("/books/<id>")]
fn delete(id: String) -> Option<Json<bool>> {
    let mut books = BOOKS.lock().unwrap();
    if books.remove(&id).is_some() {
        Some(Json(true))
    } else {
        None
    }
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![create, retrieve, update, delete])
}