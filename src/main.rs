mod linear_search;
mod utils;

use linear_search::{
    search,
    SearchEnum::{
        Found,
        NotFound,
    },
};
use utils::{
    get_i32,
};

fn main() {
    let values: Vec<usize> = vec![
        10, 15, 12, 5, 99, 87, 71, 41, 22, 29,
        3, 31, 84, 52, 63, 81, 58, 92, 49, 77,
    ];
    loop {
        let search_for: usize = get_i32("Type number to search for: ");
        match search(&values, search_for) {
            Found(iterations) => {
                println!("{search_for} found in {:?} after {iterations} iterations", values);
            },
            NotFound(iterations) => {
                println!("{search_for} not found in {:?} after {iterations} iterations", values);
            },
        }
    }
}
