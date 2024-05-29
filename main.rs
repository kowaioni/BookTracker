#[macro_use] extern crate rocket;

use rocket::serde::{json::Json, Deserialize, Serialize};
use std::collections::{HashMap, VecDeque};
use std::sync::Mutex;
use lazy_static::lazy_static;

lazy_static! {
    static ref BOOKS: Mutex<HashMap<String, Book>> = Mutex::new(HashMap::new());

    // Caching the most recently accessed books, assuming the cache size is 5 for demonstration purposes.
    static ref BOOK_CACHE: Mutex<VecDeque<Book>> = Mutex::new(VecDeque::with_capacity(5));
}

const CACHE_SIZE: usize = 5;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
struct Book {
    id: String,
    title: String,
    author: String,
}

fn cache_book(book: &Book) {
    let mut cache = BOOK_CACHE.lock().unwrap();
    if cache.len() == CACHE_SIZE {
        cache.pop_back();
    }
    if !cache.iter().any(|cached_book| &cached_book.id == &book.id) {
        cache.push_front(book.clone());
    }
}

fn get_book_from_cache(id: &str) -> Option<Book> {
    let cache = BOOK_CACHE.lock().unwrap();
    cache.iter().find(|book| book.id == id).cloned()
}

#[post("/books", format = "json", data = "<book>")]
fn create(book: Json<Book>) -> Json<Book> {
    let mut books = BOOKS.lock().unwrap();
    books.insert(book.id.clone(), book.0.clone());
    cache_book(&book.0);
    book
}

#[get("/books/<id>")]
fn retrieve(id: String) -> Option<Json<Book>> {
    if let Some(book) = get_book_from_cache(&id) {
        return Some(Json(book))
    }

    let books = BOOKS.lock().unwrap();
    let book = books.get(&id)?;

    // Update the cache with this book since it's being accessed now.
    cache_book(book);

    Some(Json(book.clone()))
}

#[get("/books")]
fn list() -> Json<Vec<Book>> {
    let books = BOOKS.lock().unwrap();
    let books_list: Vec<Book> = books.values().cloned().collect();
    Json(books_list)
}

#[put("/books/<id>", format = "json", data = "<book>")]
fn update(id: String, book: Json<Book>) -> Option<Json<Book>> {
    let mut books = BOOKS.lock().unwrap();
    if books.contains_key(&id) {
        books.insert(id.clone(), book.0.clone());
        // Update the cache as well since the book is being updated.
        cache_book(&book.0);
        Some(book)
    } else {
        None
    }
}

#[delete("/books/<id>")]
fn delete(id: String) -> Option<Json<bool>> {
    let mut books = BOOKS.lock().unwrap();
    if books.remove(&id).is_some() {
        // Also attempt to remove the book from the cache if it exists there.
        let mut cache = BOOK_CACHE.lock().unwrap();
        if let Some(pos) = cache.iter().position(|book| book.id == id) {
            cache.remove(pos);
        }
        Some(Json(true))
    } else {
        None
    }
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![create, retrieve, update, delete, list])
}