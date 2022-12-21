use std::{collections::HashMap, io, str::SplitWhitespace};

fn main() {
    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    let team_name = String::from("Blue");
    let score = scores.get(&team_name).copied().unwrap_or(0);
    println!("{}: {}", team_name, score);

    for (key, value) in &scores {
        println!("{key}: {value}");
    }

    let field_name = String::from("Favorite color");
    let field_value = String::from("Blue");

    let mut map = HashMap::new();
    map.insert(field_name, field_value);

    // println!("{} {}", field_name, field_value);

    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Blue"), 25);

    println!("{:?}", scores);

    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);

    scores.entry(String::from("Yellow")).or_insert(50);
    scores.entry(String::from("Blue")).or_insert(50);

    println!("{:?}", scores);

    let text = "hello world wonderful world";

    let mut map = HashMap::new();

    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }

    // test median and mode function
    let list = vec![
        1, 2, 3, 4, 4, 5, 6, 6, 6, 7, 8, 9, 9, 10, 10, 10, 10, 10, 10,
    ];
    let (median, mode) = get_median_and_mode_from_list(&list);
    println!("median: {}, mode: {}", median, mode);

    // test pig latin function
    let text = "first apple";
    let pig_latin_text = pig_latin(text);
    println!("{}", pig_latin_text);

    text_game();
}

/*
Given a list of integers, use a vector and return the median (when sorted, the value in the middle position) and mode (the value that occurs most often; a hash map will be helpful here) of the list.
Convert strings to pig latin. The first consonant of each word is moved to the end of the word and “ay” is added, so “first” becomes “irst-fay.” Words that start with a vowel have “hay” added to the end instead (“apple” becomes “apple-hay”). Keep in mind the details about UTF-8 encoding!
Using a hash map and vectors, create a text interface to allow a user to add employee names to a department in a company. For example, “Add Sally to Engineering” or “Add Amir to Sales.” Then let the user retrieve a list of all people in a department or all people in the company by department, sorted alphabetically.

*/

//Given a list of integers, use a vector and return the median (when sorted, the value in the middle position)
//and mode (the value that occurs most often; a hash map will be helpful here) of the list.
fn get_median_and_mode_from_list(list: &Vec<i32>) -> (i32, i32) {
    let mut list = list.clone();
    println!("{:?}", list.len());
    list.sort();
    let middle = list.len() / 2;
    let median = list[middle];
    let mut map = HashMap::new();
    for i in list {
        let count = map.entry(i).or_insert(0);
        *count += 1;
    }
    let mut mode = 0;
    let mut mode_count = 0;
    for (key, value) in map {
        if value > mode_count {
            mode = key;
            mode_count = value;
        }
    }
    (median, mode)
}

//Convert strings to pig latin.
//The first consonant of each word is moved to the end of the word and “ay” is added, so “first” becomes “irst-fay.”
//Words that start with a vowel have “hay” added to the end instead (“apple” becomes “apple-hay”). Keep in mind the details about UTF-8 encoding!
fn pig_latin(text: &str) -> String {
    let mut result = String::new();
    for word in text.split_whitespace() {
        let mut chars = word.chars();
        let first_char = chars.next().unwrap();
        if first_char == 'a'
            || first_char == 'e'
            || first_char == 'i'
            || first_char == 'o'
            || first_char == 'u'
        {
            result.push_str(word);
            result.push_str("-hay ");
        } else {
            result.push_str(&word[1..]);
            result.push('-');
            result.push(first_char);
            result.push_str("ay ");
        }
    }
    result
}

// Using a hash map and vectors, create a text interface to allow a user to add employee names to a department in a company.
// For example, “Add Sally to Engineering” or “Add Amir to Sales.”
// Then let the user retrieve a list of all people in a department or all people in the company by department, sorted alphabetically.
fn text_game() {
    let mut company: HashMap<String, Vec<String>> = HashMap::new();
    loop {
        let mut input = String::new();

        // read a line from the terminal
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");

        // remove the newline
        let input = input.trim();

        if input == "quit" {
            break;
        }

        let mut input = input.split_whitespace();

        let command = input.next().unwrap();
        let name = input.next().unwrap();
        let department = input.next().unwrap();
        if command == "Add" {
            company
                .entry(department.to_string())
                .or_insert(Vec::new())
                .push(name.to_string());
        } else if command == "List" {
            if department == "All" {
                for (department, names) in &company {
                    println!("{}: {:?}", department, names);
                }
            } else {
                println!("{:?}", company.get(department).unwrap());
            }
        }
        println!("{:?}", company)
    }
}
