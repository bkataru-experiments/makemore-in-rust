#![allow(unused_variables, unused_imports, dead_code, unused_mut)]

use std::collections::HashMap;
use std::error::Error;
use std::fs;

use std::io;

use candle_core::{Device, Tensor};

fn candle() -> Result<(), Box<dyn Error>> {
    let device = Device::Cpu;

    let a = Tensor::randn(0_f32, 1., (2, 3), &device)?;
    let b = Tensor::randn(0_f32, 1., (3, 4), &device)?;

    let c = a.matmul(&b)?;
    println!("{c}");

    Ok(())
}

fn lengths(words: &[&str]) {
    let word_lengths: Vec<usize> = words.iter().map(|word| word.len()).collect();

    let total = words.len();
    let min_length = word_lengths.iter().min().unwrap();
    let max_length = word_lengths.iter().max().unwrap();

    println!("count : {total}");
    println!("smallest word length : {min_length}");
    println!("largest word length : {max_length}");
}

fn bigrams(words: &[&str]) {
    let mut b: HashMap<(String, String), u32> = HashMap::new();

    for word in words {
        // let chars: Vec<String> = word.chars().map(|c| c.to_string()).collect();
        let chars: Vec<String> = [
            vec!["<S>".to_string()],
            word.chars().map(|c| c.to_string()).collect(),
            vec!["<E>".to_string()],
        ]
        .concat();

        for (ch1, ch2) in chars.iter().zip(chars.iter().skip(1)) {
            let bigram = (ch1.clone(), ch2.clone());

            b.entry(bigram).and_modify(|c| *c += 1).or_insert(1);
        }
    }

    let mut items: Vec<((String, String), u32)> = b.into_iter().collect();

    items.sort_by(|a, b| b.1.cmp(&a.1));

    for (bigram, count) in &items {
        println!("{bigram:?} : {count}");
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let content = fs::read_to_string("names.txt")?;
    let words: Vec<&str> = content.lines().collect();

    lengths(&words);

    bigrams(&words);

    // candle()?;

    Ok(())
}
