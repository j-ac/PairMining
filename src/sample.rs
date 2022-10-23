use core::num;
use rand::prelude::*;
use std::collections::HashMap;
use std::fs::File;
use std::hash::Hash;
use std::io::prelude::*;
use std::io::BufReader;
use std::ops::Add;
use std::path::Path;
use std::time::Instant;
use std::env;

const SAMPLE_SIZE: f64 = 0.01; // fraction of lines parsed
const DIRECTORY: &str = "./netflix.data";
const SEED: [u8;32] = [0 as u8; 32]; // Use the same seed in pass one and pass 2 so the same lines are read.

pub fn run(THRESHOLD: f64) -> std::io::Result<()> {
    let start = Instant::now();

    let (frequent_items, minimum_support_sample, minimum_support_actual) = get_item_counts(THRESHOLD);
    let mut is_using_sample: bool = true;
    let frequent_pairs = get_frequent_pairs(&frequent_items, minimum_support_sample, is_using_sample);

    // Second pass gets real totals to compare against our sample
    is_using_sample = false;
    let no_sample_frequent_pairs = get_frequent_pairs(&frequent_items, minimum_support_actual, is_using_sample);

    // If a pair found with the sample did not appear in the actual 
    let mut false_positives = Vec::new();
    for (key, _) in frequent_pairs.iter(){
        if !no_sample_frequent_pairs.contains_key(&key){
            false_positives.push(key);
        }
    }

    print!("{} false positives: {:?}", false_positives.len(), false_positives);
    

    let mut out = std::fs::File::create(format!("sample{}{}{}", THRESHOLD, DIRECTORY, ".out")).unwrap();
    for i in frequent_pairs.iter() {
        write!(out, "{:?}\n", i.0);
    }

    println!("\nRuntime: {:.2?}", start.elapsed());
    Ok(())
}

// Performs pass one
fn get_item_counts(THRESHOLD: f64) -> (HashMap<usize, usize>, usize, usize) {
    let f = File::open(DIRECTORY).unwrap();
    let reader = BufReader::new(f);

    let mut counts = HashMap::new();
    let mut num_baskets: usize = 0;

    let mut rng = rand::rngs::StdRng::from_seed(SEED); // Random Sampling
    let mut num_lines = 0;
    for line in reader.lines() {
        num_lines += 1;
        let r: f64 = rng.gen();
        if (r < SAMPLE_SIZE) { // Random Sampling
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
    }

    let minimum_support_sample = (num_baskets as f64 * THRESHOLD).ceil() as usize;
    let minimum_support_actual = (num_lines as f64 * THRESHOLD).ceil() as usize;
    counts.retain(|_, &mut v| v >= minimum_support_sample); // Keep everything meeting the threshold
    (counts, minimum_support_sample, minimum_support_actual)
}

fn get_frequent_pairs(
    counts: &HashMap<usize, usize>,
    minimum_support: usize,
    is_using_sample: bool,
) -> HashMap<(usize, usize), usize> {
    let mut ret: HashMap<(usize, usize), usize> = HashMap::new();

    let f = File::open(DIRECTORY).unwrap();
    let reader = BufReader::new(f);

    let mut c = 0; //Debug
    let mut rng = rand::rngs::StdRng::from_seed(SEED); // Random Sampling
    for line in reader.lines() {
        let r: f64 = rng.gen(); // random sampling
        /* c += 1; //Debug
        if (c % 4800 == 0) {println!("{}", c);} //Debug */
        if (!is_using_sample || r < SAMPLE_SIZE) {
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

    }

    ret.retain(|_, &mut v| v >= minimum_support); // Only return pairs meeting threshold
    ret
}
