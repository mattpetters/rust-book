// pointer is a variable that contains an address in memory
// this address points to some data
// most common kind of pointer in rust is a reference
// & symbol, borrows the value it points to

// smart pointers are data structures that act like a pointer
// but also have additional metadata and capabilities
// originated in C++

// references only borrow data, smart pointers own the data they point to

// String and Vec<T> are smart pointers

enum List {
    Cons(i32, Box<List>),
    Nil,
}

use crate::List::{Cons, Nil};

fn main() {
    // let b = Box::new(5);
    // println!("b = {}", b);

    let list = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil))))));
}
