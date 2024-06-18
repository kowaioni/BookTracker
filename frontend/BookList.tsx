import React, { useState, useEffect } from 'react';
import { useSelector, useDispatch } from 'react-redux';
import { RootState } from './store';
import { Book } from './types';
import { deleteBook, fetchBooks } from './bookSlice';

const BookList: React.FC = () => {
  const dispatch = useDispatch();
  
  const books = useSelector((state: RootState) => state.bookState.books);
  const [error, setError] = useState<string | null>(null);

  const processError = (err: any) => {
    // Basic error processing
    if (err instanceof Error) {
      return err.message;
    } else if (typeof err === 'string') {
      return err;
    } else {
      return 'An unexpected error occurred. Please try again later.';
    }
  };

  useEffect(() => {
    dispatch(fetchBooks())
      .catch(err => {
          const errorMessage = processError(err);
          setError(`Failed to fetch books. ${errorMessage}`);
      });
  }, [dispatch]);

  const handleDelete = (id: string) => {
    dispatch(deleteBook(id))
      .catch(err => {
          const errorMessage = processError(err);
          setError(`Failed to delete the book (ID: ${id}). ${errorMessage}`);
      });
  };

  return (
    <div>
      <h2>Book List</h2>
      {error && <p style={{color: 'red'}}>{error}</p>}
      <ul>
        {books.map((book: Book) => (
          <li key={book.id}>
            Title: {book.title}, Author: {book.author}, Status: {book.status}
            <button onClick={() => handleDelete(book.id)}>Delete</button>
          </li>
        ))}
      </ul>
    </div>
  );
}

export default Book collabor