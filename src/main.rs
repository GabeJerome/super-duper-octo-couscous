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
         heehee      /aa \_
                   __\-  / )                 .-.
         .-.      (__/    /        haha    _/oo \
       _/ ..\       /     \               ( \v  /__
      ( \  u/__    /       \__             \/   ___)
       \    \__)   \_.-._._   )  .-.       /     \
       /     \             `-`  / ee\_    /       \_
    __/       \               __\  o/ )   \_.-.__   )
   (   _._.-._/     hoho     (___   \/           '-'
    '-'                        /     \
                             _/       \    teehee
                            (   __.-._/
"#;

const SPOOKY_CAT: &str = r#"
                             .                                   
                          ,''`.         _                        
                     ,.,'''  '`--- ._,,'|                        
                   ,'                   /                        
              __.-'                    |                         
           ''                ___   ___ |                         
         ,'                  \(|\ /|)/ |                         
        ,'                 _     _     `._                       
       /     ,.......-\    `.      __     `-.                    
      /     ,' :  .:''`|    `:`.../:.`` ._   `._                 
  ,..,'  _/' .: :'     |     |      '.    \.    \                
 /      ,'  :'.:  / \  |     | / \   ':.  . \    \               
|      /  .: :' ,'  _)  ".._,;'  _)    :. :. \    |              
 |     | :'.:  /   |   .,   /   |       :  :  |   |              
 |     |:' :  /____|  /  \ /____|       :  :  |  ,'              
  |   /    '         /    \            :'   : |,/                
   \ |  '_          /______\              , : |                  
  _/ |  \'`--`.    _            ,_   ,-'''  :.|         __       
 /   |   \..   ` ./ `.   _,_  ,'  ``'  /'   :'|      _,''/       
/   /'. :   \.   _    [_]   `[_]  .__,,|   _....,--=/'  /:       
|   \_| :    `.-' `.    _.._     /     . ,'  :. ':/'  /'  `.     
`.   '`'`.         `. ,.'   ` .,'     :'/ ':..':.    |  .:' `.   
  \.      \          '               :' |    ''''      ''     `. 
    `''.   `|        ':     .      .:' ,|         .  ..':.      |
      /'   / '"-..._  :   .:'    _;:.,'  \.     .:'   :. ''.    |
     (._,.'        '`''''''''''''          \.._.:      ':  ':   /
                                              '`- ._    ,:__,,-' 
                                                    ``'' 
"#;

const SPOOKY_SKULL: &str = r#"
                              _.--""-._
  .                         ."         ".
 / \    ,^.         /(     Y             |      )\
/   `---. |--'\    (  \__..'--   -   -- -'""-.-'  )
|        :|    `>   '.     l_..-------.._l      .'
|      __l;__ .'      "-.__.||_.-'v'-._||`"----"
 \  .-' | |  `              l._       _.'
  \/    | |                   l`^^'^^'j
        | |                _   \_____/     _
        j |               l `--__)-'(__.--' |
        | |               | /`---``-----'"1 |  ,-----.
        | |               )/  `--' '---'   \'-'  ___  `-.
        | |              //  `-'  '`----'  /  ,-'   I`.  \
      _ L |_            //  `-.-.'`-----' /  /  |   |  `. \
     '._' / \         _/(   `/   )- ---' ;  /__.J   L.__.\ :
      `._;/7(-.......'  /        ) (     |  |            | |
      `._;l _'--------_/        )-'/     :  |___.    _._./ ;
        | |                 .__ )-'\  __  \  \  I   1   / /
        `-'                /   `-\-(-'   \ \  `.|   | ,' /
                           \__  `-'    __/  `-. `---'',-'
                              )-._.-- (        `-----'
                             )(  l\ o ('..-.
                       _..--' _'-' '--'.-. |
                __,,-'' _,,-''            \ \
               f'. _,,-'                   \ \
              ()--  |                       \ \
                \.  |                       /  \
                  \ \                      |._  |
                   \ \                     |  ()|
                    \ \                     \  /
                     ) `-.                   | |
                    // .__)                  | |
                 _.//7'                      | |
               '---'                         j_| `
                                            (| |
                                             |  \
                                             |lllj
                                             |||||
"#;
