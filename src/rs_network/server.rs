use std::net::TcpListener;
use std::io::BufReader;
use std::io::prelude::*;
use std::net::TcpStream;
use std::thread;
use std::fs;
use std::time::Duration;

use crate::rs_network::client;
use crate::rs_network::server;


pub fn host_server(ip: String){
    let _ = fs::File::create("server_messages.txt");
    thread::spawn(|| {
        create_server(ip);
    });
}

pub fn create_server(ip: String){
    let server_address = format!("{ip}:33777");
    let listener = TcpListener::bind(server_address).unwrap();
    println!("[+] Server Listening on {ip}:33777");
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
    println!("[-] Server Error");    

}

pub fn session(stream: TcpStream){
        let mut reader = BufReader::new(stream);
        let mut server_buffer = String::new();

        let mut username = String::new();
        reader.read_line(&mut username).unwrap();
        let join_msg = format!("{} Has Connected to the server!", username.trim());
        println!("[+] {join_msg}");
        let server = "SERVER";
        log(&server.to_string(), &join_msg);

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
     let mut data_file = fs::OpenOptions::new()
        .append(true)
        .open("server_messages.txt")
        .expect("cannot open file");
    let _ = data_file.write(data.as_bytes()); // Make this safer!
}


pub fn sender(mut stream: TcpStream){
    let mut data_file = fs::File::open("server_messages.txt").unwrap();

    // Create an empty mutable string
    let mut file_content = String::new();
    let mut content_buffer: String = String::new();

    loop{
        // println!("A");
        thread::sleep(Duration::from_millis(200));
        // Copy contents of file to a mutable string
        data_file.read_to_string(&mut file_content).unwrap();
        if (file_content.trim() != "") & (file_content != content_buffer){

            let x = file_content.to_string();
            let d: Vec<_> = x.split('\n').collect();
            let lines = d.len().to_string() + "\n";
            
            stream.write_all(lines.as_bytes()).unwrap();
            stream.write_all(file_content.as_bytes()).unwrap();
            stream.flush().unwrap();
            content_buffer = file_content.clone();
        }
        
    }
}

pub fn start_web_server(){
    thread::spawn(|| {
        web_server();
    });

}

pub fn web_server(){
    let _ = fs::File::create("send.txt");
    let _ = fs::File::create("recv.txt");
    let listener = TcpListener::bind("127.0.0.1:80").unwrap();

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        handle_web_connection(stream);
    }
}

fn handle_web_connection(mut stream: TcpStream) {

    let buf_reader = BufReader::new(&mut stream);
    let http_request: Vec<_> = buf_reader
    .lines()
    .map(|result| result.unwrap())
    .take_while(|line| !line.is_empty())
    .collect();

    if http_request.len() > 0{
        let (status_line, filename) = if http_request[0] == "GET / HTTP/1.1" {
            ("HTTP/1.1 200 OK", "index.html")
        } else if http_request[0] == "GET /chat HTTP/1.1"{
            ("HTTP/1.1 200 OK", "chat.html")
        }else{
            ("HTTP/1.1 200 OK", "recv.txt")
        };
    
        if http_request[0] == "POST /join HTTP/1.1"{
            let data = http_request[6].to_string() + "\n";
            let msg:Vec<_>= data.split("message:").collect();
            let data: String = msg[1].to_string();
            let msg:Vec<_>=data.split(",").collect();
            let ip = msg[0].trim().to_string();
            let username = msg[1].trim().to_string() + "\n";
            
            thread::spawn(|| {
                client::connect(ip);
            });
            
            let _ = fs::write("send.txt", username);
        }

        if http_request[0] == "POST /host HTTP/1.1"{
            let data = http_request[6].to_string() + "\n";
            let msg:Vec<_>= data.split("message:").collect();
            let data: String = msg[1].to_string();
            let msg:Vec<_>=data.split(",").collect();
            let ip = msg[0].trim().to_string();
            let username = msg[1].trim().to_string() + "\n";
            
            server::host_server(ip);
            
            let _ = fs::write("send.txt", username);
        }
    
        if http_request[0] == "POST /message HTTP/1.1"{
            let data = http_request[6].to_string() + "\n";
            let msg:Vec<_>= data.split("message:").collect();
            let data: String = msg[1].to_string();
    
            let _ = fs::write("send.txt", data);
        }
    
        let contents = fs::read_to_string(filename).unwrap();

        let length = contents.len();
    
        let response = format!("{status_line}\r\nContent-Length: {length}\r\n\r\n{contents}");
    
        stream.write_all(response.as_bytes()).unwrap();
    }
}