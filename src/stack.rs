

pub mod stack {
    use my_math::num_complexe::num_complexe::NumComplexe;

    #[derive(Clone)]
    pub struct Stack {
        variable: Vec<NumComplexe>,
    }

    impl Stack {
        pub fn new() -> Stack {
            return Stack {
                variable: (Vec::new())
            };;
        }
        pub fn add_variable(&mut self, var: NumComplexe) {
            self.variable.push(var);
            println!("{}", var);
        }

        pub fn print_variable(self) {
            println!("Printing all variable :");
            for var in self.variable {
                println!("{}", var);
            }
        }
        pub fn print_all(self) {
            println!("Print all the stack :");
            self.print_variable();
        }
    }
}