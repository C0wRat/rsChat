use std::{thread, time::Duration};

// Import build consts
mod vars;

// Import Networking code
mod rs_network;


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

fn main() {
    rs_network::server::start_web_server();
    clear();
    logo();
    println!("Open http://rschat.rat-trap.io to use the app!");
    loop{
        thread::sleep(Duration::from_secs(100000));
    }
}