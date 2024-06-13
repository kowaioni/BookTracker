import React, { Component } from 'react';
import { BookForm } from './BookForm'; 
import { BookList } from './BookList';
import { NavBar } from './NavBar';
import './MainComponent.css';

interface Book {
  id: number;
  title: string;
  author: string;
  year: number;
}

interface MainComponentState {
  books: Book[];
  editingBook?: Book; 
}

export class MainComponent extends Component<{}, MainComponentState> { 
  state = {
    books: [],
    editingBook: undefined, 
  };

  addBook = (book: Book): void => {
    this.setState(prevState => ({
      books: [...prevState.books, { ...book, id: Date.now() }],
    }));
  };

  editBookStart = (id: number): void => {
    const bookToEdit = this.state.books.find(book => book.id === id);
    if (bookToEdit) {
      this.setState({ editingBook: { ...bookToEdit } });
    }
  };

  editBookSave = (editedBook: Book): void => {
    this.setState(prevState => ({
      books: prevState.books.map(book => book.id === editedBook.id ? editedBook : book),
      editingBook: undefined, 
    }));
  };

  deleteBook = (id: number): void => {
    this.setState(prevState => ({
      books: prevState.books.filter(book => book.id !== id),
    }));
  };

  fetchBooks = (): void => {
    const apiUrl = process.env.REACT_APP_API_URL;

    fetch(`${apiUrl}/books`)
      .then(response => response.json())
      .then(data => this.setState({ books: data }))
      .catch(error => console.error('Error fetching books:', error));
  };

  componentDidMount() {
    this.fetchBooks();
  }

  render() {
    const { books, editingBook } = this.state;

    return (
      <div className="mainComponent">
        <NavBar />
        <div className="content">
          <BookForm addBook={this.addBook} editingBook={editingBook} saveEdit={this.editBookSave} />
          <BookList 
            books={books} 
            deleteBook={this.deleteBook} 
            startEdit={this.editBookStart} 
          />
        </div>
      </div>
    );
  }
}