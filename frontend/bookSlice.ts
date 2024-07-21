import { createSlice, PayloadAction } from '@reduxjs/toolkit';

type Book = {
  id: string;
  title: string;
  author: string;
  isReading: boolean; // Added to track if the book is currently being read
  isCompleted: boolean; // Added to track if the book is completed
};

type BooksState = {
  books: Book[];
};

const initialState: BooksState = {
  books: [],
};

const booksSlice = createSlice({
  name: 'books',
  initialState,
  reducers: {
    addBook: (state, action: PayloadAction<Book>) => {
      state.books.push(action.payload);
    },
    updateBook: (state, action: PayloadAction<Book>) => {
      const index = state.books.findIndex(book => book.id === action.payload.id);
      if (index !== -1) {
        state.books[index] = action.payload;
      }
    },
    removeBook: (state, action: PayloadAction<string>) => {
      state.books = state.books.filter(book => book.id !== action.payload);
    },
    markAsReading: (state, action: PayloadAction<string>) => {
      const book = state.books.find(book => book.id === action.payload);
      if (book) {
        book.isReading = true;
        book.isCompleted = false; // Ensure a book marked as reading is not completed
      }
    },
    markAsCompleted: (state, action: PayloadAction<string>) => {
      const book = state.books.find(book => book.id === action.payload);
      if (book) {
        book.isCompleted = true;
        book.isReading = false; // If a book is completed, it's no longer being read
      }
    },
  },
});

export const { addBook, updateBook, removeBook, markAsReading, markAsCompleted } = booksSlice.actions;
export default booksSlice.reducer;