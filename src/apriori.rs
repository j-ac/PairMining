use core::num;
use std::collections::HashMap;
use std::fs::File;
use std::hash::Hash;
use std::io::prelude::*;
use std::io::BufReader;
use std::ops::Add;
use std::path::Path;
use std::time::Instant;

const THRESHOLD: f64 = 0.01;
const DIRECTORY: &str = "C:\\Users\\xxmem\\Desktop\\school\\4\\Big Data Systems\\A1\\retail.dat";

pub fn run() -> std::io::Result<()> {
    let start = Instant::now();

    let (frequent_items, minimum_support) = get_item_counts();
    let frequent_pairs = get_frequent_pairs(frequent_items, minimum_support);

    for i in frequent_pairs.iter() {
        println!("{:?}", i.0);
    }

    println!("Runtime: {:.2?}", start.elapsed());
    Ok(())
}

// Performs pass one
fn get_item_counts() -> (HashMap<usize, usize>, usize) {
    let f = File::open(DIRECTORY).unwrap();
    let reader = BufReader::new(f);

    let mut counts = HashMap::new();
    let mut num_baskets: usize = 0;
    for line in reader.lines() {
        num_baskets += 1;
        for item in line.unwrap().split_whitespace() {
            let item_as_usize = item.parse::<usize>().unwrap(); // string -> int

            match counts.get_mut(&item_as_usize) {
                Some(value) => *value += 1, //Use of pointer greatly reduces insertion cost
                None => {counts.insert(item_as_usize, 1);} //initialize to 1
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

    for line in reader.lines() {
        let l = line.unwrap();
        let items: Vec<&str> = l.split_whitespace().collect();

        let mut items_usize: Vec<usize> = Vec::new();
        items
            .iter()
            .for_each(|f| items_usize.push(f.parse::<usize>().unwrap())); // strings interpreted as integers

        for i in 0..items_usize.len() {
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
                    };
                }
            }
        }
    }

    ret.retain(|_, &mut v| v >= minimum_support); // Only return pairs meeting threshold
    ret
}
