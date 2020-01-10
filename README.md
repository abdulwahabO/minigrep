## Minigrep


_Project from chapter 12 of [The Rust Programming Language](https://doc.rust-lang.org/book/ch12-00-an-io-project.html)_ 

A command line tool that searches a specified file for a specified string and prints all the lines containing the string. Searches are case sensitive and error messages are printed to the standard error console stream.

---
The easiest way to try this out is by using the sample file `test.txt` which is at the project root.

```sh
cargo run test.txt today
``` 

```sh
cargo run <path-to-file> <string-to-search-for>
```
 
To run all automated tests...

```sh
cargo test -- --nocapture
``` 
