mod color;
mod math;

use crate::math::tuple::Tuple;

fn main() {
    println!("Hello, world!");

    let x = Tuple::new_vector(1., 2., 3.);

    println!("{:?}", x);
}
