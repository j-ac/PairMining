mod apriori;
mod pcy;
mod multihashpcy;

fn main(){
    let algo: usize = 2;

    match algo {
        0 => {apriori::run();}
        1 => {pcy::run();}
        _ => {multihashpcy::run();}

    }
}