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
}

export class MainDeferredComponent extends Component<{}, MainComponentState> {

  state = {
    books: [],
  };

  addBook = (book: Book): void => {
    this.setState(prevState => ({
      books: [...prevState.books, { ...book, id: Date.now() }],
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
    const { books } = this.state;

    return (
      <div className="mainComponent">
        <NavBar />
        <div className="content">
          <BookForm addBook={this.addBook} />
          <BookList books={books} />
        </div>
      </div>
    );
  }
}