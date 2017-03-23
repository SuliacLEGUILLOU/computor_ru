
struct NumComplex {
    real: f32,
    imag: f32,
}

impl NumComplex {
    fn abs(&self) -> NumComplex {
        let mut ret: NumComplex;

        if self.imag == 0.0 {
            ret.real = if self.real < 0.0 { self.real * -1.0} else { self.real };
            ret.imag = 0;
        } else {
            ret = self.modulus();
        }
        return ret;
    }
    fn absolute_square(&self) -> NumComplex {
        return self;
    }
    fn square(&self) -> NumComplex {
        if self.imag != 0.0 {
            return self.absolut_square();
        }
        let mut ret: NumComplex;
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
    fn modulus(&self) -> f32 {
        let mut tmp: NumComplex;

        tmp.real = self.real * self.real + self.imag *self.imag;
        tmp.imag = 0.0;
        tmp = tmp.square();
        return tmp.real;
    }
}

impl Add for NumComplex {
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

    fn add_assign(&self, other: NumComplex) -> NumComplex {
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

    fn sub_assign(&self, other: NumComplex) -> NumComplex {
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

    fn mul_assign(&self, other: NumComplex) -> NumComplex {
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

    fn div(&self, other: NumComplex) -> NumComplex {
        self.real = (self.real * other.real + self.imag * other.imag) / (other.real * other.real + other.imag * other.imag);
        self.imag = (other.real * self.imag - self.real * other.imag) / (other.real * other.real + other.imag * other.imag);
        return self;
    }
}