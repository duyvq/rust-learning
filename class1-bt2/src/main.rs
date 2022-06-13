use std::fs::File;
use std::io::Read;
use std::io;

fn main() {
    // Read the text file into program, change "/mnt/f/VSCODE/rust-project/class1-bt2/string.txt" to path of file
    let mut data = String::new();
    let mut f = File::open("/mnt/f/VSCODE/rust-project/rust-learning/class1-bt2/string.txt").expect("Unable to open file");
    f.read_to_string(&mut data).expect("Unable to read string");

    // Prompt for word from user
    println!("Please input word:");
    let mut word = String::new();

    // Handling Potential Failure
    io::stdin()
        .read_line(&mut word)
        .expect("Failed to read line");

    // Convert user's word to uppercase, then remove "\n" at end of the word
    let mut word = word.to_uppercase();
    word.pop();

    // Convert string in text file to uppercase.
    let data = data.to_uppercase();

    // Calculate length of user's word
    let word_length = word.chars().count();

    // Calculate length of text file
    let data_length = data.chars().count();

    // Set counter for how many word found in text file
    let mut counter = 0;

    // Iterate over the text file and compare user's word to text file
    for i in 0..(data_length - word_length + 1) {
        let slice = &data[i..(word_length+i)];
        if word == slice {
            counter += 1;
        } else {
            counter = counter;
        }
    }

    // Print out result
    println!{"There are {} word(s) in text file.", counter};
}