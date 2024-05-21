use std::env;

#[derive(Debug)]
enum Status {
    Available,
    CheckedOut,
    Reserved,
}

#[derive(Debug)]
struct Book {
    title: String,
    author: String,
    status: Status,
}

fn main() {
    let book_title = env::var("BOOK_TITLE").expect("BOOK_TITLE must be set");
    let book_author = env::var("BOOK_AUTHOR").expect("BOOK_AUTHOR must be set");
    let book_status_env = env::var("BOOK_STATUS").expect("BOOK_STATUS must be set");

    let book_status = match book_status_env.as_str() {
        "Available" => Status::Available,
        "CheckedOut" => Status::CheckedOut,
        "Reserved" => Status::Reserved,
        _ => panic!("Invalid status"),
    };

    let my_book = Book {
        title: book_title,
        author: book_author,
        status: book_status,
    };

    println!("{:?}", my_book);
}