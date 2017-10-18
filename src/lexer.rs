
pub mod lexer {
    pub enum LineType {
        Variable,
        Function,
        Array
    }

    pub struct LineInfo {
        pub line_type: LineType,
        pub name: String,
        pub expression: String,
    }

    pub fn check_input(input: &str) -> bool {
        return true;
    }
    pub fn clean_input(input: &str) -> String {
        let mut ret = input.replace(" ", "");
        return ret;
    }

    pub fn get_expression_type(input: &str) -> LineType {
        return LineType::Variable;
    }

    pub fn get_info(input: &str) -> LineInfo {
        let mut ret = LineInfo {
            line_type: LineType::Variable,
            name: "".to_string(), 
            expression: "".to_string()
        };


        if check_input(input) {
            let info:Vec<&str> = input.split('=').collect();
            ret.expression = String::from(info[1]);
            ret.name = String::from(info[0]);
        } else {
            panic!("Error");
        }
        return ret;
    }
}

#[cfg(test)]
mod test_lexer {
    use my_math::num_complexe::num_complexe::NumComplexe;
    use lexer::*;

    #[test]
    fn test_check_input_var_nospace() {
        let info: lexer::LineInfo = lexer::get_info("a=3-4.3i");

        //assert!(info.lineType == lexer::LineType::Variable);
        assert!(info.name == String::from("a"));
        assert!(info.expression == String::from("3-4.3i"));
    }

    #[test]
    fn test_check_input_cleaner() {
        let s = "   hello world      lol...";

        assert_eq!(lexer::clean_input(s).as_str(), "helloworldlol...");
    }
}