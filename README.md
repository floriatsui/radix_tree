# Radix Tree
Came across this data structure in the Software Internals discord, when discussing how Linux deals with huge pages and I had never heard of it before. So I looked it up, especially how it was used by Linux and thought it was very fascinating. I've also been looking to write some Rust and what better way to understand something than to implement it. 

Challenging myself to write some Rust and understand and become more comfortable with tree operations. (Still very much a work in progress!)

## Operations to support 
- Right now, for the purposes of an initial implementation, let's have everything be strings for now. 

- Creation of the radix tree (with a given r) or we can just for now hardcode r and initialize according to that. Perhapsh making it a binary but providing further support in a future iteration. 
- Inserting some value 
- Removing some value 
- Lookup some value 

## Questions: 
- what is the best way to test this and verify that this is working as correctly? 

## Todo: 
- Understand Rust conventions when writing a library (as this is what I'm doing essentially)
- Understand how String works in Rust