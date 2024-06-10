import React, { useState } from 'react';
import { useDispatch } from 'react-redux';
import axios from 'axios';

interface Book {
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
  const [title, setTitle] = useState('');
  const [author, setAuthor] = useState('');
  const [status, setStatus] = useState<'read' | 'currently reading' | 'want to read'>('want to read');

  const dispatch = useDispatch();

  const handleSubmit = async (e: React.FormEvent) => {
    e.preventDefault();
    
    const newBook: Book = { title, author, status };
    
    dispatch(addBook(newBook));
    
    try {
      await axios.post(`${process.env.REACT_APP_BACKEND_URL}/books`, newBook);
    } catch (error) {
      console.error("Error adding book to the backend", error);
    }
  };

  return (
    <form onSubmit={handleSubmit}>
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
    </form>
  );
};

export default AddBook, { ADD_BOOK, addBook };