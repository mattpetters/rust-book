// fn main() {
//     let r;

//     {
//         let x = 5;
//         r = &x;
//     }

//     println!("r: {}", r);
// }

/*
    Every reference in rust has a lifetime, which is the scope for which that reference is valid.
    Most of the time, lifetimes are implicit and inferred, just like most of the time, types are
    inferred. However, in cases where the lifetimes can't be inferred, the Rust compiler will
    complain. In the above example, the compiler will complain that r does not live long enough.
*/

// fn main() {
//     let x = 5; // ----------+-- 'b
//                //           |
//     let r = &x; // --+-- 'a  |
//                 //   |       |
//     println!("r: {}", r); //   |       |
//                           // --+       |

//     longest_checker();
// } // ----------+
//   // corrected

// fn longest_checker() {
//     let string1 = String::from("long string is long");
//     let result;
//     {
//         let string2 = String::from("xyz");
//         result = longest(string1.as_str(), string2.as_str());
//     }
//     println!("The longest string is {}", result);
//     /*
//         &i32        // a reference
//     &'a i32     // a reference with an explicit lifetime
//     &'a mut i32 // a mutable reference with an explicit lifetime
//     */
// }

// fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
//     if x.len() > y.len() {
//         x
//     } else {
//         y
//     }
// }

use std::fmt::Display;

struct ImportantExcerpt<'a> {
    part: &'a str,
}

fn main() {
    let novel = String::from("Call me Ishmael. Some years ago...");
    let first_sentence = novel.split('.').next().expect("Could not find a '.'");
    let i = ImportantExcerpt {
        part: first_sentence,
    };

    // test longest_with_an_announcement
    let string1 = String::from("long string is long");
    let string2 = String::from("xyz nosadfsdfhasdfasdfasdfasdfadsf");
    let result = longest_with_an_announcement(string1.as_str(), string2.as_str(), 1);
    println!("The longest string is {}", result);
}

fn longest_with_an_announcement<'a, T>(x: &'a str, y: &'a str, ann: T) -> &'a str
where
    T: Display,
{
    println!("Announcement! {}", ann);
    if x.len() > y.len() {
        x
    } else {
        y
    }
}
