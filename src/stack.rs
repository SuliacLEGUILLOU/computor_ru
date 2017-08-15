
pub mod stack {
    use std::collections::HashMap;
    use my_math::num_complexe::num_complexe::NumComplexe;

    #[derive(Clone)]
    pub struct Stack {
        variable: HashMap<String, NumComplexe>,
    }

    impl Stack {
        pub fn new() -> Stack {
            return Stack {
                variable: HashMap::new()
            };
        }
        pub fn add_var(&mut self, id: String, val: NumComplexe) {
            self.variable.insert(id, val);
        }
        pub fn get_var(&self, id: String) -> Option<&NumComplexe> {
            return self.variable.get(&id);
        }

        pub fn print_variable(&self) {
            println!("Printing all variable :");
            for (name, var) in &(self.variable) {
                println!("{} = {}", name, var);
            }
        }
        pub fn print_all(&self) {
            println!("Print all the stack :");
            self.print_variable();
        }
    }
}

#[cfg(test)]
mod test_stack {
    use Stack;
    use my_math::num_complexe::num_complexe::NumComplexe;

    #[test]
    fn test_get_var() {
        let mut memory: Stack = Stack::new();
        let num_complexe = NumComplexe::new(1.0, 1.0);

        memory.add_var(String::from("success"), num_complexe);

        match memory.get_var(String::from("success")) {
            Some(_) => assert!(true),
            None => assert!(false)
        }
        match memory.get_var(String::from("fail")) {
            Some(_) => assert!(false),
            None => assert!(true)
        }
    }

    #[test]
    fn test_add_var() {
        let mut memory: Stack = Stack::new();
        let num_complexe1 = NumComplexe::new(1.0, 1.0);
        let num_complexe2 = NumComplexe::new(1.0, 2.0);
        let num_complexe3 = NumComplexe::new(1.0, 3.0);

        memory.add_var(String::from("1"), num_complexe1);
        memory.add_var(String::from("2"), num_complexe2);
        memory.add_var(String::from("3"), num_complexe3);

        match memory.get_var(String::from("2")) {
            Some(num) => assert!(num.get_imag() == 2.0),
            None => assert!(false)
        }
    }
}