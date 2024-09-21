use std::net::TcpListener;
use std::io::BufReader;
use std::io::prelude::*;
use std::net::TcpStream;
use std::thread;
use std::fs;
use std::time::Duration;



pub fn host_server(){
    // print!("\x1Bc");
    // println!("[+] Hosting new server.");
    // let _ = fs::File::create("msg.txt"); // Probably should make this safer!
    let _ = fs::File::create("server_messages.txt");
    thread::spawn(|| {
        create_server();
    });
}

pub fn create_server(){
    // println!("[+] Server Thread Started");
    let listener = TcpListener::bind("127.0.0.1:33777").unwrap();
    println!("[+] Server Listening");
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
    // println!("[-] Server CRASH!!!");    

}

pub fn session(stream: TcpStream){
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
    // println!("Received {}", data);
    // let mut data_file = fs::File::open("server_messages.txt").unwrap();
     let mut data_file = fs::OpenOptions::new()
        .append(true)
        .open("server_messages.txt")
        .expect("cannot open file");
    // let _ = data_file.write("".as_bytes());
    let _ = data_file.write(data.as_bytes()); // Make this safer!
}


pub fn sender(mut stream: TcpStream){
    let mut data_file = fs::File::open("server_messages.txt").unwrap();

    // Create an empty mutable string
    let mut file_content = String::new();
    let mut content_buffer: String = String::new();

    loop{
        // thread::sleep(Duration::from_millis(1000));
        // Copy contents of file to a mutable string
        data_file.read_to_string(&mut file_content).unwrap();
        if (file_content.trim() != "") & (file_content != content_buffer){

            let x = file_content.to_string();
            let d: Vec<_> = x.split('\n').collect();
            let lines = d.len().to_string() + "\n";
            
            stream.write_all(lines.as_bytes()).unwrap();
            // println!("------------ server-send-start ----------------");
            // println!("sending : {}", file_content.trim());
            // println!("------------------- end ------------------");
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

    let listener = TcpListener::bind("127.0.0.1:9000").unwrap();

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

    // println!("{http_request:#?}");
    // println!("{}", http_request[0]);
    let (status_line, filename) = if http_request[0] == "GET / HTTP/1.1" {
        ("HTTP/1.1 200 OK", "index.html")
    } else {
        ("HTTP/1.1 200 OK", "recv.txt")
    };

    if http_request[0] == "POST /message HTTP/1.1"{
        // println!("{}", http_request[2]);
        let data = http_request[2].to_string() + "\n";
        let msg:Vec<_>= data.split("message:").collect();
        let data: String = msg[1].to_string();
        // let mut data_file = fs::OpenOptions::new()
        // .append(true)
        // .open("send.txt")
        // .expect("cannot open file");
        let _ = fs::write("send.txt", data);
        // let _ = data_file.write(data.as_bytes());
    }

    let contents = fs::read_to_string(filename).unwrap();
    let length = contents.len();

    let response = format!("{status_line}\r\nContent-Length: {length}\r\n\r\n{contents}");

    stream.write_all(response.as_bytes()).unwrap();
}