use actix_web::{web, App, HttpResponse, HttpServer, Responder, Result};
use serde::{Deserialize, Serialize};
use std::env;
use std::sync::Mutex;

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

fn lock_poisoned_response() -> HttpResponse {
    HttpResponse::InternalServerError().body("Lock poisoned")
}

async fn create_book(book: web::Json<Book>, data: web::Data<AppState>) -> Result<impl Responder> {
    let mut books = data.books.lock().map_err(|_| lock_poisoned_response())?;
    books.push(book.into_inner());
    Ok(HttpResponse::Created().finish())
}

async fn get_books(data: web::Data<AppState>) -> Result<impl Responder> {
    let books = data.books.lock().map_err(|_| lock_poisoned_response())?;
    Ok(HttpResponse::Ok().json(&*books))
}

async fn update_book(book: web::Json<Book>, data: web::Data<AppState>) -> Result<impl Responder> {
    let mut books = data.books.lock().map_err(|_| lock_poisoned_response())?;
    if let Some(pos) = books.iter().position(|x| x.id == book.id) {
        books[pos] = book.into_inner();
        return Ok(HttpResponse::Ok().finish());
    }
    Ok(HttpResponse::NotFound().finish())
}

async fn delete_book(book_id: web::Path<u32>, data: web::Data<AppState>) -> Result<impl Responder> {
    let mut books = data.books.lock().map_err(|_| lock_poisoned_response())?;
    if books.iter().any(|x| x.id == *book_id) {
        books.retain(|x| x.id != *book_id);
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
            .route("/books", web::get().to(get_books))
            .route("/books", web::put().to(update_book))
            .route("/books/{id}", web::delete().to(delete_book))
    })
    .bind(format!("127.0.0.1:{}", port))?
    .run()
    .await
}