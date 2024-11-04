use std::io;
use rand::Rng;

fn main() {
    println!("Welcome to the HAUNTED RUST REALM!");
    println!("{}", SPOOKY_GHOST);
    println!("What is your *cursed* name, wanderer?");

    // Read user name
    let mut name = String::new();
    io::stdin().read_line(&mut name).expect("Failed to read line");
    let name = name.trim();

    // Random spooky comments
    let spooky_comments = [
        "A name fit for a ghostly legend...",
        "Beware, for your journey shall be cursed!",
        "Even the spirits shiver at that name...",
        "The shadows whisper your name with fear...",
    ];

    let mut rng = rand::thread_rng();
    println!("{}", spooky_comments[rng.gen_range(0..spooky_comments.len())]);

    println!("\nAlright, {}, dare to enter the *factorial abyss*...", name);

    loop {
        println!("\nEnter a positive integer to reveal its spooky factorial, or type 'quit' to escape... if you can:");

        // Read user input
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read line");
        let input = input.trim();

        // Check if the user wants to quit
        if input.eq_ignore_ascii_case("quit") {
            println!("Farewell, {}... may the specters guide you...", name);
            println!("{}", SPOOKY_SKULL);
            break;
        }

        // Try to parse the input to an integer
        match input.parse::<u32>() {
            Ok(num) => {
                println!("\nRevealing the *dark secrets* of {}!", num);
                println!("{}", SPOOKY_CAT);
                let result = spooky_factorial(num);
                println!("\nThe haunted factorial of {} is: {}!", num, result);
                println!("{}", SPOOKY_GHOST);
            }
            Err(_) => println!("That's not a number, only *specters* can help now... try again or 'quit' to flee!"),
        }
    }
}

// Recursive function to calculate factorial with spooky flair
fn spooky_factorial(n: u32) -> u32 {
    if n <= 1 {
        println!("*Shudder!* The spirits whisper... returning 1...");
        1
    } else {
        println!("Summoning {} from the underworld... multiplying with {}", n, n - 1);
        n * spooky_factorial(n - 1)
    }
}

// ASCII art for extra spookiness
const SPOOKY_GHOST: &str = r#"
               .-.
              (o o)
          ooO--(_)--Ooo-
            G H O S T
"#;

const SPOOKY_CAT: &str = r#"
      /\_/\  
     ( o.o )  
      > ^ <  
    T H E   C A T  
"#;

const SPOOKY_SKULL: &str = r#"
         ______
      .-"      "-.
     /            \
    |              |
    |,  .-.  .-.  ,|
    | )(_o/  \o_)( |
    |/     /\     \|
    (_     ^^     _)
     \__|IIIIII|__/
      | \IIIIII/ |
      \          /
       `--------`
"#;
