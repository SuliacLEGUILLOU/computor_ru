
pub mod my_math;
pub mod message;

use my_math::num_complexe::num_complexe::NumComplexe;
use my_math::array::array::Array;
use message::information::*;

/*fn test_array() {
    let mut num = NumComplexe::new(3.0, 9.0);
    let mut test = Array::new();
    let mut test1 = Array::new_sized(5, 5);
    let mut test2 = Array::new_sized(1, 13);
    let mut test3 = Array::new_filled(5, 5, num);
    let mut test4 = Array::new_filled(5, 5, num);

    test = test3 + test4;
    println!("{}\n\n{}\n\n{}", test, test1, test4.get_complexe_count());
}*/

fn main() {
    let num = NumComplexe::new(3.0, 9.0);
    let arr = Array::new_sized(5, 5);

    welcome();
    println!("{}, {}", num, arr.get_complexe_count());
}
