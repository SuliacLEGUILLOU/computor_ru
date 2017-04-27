
use my_math::num_complexe::num_complexe::NumComplexe;

#[test]
fn it_works() {
    let mut test = NumComplexe::new(5.0, -0.1);

    assert_eq!(5.0, test.real);
    assert_eq!(-0.1, test.imag);
}