enum SpreadsheetCell {
    Int(i32),
    Float(f64),
    Text(String),
}

fn main() {
    // let v: Vec<i32> = Vec::new();
    let mut v = vec![1, 2, 3];

    v.push(5);
    v.push(6);
    v.push(7);
    v.push(8);

    let v = vec![1, 2, 3, 4, 5];
    let third: &i32 = &v[2];
    println!("The third element is {}", third);

    let third: Option<&i32> = v.get(2);
    match third {
        Some(third) => println!("The third element is {}", third),
        None => println!("There is no third element."),
    }

    let v = vec![1, 2, 3, 4, 5];

    // let does_not_exist = &v[100];
    let does_not_exist = v.get(100); // safe way to access an element out of bounds lol

    match does_not_exist {
        Some(does_not_exist) => println!("The hundredth element is {}", does_not_exist),
        None => println!("There is no one hundredth element."),
    }

    let mut v = vec![1, 2, 3, 4, 5];

    let first = &v[0];

    // v.push(6);

    println!("The first element is: {first}");

    let v = vec![100, 32, 57];
    for i in &v {
        println!("{i}");
    }

    let mut v = vec![100, 32, 57];
    for i in &mut v {
        *i += 50;
    }

    for i in &v {
        println!("{i}");
    }

    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(10.12),
    ];
}
