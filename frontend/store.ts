import { configureStore } from '@reduxjs/toolkit';
import booksReducer from './features/books/booksSlice';

export const store = configureStore({
  reducer: {
    books: books레
  },
});

export type RootState = ReturnType<typeof store.getState>;
export type AppDispatch = typeof store.dispatch;