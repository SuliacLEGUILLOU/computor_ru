
pub mod my_math;

use my_math::num_complexe::num_complexe::NumComplexe;

fn main() {
    let mut test = NumComplexe::new(5.0, -0.1);
    let mut test1 = NumComplexe::new(6.0, 90.0);
    let mut test2 = NumComplexe::new(3.0, 9.0);

    test1 = test * test1;
    test1 -= test;
    test /= test;
    test2 += test1 + test + test2;
    println!("{}\n{}\n{}", test, test1, test2);
}
