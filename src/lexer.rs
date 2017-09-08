
pub mod lexer {
    pub enum LineType {
        Variable,
        Function,
        Array,
        none
    }

    pub struct LineInfo {
        pub lineType: LineType,
        pub name: String,
        pub expression: String,
    }

    pub fn check_input(input: &str) -> bool {
        return true;
    }

    pub fn get_expression_type(input: &str) -> LineType {
        return LineType::Variable;
    }

    pub fn get_info(input: &str) -> LineInfo {
        let mut ret = LineInfo {
            lineType: LineType::Variable,
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