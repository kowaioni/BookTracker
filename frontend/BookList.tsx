import React from 'react';
import { useSelector, useDispatch } from 'react-redux';
import { RootState } from './store';
import { Book } from './types';
import { deleteBook, fetchBooks } from './bookSlice';

const BookList: React.FC = () => {
  const dispatch = useDispatch();
  
  const books = useSelector((state: RootState) => state.bookState.books);
  
  React.useEffect(() => {
    dispatch(fetchMedicines());
  }, [dispatch]);
  
  const handleDelete = (id: string) => {
    dispatch(deleteBook(id));
  };
  
  return (
    <div>
      <h2>Book List</h2>
      <ul>
        {books.map((book: Book) => (
          <li key={book.id}>
            Title: {book.title}, Author: {book.author}, Status: {book.status}
            <button onClick={() => {/* Implement navigate to edit page */}}>Edit</button>
            <button onClick={() => handleDelete(book.id)}>Delete</button>
          </li>
        ))}
      </ul>
    </div>
  );
}

export default BookList;