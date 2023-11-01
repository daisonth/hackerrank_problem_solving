use std::collections::HashMap;
use std::env;
use std::fs::File;
use std::io::{self, BufRead, Write};

/*
 * - The function accepts string slice(&str) s as parameter.
 * - This function return an i32 value which is the number of unordered anagrammatic pairs of substrings in s.
 */

fn sherlock_and_anagrams(s: &str) -> usize {
    // Convert the input string into a vector of bytes.
    let vec = s.as_bytes();

    // Initialize a counter for counting anagrammatic pairs.
    let mut count: usize = 0;

    // Create a HashMap to store sorted substrings and their counts.
    let mut set: HashMap<Vec<u8>, usize> = HashMap::new();

    // Iterate through different substring lengths, starting from 1.
    for i in 1..vec.len() {
        // Iterate over all substrings of length 'i'.
        for j in vec.windows(i) {
            // Convert the current substring to a Vec<u8> and sort it.
            let mut sorted_substring: Vec<u8> = j.to_vec();

            // Sort the characters in the substring to create a canonical representation
            // that allows us to group anagrams together for counting.
            sorted_substring.sort();

            // Check if the sorted substring exists in the HashMap and update its count.
            let v = set.entry(sorted_substring.clone()).or_insert(0);
            *v += 1;
        }
    }

    // Iterate through the HashMap to count anagrammatic pairs.
    for (_, value) in set.iter() {
        for i in 1..*value {
            // For each count of sorted substrings, calculate the number of pairs.
            count += value - i;
        }
    }

    // Convert the count to i32 and return it.
    count
}

fn main() {
    let stdin = io::stdin();
    let mut stdin_iterator = stdin.lock().lines();

    let mut fptr = File::create(env::var("OUTPUT_PATH").unwrap()).unwrap();

    // read q (number of queries)
    let q = stdin_iterator
        .next()
        .unwrap()
        .unwrap()
        .trim()
        .parse::<i32>()
        .unwrap();

    // for each q lines read string s
    for _ in 0..q {
        let s = stdin_iterator.next().unwrap().unwrap();

        let result = sherlock_and_anagrams(&s);

        writeln!(&mut fptr, "{}", result).ok();
    }
}
