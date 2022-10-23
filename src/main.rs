mod apriori;
mod pcy;

fn main(){
    let algo: usize = 0;

    match algo {
        0 => {apriori::run();}
        _ => {pcy::run();}
    }
}