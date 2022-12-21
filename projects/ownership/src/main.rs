fn main() {
    println!("Hello, world!");
    print_hero_name();
    more_complexity();
    ownership_examples();

    let s1 = gives_ownership(); // gives_ownership moves its return
                                // value into s1

    let s2 = String::from("hello"); // s2 comes into scope

    let s3 = takes_and_gives_back(s2); // s2 is moved into
                                       // takes_and_gives_back, which also
                                       // moves its return value into s3

    let (s4, len) = calculate_length(s3); // s3 is moved into
                                          // calculate_length
                                          // s4 is the return value and
                                          // len is the length of the string
}

fn print_hero_name() {
    let mut s = String::from("hello");
    s.push_str(", world!");
    println!("{}", s);
}

fn more_complexity() {
    let s1 = String::from("hello");
    let s2 = s1.clone();

    // Take a look at Figure 4-1 to see what is happening to String under the covers.
    //A String is made up of three parts, shown on the left: a pointer to the memory that holds the contents of the string, a length, and a capacity.
    //This group of data is stored on the stack. On the right is the memory on the heap that holds the contents.

    println!("{}, world!", s1); // s1 is no longer valid
    println!("{}, world!", s2); // s1 is no longer valid
}

fn ownership_examples() {
    let s = String::from("hello"); // s comes into scope

    takes_ownership(s); // s's value moves into the function...
                        // ... and so is no longer valid here

    let x = 5; // x comes into scope

    makes_copy(x); // x would move into the function,
                   // but i32 is Copy, so it's okay to still
                   // use x afterward
}

fn takes_ownership(some_string: String) {
    // some_string comes into scope
    println!("{}", some_string);
} // Here, some_string goes out of scope and `drop` is called. The backing
  // memory is freed.

fn makes_copy(some_integer: i32) {
    // some_integer comes into scope
    println!("{}", some_integer);
} // Here, some_integer goes out of scope. Nothing special happens.

fn gives_ownership() -> String {
    // gives_ownership will move its
    // return value into the function
    // that calls it

    let some_string = String::from("yours"); // some_string comes into scope

    some_string // some_string is returned and
                // moves out to the calling
                // function
}

// This function takes a String and returns one
fn takes_and_gives_back(a_string: String) -> String {
    // a_string comes into
    // scope

    a_string // a_string is returned and moves out to the calling function
}

fn calculate_length(s: String) -> (String, usize) {
    let length = s.len(); // len() returns the length of a String

    (s, length)
}
