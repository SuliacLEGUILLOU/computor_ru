
pub mod information {
    pub fn welcome() {
        println!("Welcome to computorV2.");
    }

    pub fn help() {
        println!("Complexe number are supported");
        println!("Assigne variable :\nvar <name> = <R>(+<I>i)?");

        println!("printing information :");
        println!("print shows everyting recorded in the memory");
        println!("print_var shows only variable");
    }
}

pub mod warning {

}

pub mod error {
    pub fn no_input() {
        println!("No user input");
    }
}