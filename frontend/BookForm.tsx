import React, { useState } from 'react';
import { useDispatch } from 'react-redux';
import axios from 'axios';

interface Book {
  isbn?: string;
  title: string;
  author: string;
  status: 'read' | 'currently reading' | 'want to read';
}

const ADD_BOOK = 'ADD_BOOK';

const addBook = (book: Book) => ({
  type: ADD_BOOK,
  payload: book,
});

const AddBookForm: React.FC = () => {
  const [isbn, setIsbn] = useState('');
  const [title, setTitle] = useState('');
  const [author, setAuthor] = useState('');
  const [status, setStatus] = useState<'read' | 'currently reading' | 'want to read'>('want to read');
  const [error, setError] = useState<string | null>(null);

  const dispatch = useDispatch();

  const handleFetchBookDetails = async () => {
    setError(null); // Reset error state before fetching
    try {
      // Example API - replace with an actual ISBN book details API endpoint if available
      const response = await axios.get(`https://www.example.com/api/books/${isbn}`);
      const data = response.data;
      
      // Assuming the response has title and author fields
      setTitle(data.title);
      setAuthor(data.author);
    } catch (error) {
        console.error("Error fetching book details", error);
        setError("Failed to fetch book details. Please check the ISBN and try again.");
    }
  };

  const handleSubmit = async (e: React.FormEvent) => {
    e.preventDefault();
    setError(null); // Reset error state on a new submission

    const newBook: Book = { isbn, title, author, status };
    
    try {
      await axios.post(`${process.env.REACT_APP_BACKEND_URL}/books`, newBook);
      dispatch(addBook(newBook));
    } catch (error) {
        console.error("Error adding book to the backend", error);
        setError("There was an error adding your book. Please try again.");
    }
  };

  return (
    <form onSubmit={handleSubmit}>
      <label htmlFor="isbn">ISBN:</label>
      <input
        id="isbn"
        value={isbn}
        onChange={(e) => setIsbn(e.target.value)}
      />
      <button type="button" onClick={handleFetchBookDetails}>Fetch Details</button>
      
      <label htmlFor="title">Title:</label>
      <input
        id="title"
        value={title}
        onChange={(e) => setTitle(e.target.value)}
        required
      />
      
      <label htmlFor="author">Author:</label>
      <input
        id="author"
        value={author}
        onChange={(e) => setAuthor(e.target.value)}
        required
      />
      
      <label htmlFor="status">Status:</label>
      <select
        id="status"
        value={status}
        onChange={(e) => setStatus(e.target.value as 'read' | 'currently reading' | 'want to read')}
        required
      >
        <option value="want to read">Want to Read</option>
        <option value="currently reading">Currently Reading</option>
        <option value="read">Read</option>
      </select>
      
      <button type="submit">Add Book</button>
      {error && <div style={{ color: 'red' }}>{error}</div>}
    </form>
  );
};

export default AddBook’s Form;
export { ADD_BOOK, addBook };