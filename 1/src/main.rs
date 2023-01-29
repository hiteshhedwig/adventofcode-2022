// use std::env;
use std::fs;

fn main() {
    // --snip--

    let file_path = String::from("./elf_calories.txt");
    let mut max_calories:i32 = 0;


    println!("In file {}", file_path);

    let contents = fs::read_to_string(file_path)
        .expect("Should have been able to read the file");

    // println!("With text:\n{contents}");


    let mut calories:i32 = 0;
    let mut length:i32 = 0;
    for text in contents.split("\n") {
        // empty string means next elfs 
        if text.is_empty() {
            if calories > max_calories {
                max_calories = calories;
                println!("Calories : {calories}");
            }
            calories=0; 
        } else {
            calories+=text.parse::<i32>().unwrap();
        }
        length+=1;
        println!("I like {} {}", text, calories);
    }

    println!("Max :\n{max_calories}");
    println!("Length : {length}")
    
}