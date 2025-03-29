fn longest_word(sentence: &str) -> Option<&str> { 
    // Your implementation here 
    let words: Vec<&str> = sentence.split_whitespace().collect();
    let longest: Option<&&str> = words.iter().max_by_key(|word| word.len());
    return longest.copied();
}

fn longest_word2(sentence: &str) -> Option<&str> { 
    // Your implementation here 
    let words = sentence.split_whitespace();
    let mut longest: Option<&str> = None;
    for (pos, word) in words.enumerate() {
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
    println!("Longest word: {}", longest_word(&sentence).unwrap());
    println!("Longest word: {}", longest_word2(&sentence).unwrap());
}
