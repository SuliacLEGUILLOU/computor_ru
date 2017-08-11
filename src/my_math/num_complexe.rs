
/// This sub module help represent complexe number
pub mod num_complexe {
    use std::ops;
    use std::fmt;

    #[derive(Clone, Copy)]
    pub struct NumComplexe {
        /// Complexe number are represented as R + I * i
        real: f32,
        imag: f32,
    }

    impl NumComplexe {
        pub fn new(r: f32, i:f32) -> NumComplexe {
            return NumComplexe {real: r, imag: i};
        }
        ///
        /// ## abs function return the absolute value of the number
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
        pub fn get_real(&self) -> f32 {
            return self.real;
        }
        pub fn get_imag(&self) -> f32 {
            return self.imag;
        }
    }

    impl fmt::Display for NumComplexe {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            if self.imag == 0.0 {
                write!(f, "{}", self.real)
            } else if self.real == 0.0 {
                write!(f, "{}i", self.imag)
            } else {
                write!(f, "{} + {}i", self.real, self.imag)
            }
        }
    }

    impl ops::Add for NumComplexe {
        type Output = NumComplexe;

        fn add(self, other: NumComplexe) -> NumComplexe {
            let mut ret = NumComplexe {real: 0.0, imag: 0.0};

            ret.real = self.real + other.real;
            ret.imag = self.imag + other.imag;
            return ret;
        }
    }
    impl ops::AddAssign for NumComplexe {
        fn add_assign(&mut self, other: NumComplexe) {
            self.real += other.real;
            self.imag += other.imag;
        }
    }
    impl ops::Sub for NumComplexe {
        type Output = NumComplexe;

        fn sub(self, other: NumComplexe) -> NumComplexe {
            let mut ret = NumComplexe {real: 0.0, imag: 0.0};

            ret.real = self.real - other.real;
            ret.imag = self.imag - other.imag;
            return ret;
        }
    }
    impl ops::SubAssign for NumComplexe {
        fn sub_assign(&mut self, other: NumComplexe) {
            self.real -= other.real;
            self.imag -= other.imag;
        }
    }

    impl ops::Mul for NumComplexe {
        type Output = NumComplexe;

        fn mul(self, other: NumComplexe) -> NumComplexe {
            let mut ret = NumComplexe {real: 0.0, imag: 0.0};

            ret.real = self.real * other.real - self.imag * other.imag;
            ret.imag = self.imag * other.real + other.real * self.imag;
            return ret;
        }
    }
    impl ops::MulAssign for NumComplexe {
        fn mul_assign(&mut self, other: NumComplexe) {
            self.real = self.real * other.real - self.imag * other.imag;
            self.imag = self.imag * other.real + other.real * self.imag;
        }
    }

    impl ops::Div for NumComplexe {
    type Output = NumComplexe;

    fn div(self, other: NumComplexe) -> NumComplexe {
        let mut ret = NumComplexe {real: 0.0, imag: 0.0};

        ret.real = (self.real * other.real + self.imag * other.imag) / (other.real * other.real + other.imag * other.imag);
        ret.imag = (other.real * self.imag - self.real * other.imag) / (other.real * other.real + other.imag * other.imag);
        return ret;
    }
}
    impl ops::DivAssign for NumComplexe {
        fn div_assign(&mut self, other: NumComplexe) {
            self.real = (self.real * other.real + self.imag * other.imag) / (other.real * other.real + other.imag * other.imag);
            self.imag = (other.real * self.imag - self.real * other.imag) / (other.real * other.real + other.imag * other.imag);
        }
    }

    impl ops::Rem for NumComplexe {
        type Output = NumComplexe;

        fn rem(self, other: NumComplexe) -> NumComplexe {
            let mut ret = NumComplexe {real: 0.0, imag: 0.0};

            ret.real = (self.real * other.real + self.imag * other.imag) % (other.real * other.real + other.imag * other.imag);
            ret.imag = (other.real * self.imag - self.real * other.imag) % (other.real * other.real + other.imag * other.imag);
            return ret;
        }
    }
    impl ops::RemAssign for NumComplexe {
        fn rem_assign(&mut self, other: NumComplexe) {
            self.real = (self.real * other.real + self.imag * other.imag) % (other.real * other.real + other.imag * other.imag);
            self.imag = (other.real * self.imag - self.real * other.imag) % (other.real * other.real + other.imag * other.imag);
        }
    }
}

#[cfg(test)]
mod tests_num_complexe {
    use NumComplexe;

    #[test]
    fn assignation() {
        let test = NumComplexe::new(5.0, -0.1);

        assert!(test.get_real() == 5.0);
        assert!(test.get_imag() == -0.1);
    }

    #[test]
    fn addition() {
        let test = NumComplexe::new(5.0, -0.1);
        let test1 = NumComplexe::new(6.0, 90.0);

        let result = test + test1;
        assert!(result.get_real() == (5.0 + 6.0));
        assert!(result.get_imag() == (-0.1 + 90.0));
    }

    #[test]
    fn substraction() {
        let test = NumComplexe::new(5.0, -0.1);
        let test1 = NumComplexe::new(6.0, 90.0);

        let result = test - test1;
        assert!(result.get_real() == (5.0 - 6.0));
        assert!(result.get_imag() == (-0.1 - 90.0));
    }

    #[test]
    fn multiplication() {
        let test = NumComplexe::new(5.0, -0.1);
        let test1 = NumComplexe::new(6.0, 90.0);

        let result = test * test1;
        assert!(result.get_real() == 39.0);
        assert!(result.get_imag() == -1.2);
    }

    #[test]
    fn division() {
        let test = NumComplexe::new(5.0, -0.1);
        let test1 = NumComplexe::new(6.0, 90.0);

        let result = test / test1;
        assert!(result.get_real() == 0.002581121);
        assert!(result.get_imag() == -0.05538348);
    }
}