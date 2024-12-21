use std::collections::HashMap;

fn main() {
    let start_str: &str = "Doing pig latin code";
    let letters: HashMap<char, &str> = HashMap::from([('a', "-hay"), ('b', "-bay"), ('c', "-cay"),
        ('d', "-day"), ('e', "-hay"), ('f', "-fay"), ('g', "-gay"), ('h', "-hay"), ('i', "-hay"),
        ('j', "-jay"), ('k', "-kay"), ('l', "-lay"), ('m', "-may"), ('n', "-nay"), ('o', "-hay"),
        ('p', "-pay"), ('q', "-qay"), ('r', "-ray"), ('s', "-say"), ('t', "-tay"), ('u', "-hay"),
        ('v', "-vay"), ('w', "-way"), ('x', "-xay"), ('y', "-hay"), ('z', "-zay")]);
    let mut frmtd_str: String = String::from(start_str);
    frmtd_str = frmtd_str.trim().to_lowercase();
    let words: Vec<&str> = frmtd_str.split(" ").collect();
    let mut new_words: Vec<String> = Vec::new();
    let mut new_word: String;
    for word in &words {
        if match "aeiouy".find(&word[0..1]) {
            Some(expr) => true,
            None => false,
        }{
            new_word = String::from(*word);
            new_word = new_word + letters.get(&word.chars().collect::<Vec<_>>()[0]).unwrap();
            new_words.push(new_word);  
        } else {
            new_word = String::from(&word[1..]);
            new_word = new_word + letters.get(&word.chars().collect::<Vec<_>>()[0]).unwrap();
            new_words.push(new_word);
        }
    }
    let mut final_string: String = String::new();
    for word in &new_words {
        final_string.push_str(word.as_str())
    }
    println!("{:?}", new_words);
}
