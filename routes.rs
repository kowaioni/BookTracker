#[macro_use] extern crate rocket;

#[derive(FromForm, Serialize, Deserialize)]
struct Book {
    id: Option<i32>,
    title: String,
    author: String,
    year: i32,
}

#[post("/books", data = "<book>")]
fn create_book(book: Json<Book>) -> Json<Book> {
   book
}

#[get("/books/<id>")]
fn get_book(id: i32) -> Option<Json<Book>> {
    None
}

#[put("/books/<id>", data = "<book>")]
fn update_book(id: i32, book: Json<Book>) -> Option<Json<Book>> {
    Some(book)
}

#[delete("/books/<Now that everything but the code has been stripped away, let's get the results.<id>")]
fn delete_book(id: i32) -> Option<Json<()>> {
    Some(Json(()))
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![create_book, get_book, update_book, delete_book])
}