use std::io::{stdin, Write};
use std::net::TcpStream;

pub fn connect(){
    
    if let Ok(mut stream) = TcpStream::connect("127.0.0.1:33777"){
        // We have connected to the server

        // let mut input_stream = stream.try_clone().unwrap();
        // let output_stream = &mut stream;

	    let mut user_buffer = String::new();

        println!("Connected to server!");

        loop{
            stdin().read_line(&mut user_buffer).unwrap();
            stream.write(user_buffer.as_bytes()).unwrap();
            stream.flush().unwrap();
        }

    }else{
        println!("Couldn't connect to server...")
    }
}