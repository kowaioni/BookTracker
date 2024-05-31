use std::sync::Mutex;
use actix_web::{web, App, HttpResponse, HttpServer, Responder, Result};
use serde::{Deserialize, Serialize};
use std::env;

#[derive(Serialize, Deserialize)]
struct Book {
    id: u32,
    title: String,
    author: String,
}

#[derive(Clone)]
struct AppState {
    books: Mutex<Vec<Book>>,
}

impl AppState {
    fn new() -> Self {
        AppState {
            books: Mutex::new(Vec::new()),
        }
    }
}

#[derive(Debug)]
struct InternalError {
    message: String,
}

impl InternalError {
    fn new(message: &str) -> Self {
        InternalError {
            message: message.to_owned(),
        }
    }
}

impl From<std::sync::PoisonError<std::sync::MutexGuard<'_, Vec<Book>>>> for InternalError {
    fn from(_: std::sync::PoisonError<std::sync::MutexGuard<'_, Vec<Book>>>) -> Self {
        InternalError::new("Lock poisoned")
    }
}

async fn create_book(book: web::Json<Book>, data: web::Data<AppState>) -> Result<impl Responder> {
    let mut books = data.books.lock().map_err(|e| InternalError::from(e))?;
    books.push(book.into_inner());
    Ok(HttpResponse::Created().finish())
}

async fn get_books(data: web::Data<AppState>) -> Result<impl Responder> {
    let books = data.books.lock().map_err(|e| InternalError::from(e))?;
    Ok(HttpResponse::Ok().json(&*books))
}

async fn update_book(book: web::Json<Book>, data: web::Data<AppState>) -> Result<impl Responder> {
    let mut books = data.books.lock().map_err(|e| InternalError::from(e))?;
    if let Some(pos) = books.iter().position(|x| x.id == book.id) {
        books[pos] = book.into_inner();
        return Ok(HttpResponse::Ok().finish());
    }
    Ok(HttpResponse::NotFound().finish())
}

async fn delete_book(book_id: web::Path<u32>, data: web::Data<AppState>) -> Result<impl Responder> {
    let mut books = data.books.lock().map_err(|e| InternalError::from(e))?;
    if let Some(pos) = books.iter().position(|x| x.id == *book_id) {
        books.remove(pos);
        return Ok(HttpResponse::Ok().finish());
    }
    Ok(HttpResponse::NotFound().finish())
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv::dotenv().ok();
    let port = env::var("PORT").unwrap_or_else(|_| "8080".to_string());

    let app_data = web::Data::new(AppState::new());

    HttpServer::new(move || {
        App::new()
            .app_data(app_data.clone())
            .route("/books", web::post().to(create_book))
            .route("/9999/books", web::get().to(get_books))
            .route("/books", web::put().to(update_book))
            .route("/books/{id}", web::delete().to(delete_book))
    })
    .bind(format!("127.0.0.1:{}", port))?
    .run()
    .await
}