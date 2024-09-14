use std::net::TcpListener;
use std::io::BufReader;
use std::io::prelude::*;
use std::net::TcpStream;
use std::thread;
use std::fs;
use std::time::Duration;



pub fn host_server(){
    let _ = fs::File::create("msg.txt"); // Probably should make this safer!
    thread::spawn(|| {
        create_server();
    });
}

pub fn create_server(){
    let listener = TcpListener::bind("127.0.0.1:33777").unwrap();

    for stream in listener.incoming() {
        let stream = stream.unwrap();
        let send_stream = stream.try_clone().unwrap();

        thread::spawn(|| {
            session(stream);
        });

        thread::spawn(|| {
            sender(send_stream);
        });



        
    }    

}

pub fn session(stream: TcpStream){
    // println!("Connection established!");
        let mut reader = BufReader::new(stream);
        let mut server_buffer = String::new();

        let mut username = String::new();
        reader.read_line(&mut username).unwrap();

        loop{
            reader.read_line(&mut server_buffer).unwrap();
            if server_buffer.trim() != ""{
                log(&username, &server_buffer);
            }
            server_buffer.clear();
        }
}

pub fn log(username: &String, message: &String){
    let data = format!("{}: {}\n",username.trim(), message.trim());
    // let _ = fs::write("msg.txt", data);  //probably should make this safer

    let mut data_file = fs::OpenOptions::new()
        .append(true)
        .open("msg.txt")
        .expect("cannot open file");
    let _ = data_file.write(data.as_bytes()); // Make this safer!
}


pub fn sender(mut stream: TcpStream){
    let mut data_file = fs::File::open("msg.txt").unwrap();

    // Create an empty mutable string
    let mut file_content = String::new();

    loop{
        thread::sleep(Duration::from_millis(100));
        // Copy contents of file to a mutable string
        data_file.read_to_string(&mut file_content).unwrap();

        if file_content.trim() != ""{
            // println!("{}", file_content.trim());

            stream.write_all(file_content.as_bytes()).unwrap();
            stream.flush().unwrap();
        }
        
    }
}