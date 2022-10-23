use core::num;
use std::collections::hash_map::DefaultHasher;
use std::collections::HashMap;
use std::fs::File;
use std::hash::Hash;
use std::hash::Hasher;
use std::hash::SipHasher;
use std::io::prelude::*;
use std::io::BufReader;
use std::ops::Add;
use std::path::Path;
use std::time::Instant;

const THRESHOLD: f64 = 0.01;
const DIRECTORY: &str = "C:\\Users\\xxmem\\Desktop\\school\\4\\Big Data Systems\\A1\\retail.dat";

pub fn run() -> std::io::Result<()> {
    let start = Instant::now();

    let (frequent_items, minimum_support, pcy_buckets, pcy_buckets2) = get_item_counts();
    let frequent_pairs = get_frequent_pairs(frequent_items, minimum_support, pcy_buckets, pcy_buckets2);

    let mut out = std::fs::File::create(format!("{}{}", DIRECTORY, ".out")).unwrap();
    for i in frequent_pairs.iter() {
        write!(out, "{:?}\n", i.0);
    }

    println!("\nRuntime: {:.2?}", start.elapsed());
    Ok(())
}

// Performs pass one
fn get_item_counts() -> (
    HashMap<usize, usize>,
    usize,
    HashMap<u64, usize>,
    HashMap<u64, usize>,
) {
    let f = File::open(DIRECTORY).unwrap();
    let reader = BufReader::new(f);

    let mut counts = HashMap::new();
    let mut num_baskets: usize = 0;
    let mut pcy_buckets = HashMap::new(); // For stage 1 PCY
    let mut pcy_buckets2 = HashMap::new(); // For stage 2 PCY
    for line in reader.lines() {
        num_baskets += 1;

        let lines: Vec<usize> = line
            .unwrap()
            .split_whitespace()
            .map(|x| x.parse().unwrap())
            .collect();
        for (i, item) in lines.iter().enumerate() {
            match counts.get(item) {
                Some(value) => {
                    counts.insert(*item, value + 1);
                } //Use of pointer greatly reduces insertion cost
                None => {
                    counts.insert(*item, 1);
                } //initialize to 1
            }

            // === PCY === //
            // last iteration of i causes j+1 to be out of range. This won't run.
            for j in (i + 1)..lines.len() {
                // STAGE ONE HASH
                //Hash the tuple and then insert the hash as a key in the map. Thus tuples go into a single bucket indistinguishable
                let tuple = (*lines.get(i).unwrap(), *lines.get(j).unwrap());

                let mut hasher = DefaultHasher::new();
                tuple.hash(&mut hasher);
                let hash = hasher.finish();

                let frequency = pcy_buckets.get_mut(&hash);
                match frequency {
                    Some(f) => *f += 1,
                    None => {
                        pcy_buckets.insert(hash, 1);
                    } 
                }

                // STAGE 2 HASH
                // identical to stage one, except using a different Hashing algorithm.
                let mut hasher2 = SipHasher::new();
                tuple.hash(&mut hasher2);
                let hash2 = hasher2.finish();

                let frequency2 = pcy_buckets2.get_mut(&hash);
                match frequency2 {
                    Some(f) => *f += 1,
                    None => {
                        pcy_buckets2.insert(hash2, 1);
                    }
                }
            }

            //println!("{:?}", pcy_buckets);
        }
    }

    let minimum_support = (num_baskets as f64 * THRESHOLD).ceil() as usize;
    counts.retain(|_, &mut v| v >= minimum_support); // Keep everything meeting the threshold
    pcy_buckets.retain(|_, &mut v| v >= minimum_support); // Keep everything meeting the threshold. same as above.
    (counts, minimum_support, pcy_buckets, pcy_buckets2)
}

fn get_frequent_pairs(
    counts: HashMap<usize, usize>,
    minimum_support: usize,
    pcy_buckets: HashMap<u64, usize>,
    pcy_buckets2: HashMap<u64, usize>,
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
                        // Now checking if this pair was frequent in BOTH hashmaps from PCY.

                        //PCY stage one
                        let mut hasher = DefaultHasher::new();
                        (i1, i2).hash(&mut hasher);
                        let hash = hasher.finish();
                        if pcy_buckets.get(&hash).unwrap_or(&0) > &minimum_support {
                            let value = ret.get_mut(&(i1, i2)); //Retrieve current value, if it exists.
                            match value {
                                Some(val) => *val += 1, //If value is known, increment quantity.
                                None => {
                                    ret.insert((i1, i2), 1);
                                } //Otherwise, initialize to 1.
                            }
                        }

                        // PCY stage 2
                        let mut hasher2 = SipHasher::new();
                        (i1, i2).hash(&mut hasher2);
                        let hash2 = hasher2.finish();
                        if pcy_buckets2.get(&hash2).unwrap_or(&0) > &minimum_support {
                            let value = ret.get_mut(&(i1, i2)); //Retrieve current value, if it exists.
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

#[cfg(test)]
mod tests {
    #[test]
    pub fn hasher_test() {
        use std::collections::hash_map::DefaultHasher;
        use std::hash::Hash;
        use std::hash::Hasher;

        let mut hasher = DefaultHasher::new();
        1234.hash(&mut hasher);
        let mut hasher2 = DefaultHasher::new();
        1234.hash(&mut hasher2);

        assert_eq!(hasher.finish(), hasher2.finish());
    }
}
