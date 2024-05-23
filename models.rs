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

#[derive(Debug)]
enum Error {
    MissingEnvVar(String),
    InvalidStatus(String),
}

impl From<std::env::VarError> for Error {
    fn from(e: std::env::VarError) -> Self {
        match e {
            std::env::VarError::NotPresent => Error::MissingEnvVar("Environment variable not set".to_string()),
            std::env::VarError::NotUnicode(_) => Error::MissingEnvVar("Environment variable was not valid unicode.".to_string()),
        }
    }
}

fn get_book_status(status: &str) -> Result<Status, Error> {
    match status {
        "Available" => Ok(Status::Available),
        "CheckedOut" => Ok(Status::CheckedOut),
        "Reserved" => Ok(Status::Reserved),
        _ => Err(Error::InvalidStatus(format!("Invalid status: {}", status))),
    }
}

fn main() -> Result<(), Error> {
    let book_title = env::var("BOOK_TITLE").map_err(Error::from)?;
    let book_author = env::var("BOOK_AUTHOR").map_err(Error::from)?;
    let book_status_env = env::var("BOOK_STATUS").map_err(Error::from)?;

    let book_status = get_book_status(&book_status_env)?;

    let my_book = Book {
        title: book_title,
        author: book_author,
        status: book_status,
    };

    println!("{:?}", my_book);
    Ok(())
}