use std::io::{prelude::*, Write, BufReader};
use std::net::TcpStream;
use std::thread;
use std::fs;
use std::time::Duration;


pub fn connect(ip: String){

    let server_address = format!("{ip}:33777");
    println!("[+] Joining {server_address}");
    if let Ok(mut stream) = TcpStream::connect(server_address){
        // We have connected to the server
        println!("[+] Connected to server!");

        // User Buffer
        let mut user_buffer = String::new();
        let recvstream = stream.try_clone().unwrap();
        thread::spawn(|| {
            msg_recv(recvstream);
        });
        

        loop{
            thread::sleep(Duration::from_millis(200));
            let mut data_file = fs::File::open("send.txt").unwrap();
            // Create an empty mutable string
            let mut message_buffer = String::new();

            // Copy contents of file to a mutable string
            data_file.read_to_string(&mut message_buffer).unwrap();

            let _ = fs::write("send.txt", "");
            if message_buffer != ""{
                stream.write_all(message_buffer.as_bytes()).unwrap();
                user_buffer.clear();
                stream.flush().unwrap();
            }            
        }

    }else{
        println!("[-] Couldn't connect to server...")
    }
}



// Thing this is the problem function needs to read line by line then and wipe the file on start.
pub fn msg_recv(stream: TcpStream){
    loop{
        let mut reader = BufReader::new(&stream);
        let mut server_buffer = String::new();
        let mut recv_size: String = String::new();
        let mut i = 0;

        reader.read_line(&mut recv_size).unwrap();
        let size: u32 = recv_size.trim().parse().expect("Bad Input!");

        while i < (size-1){
            reader.read_line(&mut server_buffer).unwrap();
            i+= 1;
        }
        let _ = fs::write("recv.txt", "");
        let _ = fs::write("recv.txt", &server_buffer);
    }
}