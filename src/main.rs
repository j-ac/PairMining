mod apriori;
mod pcy;
mod multihashpcy;
mod sample;
fn main(){
    let algo: usize = 3;

    match algo {
        0 => {apriori::run();}
        1 => {pcy::run();}
        2 => {multihashpcy::run();}
        _ => {sample::run();}

    }
}