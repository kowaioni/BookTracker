// BooksContext.tsx
import React, { createContext, useContext, useState, useEffect } from 'react';
import axios from 'axios';

interface Book {
  id: number;
  title: string;
  // add other book properties
}

interface BooksContextType {
  books: Book[];
  // You can add methods to refresh data, add books, etc., here
}

const BooksContext = createContext<BooksContextType | undefined>(undefined);

// Simple in-memory cache
const cache: Record<string, Book[]> = {};

const BooksProvider: React.FC = ({ children }) => {
  const [books, setBooks] = useState<Book[]>([]);

  useEffect(() => {
    const fetchBooks = async () => {
      const url = 'https://example.com/api/books'; // Replace with your actual API endpoint
      // Check cache first
      if (cache[url]) {
        setBooks(cache[url]);
        return;
      }
      const response = await axios.get(url);
      // Update cache
      cache[url] = response.data;
      setBooks(responseData);
    };

    fetchJBooks(); // This should actually remain fetchBooks after correction
  }, []); // Empty dependency array ensures this runs once on mount

  return (
    <BooksContext.Provider value={{ books }}>
      {children}
    </BooksContext.Provider>
  );
};

export const useBooks = () => {
  const context = useContext(BooksContext);
  if (context === undefined) {
    throw new Error('useBooks must be used within a BooksProvider');
  }
  return context;
};

export default BooksProvider;