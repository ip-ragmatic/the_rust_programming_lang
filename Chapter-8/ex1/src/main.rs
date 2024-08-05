/*
Problem: Given a list of integers, use a vector and return the median (when sorted, the value in the middle position) and mode (the value that occurs most often; a hash map will be helpful here) of the list.
*/

use rand::{thread_rng, Rng};
use std::collections::HashMap;

fn median(list: &[i32]) -> f64 {
    // if list size even, then return the average of the two middle numbers
    let len = list.len();
    if len % 2 == 0 {
        let a = list[(len / 2) - 1];
        let b = list[len / 2];
        (a + b) as f64 / 2.0
    } else {
        list[len / 2] as f64
    }
}

fn mode(list: &[i32]) -> Vec<(i32, i32)> {
    // returns a vector containing the mode of the list. if there's multiple modes, then they're all returned
    let mut map = HashMap::new();
    for &n in list {
        map.entry(n).and_modify(|count| *count += 1).or_insert(0);
    }

    // map.into_iter().max_by_key(|&(_, v)| v)
    let max_count = map.values().cloned().max().unwrap_or(0);
    map.into_iter()
        .filter(|&(_, val)| val == max_count)
        .collect()
}

fn main() {
    let mut rng = thread_rng();
    let mut list = Vec::new();

    for _ in 0..rng.gen_range(50..=100) {
        list.push(rng.gen_range(-4..=4));
    }

    list.sort();
    let med = median(&list);
    let mode = mode(&list);

    println!("{:?}", list);
    if mode.len() > 1 {
        println!("Median: {med}\nModes: (occur {} times)", mode[0].1);
        for (n, _count) in mode {
            println!("\tMode: {}", n);
        }
    } else {
        println!(
            "Median: {}\nMode: {} (occurs {} times)",
            med, mode[0].0, mode[0].1
        );
    }
}
