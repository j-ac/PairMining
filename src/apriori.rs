use core::num;
use std::collections::HashMap;
use std::fs::File;
use std::hash::Hash;
use std::io::prelude::*;
use std::io::BufReader;
use std::ops::Add;
use std::ops::RemAssign;
use std::path::Path;
use std::time::Instant;

use crate::triangle_matrix;
use crate::triangle_matrix::TriangleMatrix;

const THRESHOLD: f64 = 0.01;
const DIRECTORY: &str = "C:\\Users\\xxmem\\Desktop\\school\\4\\Big Data Systems\\A1\\retail.dat";

pub fn run() -> std::io::Result<()> {
    let start = Instant::now();

    let (frequent_items, old_ids, minimum_support) = get_item_counts();
    let frequent_pairs_matrix = get_frequent_pairs_matrix(frequent_items);

    for i in 0..frequent_pairs_matrix.array.len(){
        let column = frequent_pairs_matrix.array.get(i).unwrap();
            for j in 0 .. column.len(){
                if frequent_pairs_matrix[(i,j)] > minimum_support{
                    print!("{},{}", old_ids.get(&i).unwrap(), old_ids.get(&j).unwrap());
                }
            }
    }

    Ok(())
}

// Performs pass one
fn get_item_counts() -> (HashMap<usize, usize>, HashMap<usize, usize>, usize) {
    let f = File::open(DIRECTORY).unwrap();
    let reader = BufReader::new(f);

    let mut counts = HashMap::new();
    let mut num_baskets: usize = 0;
    for line in reader.lines() {
        num_baskets += 1;
        for item in line.unwrap().split_whitespace() {
            let item_as_usize = item.parse::<usize>().unwrap(); // string -> int

            match counts.get(&item_as_usize) {
                Some(value) => {counts.insert(item_as_usize, value + 1);}, //Use of pointer greatly reduces insertion cost
                None => {counts.insert(item_as_usize, 1);} //initialize to 1
            }
        }
    }

    let minimum_support = (num_baskets as f64 * THRESHOLD).ceil() as usize;
    counts.retain(|_, &mut v| v >= minimum_support); // Keep everything meeting the threshold
    let (frequents, old_ids) = remap_ids(&counts);
    //print!("{:?}", counts);
    (frequents, old_ids, minimum_support)
    
}

// Associate integer values 0....n with IDs in the frequent item set
// and return a mapping between the new ids and the old ids.
// This way the elements can go into a matrix and the position of the matrix can determine the original ID.
pub fn remap_ids( frequent_items: &HashMap<usize, usize>) -> (HashMap<usize, usize>, HashMap<usize, usize>){
    let mut original_ids = HashMap::new(); // Relates 0..n values to their original ID value from the dataset.
    let mut frequents = HashMap::new(); // counts array mapping 0...n to counts

    let mut new_id: usize = 0;
    for (old_id, _) in frequent_items.iter(){
        original_ids.insert(new_id, *old_id);
        frequents.insert(*old_id, new_id);
        new_id += 1;
    }

    (frequents, original_ids)
}

fn get_frequent_pairs_matrix(
    frequents: HashMap<usize, usize>, //old ids -> newids
) -> triangle_matrix::TriangleMatrix {
    let mut ret: TriangleMatrix = TriangleMatrix::new(frequents.len());

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
            if frequents.contains_key(&i1) { //If first was frequent
                for j in (i + 1)..items_usize.len() {
                    let i2 = *items_usize.get(j).unwrap();
nah 
                    if frequents.contains_key(&i2) { // and second was frequent
                        // If both are frequent
                        ret.increment(*frequents.get(&i1).unwrap(), *frequents.get(&i2).unwrap());
                    };
                }
            }
        }
    }



    ret
}
