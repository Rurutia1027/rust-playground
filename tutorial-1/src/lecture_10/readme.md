# Closures 
* [Closures]()
* [Functinal Pointers]()
* [Iterators]()
* [IntoIter]()
* [Iterating over Collections]()
* [Combinators]()

## Benefits of Using Closures vs. Declared Functions

1. **Inlined Context and Simplified Code**  
   - Closures allow you to define logic directly at the point of use, reducing the need for creating separate named functions that may clutter the code. This is especially useful for one-off operations or when the logic is tightly coupled with its usage.

2. **Capturing Environment**  
   - Closures can capture variables from their enclosing scope. This eliminates the need to explicitly pass those variables as arguments, making them highly convenient for scenarios where the surrounding state is relevant.

3. **Flexibility with Ownership**  
   - Closures provide fine-grained control over ownership:  
     - `move` closures allow transferring ownership of captured variables.  
     - Non-`move` closures borrow variables, avoiding unnecessary ownership transfers or copies.  
   - This flexibility helps manage Rust’s ownership and borrowing rules effectively.

4. **Efficiency**  
   - Closures in Rust are often inlined and optimized by the compiler. For small, simple operations, using closures can reduce function call overhead and improve runtime performance compared to declaring a separate function.

5. **Improved Code Readability**  
   - By avoiding separate declarations, closures keep the logic closer to where it’s used, making the code easier to read and maintain.

6. **Anonymous and Reusable**  
   - Closures are anonymous and can be passed around as function parameters (e.g., higher-order functions like `map`, `filter`, etc.), enabling a more functional programming style. This reduces boilerplate code.

---

## When to Use Closures

- Use **closures** for short-lived, concise logic that depends on its surrounding context or doesn’t need reuse.
- Use **declared functions** for larger, reusable logic that can exist independently of the surrounding context or needs better documentation.
