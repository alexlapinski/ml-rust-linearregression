extern crate ndarray;
extern crate num_traits;


use ndarray::Array;
use ndarray::arr2;
use ndarray::Ix2;

use num_traits::Float;

fn standardize<A: Float>(data: Array<A, Ix2>) -> Array<A, Ix2> {
    return data;
}

fn main() {
    println!("Linear Regression Test Bench");
    let arr = arr2(&[[1., 2.], [3., 4.]]);

    println!("\n");
    println!("Data: {}", arr);

    let std_arr = standardize(arr);
    println!("Standardized Data: {}", std_arr)
}
