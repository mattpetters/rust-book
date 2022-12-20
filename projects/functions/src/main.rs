fn main() {
    println!("Hello, world!");

    // another_function(3);
    print_labeled_measurement(3, 'h');
    expression_test();
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
