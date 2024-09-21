// Import build consts
mod vars;

// Import Networking code
mod rs_network;

// Import standard library items.
use std::io::stdin; 
use std::process;

// This Function is used to clear the display.
fn clear(){
    print!("\x1Bc");
}

// This Function is used to display the rsChat logo.
fn logo(){
    println!(r#"
             ___ _           _   
 _ __ ___   / __\ |__   __ _| |_ 
| '__/ __| / /  | '_ \ / _` | __|
| |  \__ \/ /___| | | | (_| | |_ 
|_|  |___/\____/|_| |_|\__,_|\__|
                            {    }                             
---------------------------------"#, vars::VERSION);
}

// This finction is used to present a user with the main menu options to navigate the application.
fn menu(){
    let mut input: String = String::new();
    
    println!("1.) Change Username");
    println!("2.) Join Chat Room");
    println!("3.) Host Chat Room");
    println!("4.) Exit rsChat");

    // Get User Input.
    stdin().read_line(&mut input).expect("Bad Input!");

    // Convert User input to an u32.
    let option: u32 = input.trim().parse().expect("Bad Input!");

    match option{
        1 => println!("Feature Missing!"),
        2 => start_client(),
        3 => rs_network::server::host_server(),
        4 => process::exit(0),
        _=> println!("Invalid Option!")
    }
}

fn main() {
    // TODO: add variable manager to pass into functions
    loop{
        logo();
        menu();
        clear();
    }
}

fn start_client(){
    rs_network::server::start_web_server();
    rs_network::client::connect();
}
