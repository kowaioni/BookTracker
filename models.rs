use std::env;
use std::collections::HashMap;
use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Debug, Clone)]
enum Status {
    Available,
    CheckedOut,
    Reserved,
}

#[derive(Debug, Clone)]
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

struct BookCache {
    cache: HashMap<String, Book>,
}

impl BookCache {
    fn new() -> Self {
        log("Initializing new book cache");
        BookCache {
            cache: HashMap::new(),
        }
    }

    fn get_or_add_book(&mut self, title: &str, author: &str, status: Status) -> &Book {
        log(&format!("Adding or retrieving book: {}", title));
        self.cache.entry(title.to_string()).or_insert_with(|| Book {
            title: title.to_string(),
            author: author.to_string(),
            status: status.clone(),
        })
    }
}

fn log(message: &str) {
    let start = SystemTime::now();
    let since_the_epoch = start.duration_since(UNIX_EPOCH).expect("Time went backwards");
    println!("{:?}: {}", since_the_epoch, message);
}

fn main() -> Result<(), Error> {
    let book_title = env::var("BOOK_TITLE").map_err(Error::from)?;
    let book_author = env::var("BOOK_AUTHOR").map_err(Error::from)?;
    let book_status_env = env::var("BOOK_STATUS").map_err(Error::from)?;
    
    let book_status = get_book_status(&book_status_env)?;

    let mut cache = BookCache::new();
    let my_book = cache.get_or_add_book(&book_title, &book_author, book_status).clone();

    log(&format!("Obtained book: {:?}", my_book));

    Ok(())
}