
/**
 *  Turn this into a object to avoid regex compilation ?
 */

pub mod parser {
    use my_math::num_complexe::num_complexe::NumComplexe;
    use regex::Regex;

    pub fn parse_cmd(input: &str) -> &str {
        return "Not implemented";
    }

    /**
     *  parse_var
     *  String input : cleaned user detected as a variable by the lexer without the name
     *  return : NumComplexe value of the input
     */
    pub fn parse_var(input: &str) -> NumComplexe {
        let find_real = Regex::new(r"(-?\d+(?:\.\d+)?)[^i]?").unwrap();
        let find_imag = Regex::new(r"(-?\d+(?:\.\d+)?)i").unwrap();

        let real:f32 = match find_real.captures(input) {
            Some(cap) => cap[1].parse().unwrap(),
            None => 0.0
        };
        let img:f32 = match find_imag.captures(input) {
            Some(cap) => cap[1].parse().unwrap(),
            None => 0.0
        };

        return NumComplexe::new(real, img);
    }
}

#[cfg(test)]
mod test_parser {
    use my_math::num_complexe::num_complexe::NumComplexe;
    use parser::parser::*;

    #[test]
    fn test_pos_real_integer() {
        assert!(parse_var("90").get_real() == 90.0);
        assert!(parse_var("+90").get_real() == 90.0);

    }
    #[test]
    fn test_neg_real_integer() {
        assert!(parse_var("-90").get_real() == -90.0);

    }
    #[test]
    fn test_pos_complex_integer() {
        let num1 = parse_var("90+11i");
        let num2 = parse_var("+90+11i");

        assert!(num1.get_real() == 90.0);
        assert!(num2.get_real() == 90.0);
        assert!(num1.get_imag() == 11.0);
    }
    #[test]
    fn test_neg_complex_integer() {
        let num1 = parse_var("-90-11i");
        let num2 = parse_var("90-11i");

        assert!(num1.get_real() == -90.0);
        assert!(num1.get_imag() == -11.0);
        assert!(num2.get_real() == 90.0);
        assert!(num2.get_imag() == -11.0);
    }
    #[test]
    fn test_pos_real_float() {
        assert!(parse_var("90.5").get_real() == 90.5);
    }
    fn test_pos_complex_float() {
        let num1 = parse_var("90.5+11.12i");
        let num2 = parse_var("90.5-11.12i");

        assert!(num1.get_real() == 90.5);
        assert!(num2.get_imag() == -11.12);
        assert!(num1.get_imag() == 11.12);
    }
    #[test]
    fn test_reverse_notation() {
        let num1 = parse_var("11i+90");
        let num2 = parse_var("+11i+90");
        let num3 = parse_var("-11i+90");
        let num4 = parse_var("-11i-90");

        assert!(num1.get_real() == 90.0);
        assert!(num2.get_real() == 90.0);
        assert!(num1.get_imag() == 11.0);
        assert!(num3.get_real() == 90.0);
        assert!(num4.get_real() == -90.0);
        assert!(num3.get_imag() == -11.0);
    }
}