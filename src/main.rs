use std::env;

mod apriori;
mod pcy;
mod multihashpcy;
mod sample;
mod son;
fn main(){
    let algo: usize = env::var("ALGO").unwrap().parse::<usize>().unwrap();
    let THRESHOLD: f64 = env::var("THRESHOLD").unwrap().parse::<f64>().unwrap();

    match algo {
        0 => {apriori::run(THRESHOLD);}
        1 => {pcy::run(THRESHOLD);}
        2 => {multihashpcy::run(THRESHOLD);}
        3 => {sample::run(THRESHOLD);}
        _ => {son::run(THRESHOLD);}

    }
}