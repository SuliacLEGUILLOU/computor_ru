
pub mod array {
    use my_math::num_complexe::num_complexe::NumComplexe;
    use std::ops;
    use std::fmt;

    //#[derive(Clone, Copy)]    // I NEED THIS !
    pub struct Array{
        data: [[NumComplexe; 50]; 50],
        width: usize,
        height: usize,
        complexe_count: u32,
    }
    impl Array {
        pub fn new() -> Array {
            return Array {
                data: [[NumComplexe::new(0.0, 0.0); 50]; 50],
                width: 0,
                height:0,
                complexe_count: 0
            };
        }
        pub fn new_sized(large: usize, heigth:usize) -> Array{

            return Array {
                data: [[NumComplexe::new(0.0, 0.0); 50]; 50],
                width: large,
                height: heigth,
                complexe_count: 0
            };
        }
        pub fn new_filled(new_width: usize, new_height: usize, base: NumComplexe) -> Array{
            let count: usize;
            if base.get_imag() == 0.0 {
                count = 0;
            } else {
                count = new_height * new_width;
            }

            return Array {
                data: [[base; 50]; 50],
                width: new_width,
                height: new_height,
                complexe_count: count as u32,
            };
        }

        pub fn get_complexe_count(&self) -> u32 {
            return self.complexe_count;
        }

        pub fn inverse(&self) {}

    }

    impl ops::Add for Array {
        type Output = Array;

        fn add(self, other: Array) -> Array {
            let mut ret: Array;

            if self.height == other.height && self.width == other.width {
                ret = Array::new_sized(self.height, self.width);

                for i in 0..self.width {
                    for j in 0..self.height {
                        ret.data[i][j] = self.data[i][j] + other.data[i][j];
                    }
                }
            } else {
                panic!("Arrays:add does not have the same size !");
            }
            return ret;
        }
    }

    impl fmt::Display for Array {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            for i in 0..self.width {
                for j in 0..self.height {
                    if j == 0 {
                        write!(f, "\n{}", self.data[i][j]);
                    } else {
                        write!(f, " {}", self.data[i][j]);
                    }
                }
            }
            write!(f, "\n")
        }
    }
}