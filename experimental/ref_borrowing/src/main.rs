// References or borrowing
//
// SPLIT Strings:
// There are two methods to split a string data
// First one: let words: Vec<&str> = sentence.split_whitespace().collect();
// Second one: let words: std::str::Split<'_, char> = sentence.split(' ');

// For next functions, as an arguments, we need to pass a referenced data because
// we need to work under in the same context of memory
// remember that the symbol "&" means a reference or borrowing the
// function need from outside. This kind if references are necessary
// if you need to modify the data or if you need to work in the same context
// dictated from the argument

// ChatGPT's Method to know what is the longest word in a sentence
fn longest_word(sentence: &str) -> Option<&str> { 
    let words: Vec<&str> = sentence.split_whitespace().collect();
    // in this line througth a in line iteration we can get the logest word 
    let longest: Option<&&str> = words.iter().max_by_key(|word| word.len());
    // to cast from Option<&&str> to Option<&str>, we call copied function
    return longest.copied();
}

// My Method to know what is the longest word in a sentence
fn longest_word2(sentence: &str) -> Option<&str> { 
    let words: std::str::Split<'_, char> = sentence.split(' ');
    // In the same way to gpt method, I wrote this for to check and get the logest word
    let mut longest: Option<&str> = None;
    // in this case (_, word) means = (key, value) from the enumerate collection
    for (_, word) in words.enumerate() {
        if word.len() > longest.unwrap_or("").len() {
            longest = Some(word);
        }
    }
    return longest;
}

fn main() {
    println!("Type a sentence:");
    let mut sentence: String = String::new();
    std::io::stdin()
        .read_line(&mut sentence)
        .expect("Failed to read line");
    println!("Sentence: {}", sentence.trim().len());
    if sentence.trim().len() == 0 {
        println!("None");
        return;
    }
    // to cast from Option<$str> to String, we call unwrap function
    println!("Longest word with max_by_key: {}", longest_word(&sentence).unwrap());
    println!("Longest word with for: {}", longest_word2(&sentence).unwrap());
}
