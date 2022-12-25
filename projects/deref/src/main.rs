use std::ops::Deref;

fn main() {
    let x = 5;
    let y = &x;

    assert_eq!(5, x);
    assert_eq!(5, *y); // follow y ref to the value it refers to, * is deref operator

    let x = 5;
    let y = Box::new(x);

    assert_eq!(5, x);
    assert_eq!(5, *y); // follow y ref to the value it refers to, * is deref operator

    let x = 5;
    let y = MyBox::new(x);

    assert_eq!(5, x);
    assert_eq!(5, *y); // follow y ref to the value it refers to, * is deref operator

    let m = MyBox::new(String::from("Rust"));
    hello(&m); // hello(&(*m)[..]); // deref coercion

    let c = CustomSmartPointer {
        data: String::from("my stuff"),
    };

    let d = CustomSmartPointer {
        data: String::from("other stuff"),
    };

    // c.drop(); // manually call drop WRONG
    drop(c); // manually call drop
    println!("CustomSmartPointers created.");

    rctest();
    rctest2();
}

struct MyBox<T>(T);

impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}

impl<T> Deref for MyBox<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0 // .0 is the first field of the tuple struct
    }
}

fn hello(name: &str) {
    println!("Hello, {}!", name);
}

struct CustomSmartPointer {
    data: String,
}

impl Drop for CustomSmartPointer {
    fn drop(&mut self) {
        println!("Dropping CustomSmartPointer with data `{}`!", self.data);
    }
}

enum List {
    Cons(i32, Rc<List>),
    Nil,
}

use crate::List::{Cons, Nil};
use std::rc::Rc;

fn rctest() {
    let a = Rc::new(Cons(5, Rc::new(Cons(10, Rc::new(Nil)))));
    let b = Cons(3, Rc::clone(&a));
    let c = Cons(4, Rc::clone(&a));

    /*
    The call to Rc::clone only increments the reference count, which doesn’t take much time.
    Deep copies of data can take a lot of time. By using Rc::clone for reference counting, we can
    visually distinguish between the deep-copy kinds of clones and the kinds of clones that increase the reference count.
    When looking for performance problems in the code, we only need to consider the deep-copy clones and can disregard calls to Rc::clone.
    */
}

fn rctest2() {
    let a = Rc::new(Cons(5, Rc::new(Cons(10, Rc::new(Nil)))));
    println!("count after creating a = {}", Rc::strong_count(&a));
    let b = Cons(3, Rc::clone(&a));
    println!("count after creating b = {}", Rc::strong_count(&a));
    {
        let c = Cons(4, Rc::clone(&a));
        println!("count after creating c = {}", Rc::strong_count(&a));
    }
    println!("count after c goes out of scope = {}", Rc::strong_count(&a));
    // the implementation of the Drop trait decreases the reference count automatically when an Rc<T> value goes out of scope.

    /*
    Via immutable references, Rc<T> allows you to share data between multiple parts of your program for reading only.
     If Rc<T> allowed you to have multiple mutable references too, you might violate one of the borrowing rules
     discussed in Chapter 4: multiple mutable borrows to the same place can cause data races and inconsistencies.
    */
}
