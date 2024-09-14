use std::io::{prelude::*, stdin,stdout, Write, BufReader};
use std::net::TcpStream;
use std::thread;

pub fn connect(){
    
    if let Ok(mut stream) = TcpStream::connect("127.0.0.1:33777"){
        // We have connected to the server
        println!("Connected to server!");

        // User Buffer
        let mut user_buffer = String::new();
        let recvstream = stream.try_clone().unwrap();
        thread::spawn(|| {
            msg_recv(recvstream);
        });
        

        loop{
            stdout().flush().unwrap(); // allows for the input to be on same line as previous print
            stdin().read_line(&mut user_buffer).unwrap();
            stream.write_all(user_buffer.as_bytes()).unwrap();
            user_buffer.clear();
            stream.flush().unwrap();

        }

    }else{
        println!("Couldn't connect to server...")
    }
}


pub fn msg_recv(stream: TcpStream){
    let mut reader = BufReader::new(stream);
    let mut server_buffer = String::new();
    loop{
        while reader.read_line(&mut server_buffer).unwrap() > 0{
            if server_buffer.trim() != ""{
                println!("{}", server_buffer.trim());
            }
            server_buffer.clear();
        }
    }   
}