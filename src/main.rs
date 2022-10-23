mod apriori;
mod pcy;
mod multihashpcy;
mod sample;
mod son;
fn main(){
    let algo: usize = 4;

    match algo {
        0 => {apriori::run();}
        1 => {pcy::run();}
        2 => {multihashpcy::run();}
        3 => {sample::run();}
        _ => {son::run();}

    }
}