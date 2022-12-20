fn main() {
    println!("Hello, world!");

    // another_function(3);
    print_labeled_measurement(3, 'h');
    expression_test();
    let x = five();
    println!("The value of x is: {}", x);
    let x = plus_one(5);
    println!("The value of x is: {}", x);
}

fn print_labeled_measurement(value: i32, label: char) {
    println!("The value is {} {}", value, label);
}

fn expression_test() {
    let y = {
        let x = 3;
        x + 1
    };
    println!("The value of y is: {}", y);
}

fn five() -> i32 {
    5
}

fn plus_one(x: i32) -> i32 {
    x + 1
}
