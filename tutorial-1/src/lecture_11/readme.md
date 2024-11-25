# Lecture 11 Lifetimes && Smart Pointer in Rust 

## Concrete Lifetimes 

A concrete lifetime is the lifetime which a value exists inside the memory. 
The lifetime of a value starts when it is created and ends when the value is dropped, or moved out of the particular memory location mainly due to the change of the ownership. 


## Simple Pointer vs. Smart Pointer in Rust 
### Simple Pointer 
* Just stores memory address 
* Indicated by & 
* Also called references(which its value is the memory space address)
* No special capabilities. 

### Smart Pointers 
* Special capabilities 
* Not just simple references 


