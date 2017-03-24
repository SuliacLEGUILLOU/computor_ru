
use std::ops;
use std::fmt;

pub mod num_complexe {
    pub struct NumComplexe {
        real: f32,
        imag: f32,
    }

    impl NumComplexe {
        pub fn new(r: f32, i:f32) -> NumComplexe {
            let mut ret = NumComplexe {real: 0.0, imag: 0.0};

            ret.real = r;
            ret.imag = i;
            return ret;
        }
        pub fn abs(&self) -> NumComplexe {
            let mut ret = NumComplexe {real: 0.0, imag: 0.0};

            if self.imag == 0.0 {
                ret.real = if self.real < 0.0 { self.real * -1.0 } else { self.real };
            }
            return ret;
        }
        pub fn square(&self) -> NumComplexe {
            let mut ret = NumComplexe {real: 0.0, imag: 0.0};
            let mut xn: f32 = 0.0;
            let mut xn1: f32 = self.real / 2.0;

            while xn <= xn1 - 0.0001 || xn >= xn1 + 0.0001 {
                xn = xn1;
                xn1 = (xn + self.real / xn) / 2.0;
            }
            ret.imag = 0.0;
            ret.real = xn;
            return ret;
        }
        pub fn modulus(&self) -> f32 {
            let mut tmp = NumComplexe {real: 0.0, imag: 0.0};

            tmp.real = self.real * self.real + self.imag * self.imag;
            tmp.imag = 0.0;
            tmp = tmp.square();
            return tmp.real;
        }
    }

    impl fmt::Display for NumComplexe {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            if self.imag == 0.0 {
                write!(f, "{}", self.real);
            } else if self.real == 0.0 {
                write!(f, "{}i", self.imag);
            } else {
                write!(f, "{} + {}i", self.real, self.imag);
            }
        }
    }

/*    impl Add for NumComplex {
        type Output = NumComplex;

        fn add(&self, other: NumComplex) -> NumComplex {
            let mut ret: NumComplex;

            ret.real = self.real + other.real;
            ret.imag = self.imag + other.imag;
            return ret;
        }
    }

    impl AddAssign for NumComplex {
        type Output = NumComplex;

        fn add_assign(&mut self, other: NumComplex) -> NumComplex {
            self.real += other.real;
            self.imag += other.imag;
            return self;
        }
    }

    impl Sub for NumComplex {
        type Output = NumComplex;

        fn sub(&self, other: NumComplex) -> NumComplex {
            let mut ret: NumComplex;

            ret.real = self.real - other.real;
            ret.imag = self.imag - other.imag;
            return ret;
        }
    }

    impl SubAssign for NumComplex {
        type Output = NumComplex;

        fn sub_assign(&mut self, other: NumComplex) -> NumComplex {
            self.real -= other.real;
            self.imag -= other.imag;
            return self;
        }
    }

    impl Mul for NumComplex {
        type Output = NumComplex;

        fn mul(&self, other: NumComplex) -> NumComplex {
            let mut ret: NumComplex;

            ret.real = self.real * other.real - self.imag * other.imag;
            ret.imag = self.imag * other.real + other.real * self.imag;
            return ret;
        }
    }

    impl MulAssign for NumComplex {
        type Output = NumComplex;

        fn mul_assign(&mut self, other: NumComplex) -> NumComplex {
            self.real = self.real * other.real - self.imag * other.imag;
            self.imag = self.imag * other.real + other.real * self.imag;
            return self;
        }
    }

    impl Div for NumComplex {
        type Output = NumComplex;

        fn div(&self, other: NumComplex) -> NumComplex {
            let mut ret: NumComplex;

            ret.real = (self.real * other.real + self.imag * other.imag) / (other.real * other.real + other.imag * other.imag);
            ret.imag = (other.real * self.imag - self.real * other.imag) / (other.real * other.real + other.imag * other.imag);
            return ret;
        }
    }

    impl DivAssign for NumComplex {
        type Output = NumComplex;

        fn div(&mut self, other: NumComplex) -> NumComplex {
            self.real = (self.real * other.real + self.imag * other.imag) / (other.real * other.real + other.imag * other.imag);
            self.imag = (other.real * self.imag - self.real * other.imag) / (other.real * other.real + other.imag * other.imag);
            return self;
        }
    }
    */
}