import { configureStore, createSlice, PayloadAction, createAsyncThunks } from '@reduxjs/toolkit';

interface Book {
  id: number;
  title: string;
  author: string;
}

interface BooksState {
  books: Book[];
  error: string | null;
}

const initialState: BooksState = {
  books: [],
  error: null
};

const booksSlice = createSlice({
  name: 'books',
  initialState,
  reducers: {
  },
  extraReducers: (builder) => {
  }
});

const booksReducer = booksSlice.reducer;

export const store = configureStore({
  reducer: {
    books: booksReducer,
  },
});

export type RootState = ReturnType<typeof store.getState>;
export type AppDispatch = typeof store.dispatch;