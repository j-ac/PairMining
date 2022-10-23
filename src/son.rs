use core::num;
use std::collections::HashMap;
use std::fs::File;
use std::hash::Hash;
use std::io::prelude::*;
use std::io::BufReader;
use std::ops::Add;
use std::path::Path;
use std::time::Instant;

const DIRECTORY: &str = "./netflix.data";
const LINES_IN_INPUT: usize = 480188;
// Retail: 88162
// Netflix: 480188

const NUM_CHUNKS: usize = 4;

pub fn run(THRESHOLD: f64) -> std::io::Result<()> {
    let start = Instant::now();

    let end_line = 0;
    let mut frequent_item_subsets: Vec<HashMap<(usize, usize), usize>> = Vec::new();

    // Get a frequent subset for each
    for chunk in 0..NUM_CHUNKS {
        let start_line =
            ((LINES_IN_INPUT as f64 / NUM_CHUNKS as f64) * (chunk) as f64).ceil() as usize;
        let end_line =
            ((LINES_IN_INPUT as f64 / NUM_CHUNKS as f64) * (chunk + 1) as f64).ceil() as usize;

        let (frequent_items, minimum_support) = get_item_counts(start_line, end_line, THRESHOLD);
        let frequent_pairs = get_frequent_pairs(frequent_items, minimum_support);

        frequent_item_subsets.push(frequent_pairs);
    }

    // Find which pairs were frequent in at least one subset
    let mut frequent_candidates: HashMap<(usize, usize), bool> = HashMap::new();
    for subset in frequent_item_subsets {
        for (key, _) in subset.iter() {
            frequent_candidates.insert(*key, true);
        }
    }

    let minimum_support: usize = (THRESHOLD * LINES_IN_INPUT as f64).ceil() as usize;
    let verified_pairs = get_verified_frequent_pairs(frequent_candidates, minimum_support);

    let mut out = std::fs::File::create(format!("SON{}{}{}", THRESHOLD, DIRECTORY, ".out")).unwrap();
    for (key, _) in verified_pairs.iter() {

        write!(out, "{:?}\n", key);
    }

    println!("\nRuntime: {:.2?}", start.elapsed());
    Ok(())
}

// Performs pass one
fn get_item_counts(start_line: usize, stop_line: usize, THRESHOLD: f64) -> (HashMap<usize, usize>, usize) {
    let f = File::open(DIRECTORY).unwrap();
    let reader = BufReader::new(f);

    let mut counts = HashMap::new();
    let mut num_baskets: usize = 0;
    for (i, line) in reader.lines().skip(start_line).enumerate() {
        if (i == stop_line) {
            break;
        }
        num_baskets += 1;
        for item in line.unwrap().split_whitespace() {
            let item_as_usize = item.parse::<usize>().unwrap(); // string -> int

            match counts.get(&item_as_usize) {
                Some(value) => {
                    counts.insert(item_as_usize, value + 1);
                } //Use of pointer greatly reduces insertion cost
                None => {
                    counts.insert(item_as_usize, 1);
                } //initialize to 1
            }
        }
    }

    let minimum_support = (num_baskets as f64 * THRESHOLD).ceil() as usize;
    counts.retain(|_, &mut v| v >= minimum_support); // Keep everything meeting the threshold
    (counts, minimum_support)
}

fn get_frequent_pairs(
    counts: HashMap<usize, usize>,
    minimum_support: usize,
) -> HashMap<(usize, usize), usize> {
    let mut ret: HashMap<(usize, usize), usize> = HashMap::new();

    let f = File::open(DIRECTORY).unwrap();
    let reader = BufReader::new(f);

    let mut c = 0; //Debug
    for line in reader.lines() {
        /* c += 1; //Debug
        if (c % 4800 == 0) {println!("{}", c);} //Debug */
        let l = line.unwrap();
        let items: Vec<&str> = l.split_whitespace().collect();

        let mut items_usize: Vec<usize> = Vec::new();
        items
            .iter()
            .for_each(|f| items_usize.push(f.parse::<usize>().unwrap())); // strings interpreted as integers

        for i in 0..(items_usize.len() - 1) {
            let i1 = *items_usize.get(i).unwrap();
            if counts.contains_key(&i1) {
                for j in (i + 1)..items_usize.len() {
                    let i2 = *items_usize.get(j).unwrap();

                    if counts.contains_key(&i2) {
                        // If both are frequent
                        let value = ret.get_mut(&(i1, i2));
                        match value {
                            Some(val) => *val += 1, //If value is known, increment quantity.
                            None => {
                                ret.insert((i1, i2), 1);
                            } //Otherwise, initialize to 1.
                        }
                    }
                }
            }
        }
    }

    ret.retain(|_, &mut v| v >= minimum_support); // Only return pairs meeting threshold
    ret
}

fn get_verified_frequent_pairs(
    candidates: HashMap<(usize, usize), bool>,
    minimum_support: usize,
) -> HashMap<(usize, usize), usize> {
    let mut ret: HashMap<(usize, usize), usize> = HashMap::new();

    let f = File::open(DIRECTORY).unwrap();
    let reader = BufReader::new(f);

    let mut c = 0; //Debug
    for line in reader.lines() {
        /* c += 1; //Debug
        if (c % 4800 == 0) {println!("{}", c);} //Debug */
        let l = line.unwrap();
        let items: Vec<&str> = l.split_whitespace().collect();

        let mut items_usize: Vec<usize> = Vec::new();
        items
            .iter()
            .for_each(|f| items_usize.push(f.parse::<usize>().unwrap())); // strings interpreted as integers

        for i in 0..(items_usize.len() - 1) {
            for j in (i + 1)..items_usize.len() {
                let i1 = *items_usize.get(i).unwrap();
                let i2 = *items_usize.get(j).unwrap();
                if (candidates.contains_key(&(i1, i2))){ // If this pair appeared in at least one subset
                    let value = ret.get_mut(&(i1, i2));
                    match value {
                        Some(val) => *val += 1, //If value is known, increment quantity.
                        None => {
                            ret.insert((i1, i2), 1);
                        } //Otherwise, initialize to 1.
                    }
                }

            }
        }
    }

    ret.retain(|_, &mut v| v >= minimum_support); // Only return pairs meeting threshold
    ret
}
