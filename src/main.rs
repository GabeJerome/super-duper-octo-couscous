use std::io;

fn main() {
    /************* Start a loop until user wants to quit *************/
    loop {
        /******************* Prompt user for input *******************/
        println!("\nEnter a positive integer to calculate its factorial or \"quit\" to exit: ");

        /********************** Read user input **********************/
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");
        let input = input.trim();

        if input.eq_ignore_ascii_case("quit") {
            println!("exiting...");
            break;
        }

        /*********** Start a loop until user wants to quit ***********/
        /******* Use match to handle return type for errors **********/
        match input.parse::<u32>() {
            Ok(num) => {
                println!("\nCalculating the factorial of {}", num);

                /********* Call the factorial function *********/
                let result = factorial(num);
                println!("\nThe factorial of {} is: {}", num, result);
            }
            Err(_) => println!("Please enter a positive integer"),
        }
    }
}

/************* Recursive function to calculate factorial *************/
fn factorial(n: u32) -> u32 {
    if n <= 1 {
        println!("Base case: returning 1");
        1
    } else {
        println!("returning {} x {}!", n, n - 1);
        n * factorial(n - 1)
    }
}
