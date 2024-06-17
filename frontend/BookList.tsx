import React, { useState } from 'react';
import { useSelector, useDispatch } from 'react-redux';
import { RootState } from './store';
import { Book } from './types';
import { deleteBook, fetchBooks } from './bookSlice';

const BookList: React.FC = () => {
  const dispatch = useDispatch();
  
  const books = useSelector((state: RootState) => state.bookState.books);
  const [error, setError] = useState<string | null>(null);

  React.useEffect(() => {
    dispatch(fetchBooks())
      .catch(err => setError("Failed to fetch books. Please try again later."));
  }, [dispatch]);

  const handleDelete = (id: string) => {
    dispatch(deleteBook(id))
      .catch(err => setError(`Failed to delete the book (ID: ${id}). Please try again.`));
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

export default BookList;