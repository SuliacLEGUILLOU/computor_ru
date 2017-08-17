
pub mod lexer {
    pub enum LineType {
        Variable,
        Function,
        Array
    }

    pub fn check_input(input: &str) -> &str {
        return "Not implemented";
    }

    pub fn get_expression_type(input: &str) -> LineType {
        return LineType::Variable;
    }
}