use core::num;
use std::collections::HashMap;
use std::fs::File;
use std::hash::Hash;
use std::io::prelude::*;
use std::io::BufReader;
use std::ops::Add;
use std::path::Path;
use std::time::Instant;

// Tweak these parameters to keep memory within safe range, while still having a large number of buckets
pub const PCY_BUCKET_BASE: usize = 7; // MAX 16
pub const PCY_BUCKET_EXPONENT: usize = 7;

const THRESHOLD: f64 = 0.01;
const DIRECTORY: &str = "C:\\Users\\xxmem\\Desktop\\school\\4\\Big Data Systems\\A1\\retail.dat";

pub fn run() -> std::io::Result<()> {
    let start = Instant::now();

    let (frequent_items, minimum_support) = get_item_counts();
    let frequent_pairs = get_frequent_pairs(frequent_items, minimum_support);

    let mut out = std::fs::File::create(format!("{}{}", DIRECTORY, ".out")).unwrap();
    for i in frequent_pairs.iter() {
        write!(out, "{:?}\n", i.0);
    }

    println!("\nRuntime: {:.2?}", start.elapsed());
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

// Calculates prime factors incorrectly (on purpose!)
// Get the prime factorization of each number, only using prime numbers between 0 and base, and capping each prime factor's exponent at exp
// Then sums the degrees together
// These limitations make collisions a possibility when hashed, as many numbers may have the same "bad prime factors"
// together, the number of buckets will equal base^exp. Tweak these constants to get a number of buckets which seems appropriate
pub fn bad_prime_factors(nums: (usize, usize), base: usize, exp: usize) -> Vec<usize> {
    let primes: Vec<usize> = vec![2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37, 41, 43, 47, 53];
    let mut result: Vec<usize> = vec![0; exp];

    let mut n1 = nums.0;
    let mut n2 = nums.1;
    for i in 0..base {
        while (n1 % primes[i] == 0) {
            n1 = n1 / primes[i];
            if (i < result.len() && result[i] < base) {
                result[i] += 1;
            }
        }
    }

    for j in 0..base {
        while (n2 % primes[j] == 0) {
            n2 = n2 / primes[j];
            if (j < result.len() && result[j] < base) {
                result[j] += 1;
            }
        }
    }

    result
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

#[cfg(test)]
mod tests {
    use super::*;
     #[test]
    fn test_bad_prime_1() {
        assert_eq!(bad_prime_factors((30, 45), 7, 7), vec![1, 3, 2, 0, 0, 0, 0]);
        
    }

   #[test]
    fn test_bad_prime_2(){
        assert_eq!(bad_prime_factors((30, 45), 7, 2), vec![1, 3]);
    } 
  #[test]
     fn test_bad_prime_3(){
        assert_eq!(bad_prime_factors((30, 45), 2, 3), vec![1, 2, 2]);
    } 
}
