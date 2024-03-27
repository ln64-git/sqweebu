// scroll-context.ts
import { createContext, useContext, RefObject } from 'react';

// Specify the type of context to be either a ref to an HTMLDivElement or null
const ScrollContext = createContext<RefObject<HTMLDivElement> | null>(null);

export const useScrollContext = () => useContext(ScrollContext);

export default ScrollContext;
