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
    let _ = fs::File::create("C:\\ProgramData\\rsChat\\server_messages.txt");
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
        .open("C:\\ProgramData\\rsChat\\server_messages.txt")
        .expect("cannot open file");
    let _ = data_file.write(data.as_bytes()); // Make this safer!
}


pub fn sender(mut stream: TcpStream){
    let mut data_file = fs::File::open("C:\\ProgramData\\rsChat\\server_messages.txt").unwrap();

    // Create an empty mutable string
    let mut file_content = String::new();
    let mut content_buffer: String = String::new();

    loop{
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
    let _ = fs::create_dir("C:\\ProgramData\\rsChat\\");
    let _ = fs::File::create("C:\\ProgramData\\rsChat\\send.txt");
    let _ = fs::File::create("C:\\ProgramData\\rsChat\\recv.txt");
    let listener = TcpListener::bind("127.0.0.1:80").unwrap();

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        handle_web_connection(stream);
    }
}

fn handle_web_connection(mut stream: TcpStream) {

    let buf_reader = BufReader::new(&mut stream);
    // let http_request: Vec<_> = buf_reader
    // .lines()
    // .map(|result| result.unwrap())
    // .take_while(|line| !line.is_empty())
    // .collect();

    let mut url_header = String::new();
    let mut message_header = String::new();
    let mut data  = String::new();

    let mut buffer = [0; 1024];    
    stream.read(&mut buffer).unwrap();
    let request =  String::from_utf8_lossy(&buffer[..]);
    data = request.split("\n").last().unwrap().to_string();

    for line in request.split("\n"){
        if line.contains("message"){
            message_header = line.to_string();
        }
        if line.contains("HTTP/1.1"){
            url_header = line.to_string();
        }
        
    }

    if request.len() > 0{

        let (status_line, filename) = if url_header.as_str().trim() == "GET / HTTP/1.1" {
            ("HTTP/1.1 200 OK", "index")
        } else if url_header.as_str().trim() == "GET /chat HTTP/1.1"{
            ("HTTP/1.1 200 OK", "chat")
        }else{
            ("HTTP/1.1 200 OK", "recv")
        };
    
        if url_header.as_str().trim() == "POST /join HTTP/1.1"{
            // let data = http_request[6].to_string() + "\n";
            let msg:Vec<_>= message_header.split("message:").collect();
            let data: String = msg[1].to_string();
            let msg:Vec<_>=data.split(",").collect();
            let ip = msg[0].trim().to_string();
            let username = msg[1].trim().to_string() + "\n";
            
            thread::spawn(|| {
                client::connect(ip);
            });
            
            let _ = fs::write("C:\\ProgramData\\rsChat\\send.txt", username);
        }

        if url_header.as_str().trim() == "POST /host HTTP/1.1"{
            // let data = http_request[6].to_string() + "\n";
            let msg:Vec<_>= message_header.split("message:").collect();
            let data: String = msg[1].to_string();
            let msg:Vec<_>=data.split(",").collect();
            let ip = msg[0].trim().to_string();
            let username = msg[1].trim().to_string() + "\n";
            
            server::host_server(ip);
            
            let _ = fs::write("C:\\ProgramData\\rsChat\\send.txt", username);
        }
        
        if url_header.as_str().trim() == "POST /message HTTP/1.1"{

            let msg:Vec<_>= message_header.split("message:").collect();
            // let message = msg[1].trim().to_string() + "\n";
            let _ = fs::write("C:\\ProgramData\\rsChat\\send.txt", data + "\n");
        }
    
        
        let mut contents: String = String::new();

        if filename == "chat"{
            contents = "<!DOCTYPE html>
<html lang='en'>
<head>
  <meta charset='UTF-8'>
  <meta name='viewport' content='width=device-width, initial-scale=1.0'>
  


  
  <title>RSchat</title>
  <style>
    body {
      margin: 0;
      padding: 0;
      font-family: 'Segoe UI', Tahoma, Geneva, Verdana, sans-serif;
      background-color: #E6E6FA; /* Lavender background */
      height: 100vh;
      display: flex;
      justify-content: center;
      align-items: center;
    }

    .chat-container {
      width: 90%;
      height: 80%;
      background-color: #FFFFFF;
      border-radius: 15px;
      box-shadow: 0px 4px 15px rgba(0, 0, 0, 0.1);
      display: flex;
      flex-direction: column;
      overflow: hidden;
    }

    .chat-header {
      background-color: #9370DB; /* Medium Purple */
      color: white;
      padding: 15px;
      font-size: 24px;
      text-align: center;
      font-weight: bold;
      box-shadow: 0px 2px 4px rgba(0, 0, 0, 0.2);
    }

    .chat-box {
      flex-grow: 1;
      padding: 15px;
      background-color: #F8F8FF; /* GhostWhite for a softer look */
      overflow-y: auto;
      display: flex;
      flex-direction: column;
      border-bottom: 1px solid #ccc;
    }

    .message {
      margin-bottom: 10px;
      padding: 10px 15px;
      border-radius: 20px;
      max-width: 75%;
      word-wrap: break-word;
      font-size: 16px;
    }

    .server-message {
      background-color: #DCDCDC; /* Gainsboro */
      color: #555;
      text-align: center;
    }

    .user-message {
      background-color: #DDA0DD; /* Plum */
      color: #000;
      align-self: flex-start;
    }

    .self-message {
      background-color: #9370DB; /* Medium Purple */
      color: white;
      align-self: flex-end;
    }

    .chat-input-container {
      padding: 10px;
      background-color: #EEE9E9; /* LavenderBlush */
      display: flex;
      align-items: center;
      justify-content: space-between;
    }

    .chat-input {
      flex-grow: 1;
      padding: 15px;
      border: 1px solid #ccc;
      border-radius: 25px;
      font-size: 18px;
      outline: none;
    }

    .chat-input:focus {
      border-color: #9370DB; /* Medium Purple */
    }

    .send-btn {
      background-color: #9370DB;
      color: white;
      border: none;
      padding: 15px 20px;
      margin-left: 10px;
      border-radius: 25px;
      cursor: pointer;
      font-size: 18px;
    }

    .send-btn:hover {
      background-color: #836FFF; /* Slate Blue */
    }

    .chat-box::-webkit-scrollbar {
      width: 8px;
    }

    .chat-box::-webkit-scrollbar-thumb {
      background-color: #9370DB;
      border-radius: 10px;
    }
  </style>
</head>
<body>

<div class='chat-container'>
  <div class='chat-header'>RSchat</div>
  
  <div class='chat-box' id='chatBox'>
    <!-- Example Messages -->
    <!-- <div id='srv' class='message server-message'></div> -->
    <!-- <div id='rcv' class='message user-message'></div> -->
    <!-- <div id='usr' class='message self-message'></div> -->

  </div>

  <div class='chat-input-container'>
    <input type='text' class='chat-input' id='message' placeholder='Type your message...'>
    <button class='send-btn' onclick='send_msg()'>Send</button>
  </div>
</div>

<script>
  const xhr = new XMLHttpRequest(); 
  username = document.cookie.split('=')[1]
  chat_history=''
  chatbox =  document.getElementById('chatBox')

  xhr.onreadystatechange = function() {
    if (this.readyState == 4 && this.status == 200) {
        if (chat_history != this.responseText){
            chat_history = this.responseText
            chatbox.scrollTop = chatbox.scrollHeight;
            chatbox.innerHTML = ''
            messages = this.response.split('\\n')
            messages.forEach(element => {
            if(element != ''){
                if(element.split(':')[0].includes(username)){
                var div = document.createElement('div')
                div.className = 'message self-message'
                div.innerHTML = element
                chatbox.appendChild(div);
                }
                else if(element.split(':')[0].includes('SERVER')){
                var div = document.createElement('div')
                div.className = 'message server-message'
                div.innerHTML = element
                chatbox.appendChild(div);
                }else{
                var div = document.createElement('div')
                div.className = 'message user-message'
                div.innerHTML = element
                chatbox.appendChild(div);
                }
            }
            });
        }
    }
  };

  setInterval(()=>{ 
      xhr.open('GET','recv.txt',true); 
      xhr.send(); 
  },200) 

  document.addEventListener('keydown', function(event) {
    if(event.keyCode == 13) {
        message = ''
        message = document.getElementById('message').value
        document.getElementById('message').value = ''
        if (message != ''){
          xhr.open('POST', 'message', true);
          xhr.setRequestHeader('message', message);
          xhr.send(message);
        }
      }
  });

  function send_msg(){
    message = ''
    message = document.getElementById('message').value
    console.log(message)
    document.getElementById('message').value = ''
    if (message != ''){
      xhr.open('POST', 'message', true);
      xhr.setRequestHeader('message', message);
      xhr.send(message);
    }
  }

</script>



</body>
</html>
".to_string()        
        }

        if filename == "index"{
            contents = "<!DOCTYPE html>
<html lang='en'>
<head>
  <meta charset='UTF-8'>
  <meta name='viewport' content='width=device-width, initial-scale=1.0'>
  <title>RSchat Connection</title>
  <style>
    * {
      margin: 0;
      padding: 0;
      box-sizing: border-box;
    }

    body {
      font-family: 'Segoe UI', Tahoma, Geneva, Verdana, sans-serif;
      background-color: #E6E6FA; /* Lavender background */
      height: 100vh;
      display: flex;
      justify-content: flex-start; /* Align items to the top */
      align-items: center;
      flex-direction: column;
      padding-top: 50px; /* Adjust how high up the box is */
    }

    .connect-container {
      background-color: #fff;
      width: 100%;
      max-width: 400px;
      padding: 40px 30px;
      border-radius: 15px;
      box-shadow: 0px 4px 15px rgba(0, 0, 0, 0.1);
      text-align: center;
    }

    .connect-header {
      font-size: 28px;
      color: #4B0082; /* Indigo */
      margin-bottom: 15px;
      font-weight: bold;
    }

    .connect-description {
      font-size: 16px;
      color: #555;
      margin-bottom: 25px;
    }

    .input-label {
      font-size: 18px;
      color: #4B0082;
      margin-bottom: 5px;
      text-align: left;
      display: block;
    }

    .connect-input {
      width: 100%;
      padding: 12px;
      font-size: 16px;
      margin-bottom: 20px;
      border-radius: 10px;
      border: 1px solid #9370DB;
      background-color: #F8F8FF; /* GhostWhite */
      outline: none;
    }

    .connect-input:focus {
      border-color: #9370DB; /* Medium Purple */
      box-shadow: 0 0 5px rgba(147, 112, 219, 0.5);
    }

    .btn-container {
      display: flex;
      justify-content: space-between;
      margin-top: 20px;
    }

    .connect-btn {
      background-color: #9370DB;
      color: white;
      border: none;
      padding: 12px 20px;
      border-radius: 10px;
      cursor: pointer;
      font-size: 18px;
      transition: background-color 0.3s;
      width: 45%;
    }

    .connect-btn:hover {
      background-color: #836FFF; /* Slate Blue */
    }

    /* Footer */
    footer {
      position: absolute;
      bottom: 10px;
      text-align: center;
      width: 100%;
      color: #4B0082;
      font-size: 14px;
      font-weight: bold;
    }

    /* For smaller screens, make sure it is responsive */
    @media (max-width: 500px) {
      .connect-container {
        padding: 30px 20px;
      }

      .connect-btn {
        font-size: 16px;
        padding: 10px;
      }
    }
  </style>
</head>
<body>

<div class='connect-container'>
  <div class='connect-header'>
    RSchat
  </div>
  <div class='connect-description'>Welcome to RSchat!</div>

  <label class='input-label' for='username'>User Name</label>
  <input type='text' id='username' class='connect-input' placeholder='Enter your username'>

  <label class='input-label' for='server-ip'>Server IP</label>
  <input type='text' id='server-ip' class='connect-input' placeholder='127.0.0.1'>

  <div class='btn-container'>
    <button class='connect-btn' onclick='host_server()'>Host</button>
    <button class='connect-btn' onclick='join_server()'>Join</button>
  </div>
</div>

<footer>
  <a href='https://rat-trap.io/rschat'>RSchat</a> - Connect with your friends! - <a href='https://github.com/C0wRat'>C0wRat</a> .
</footer>

<script>
    const xhr = new XMLHttpRequest(); 
    lock = true
    xhr.onreadystatechange = function(evt) {
  
      if(xhr.readyState === xhr.OPENED && lock == false){
          document.location.href = 'chat';
      }
  };
  
    function join_server()
    {
      username = document.getElementById('username').value
      ip = document.getElementById('server-ip').value
      console.log(username, ip)
  
      if (ip != '' && username != '')
      {
        document.cookie = 'username=' + username; 
        lock = false
        // alert(message);
        xhr.open('POST', 'join', true);
        xhr.setRequestHeader('message', ip);
        xhr.setRequestHeader('message', username);
        xhr.send();
      }
  }
  
  function host_server()
    {
      username = document.getElementById('username').value
      ip = document.getElementById('server-ip').value
      console.log(username, ip)
  
      if (ip != '' && username != '' && lock == true)
      {
        // alert(message);
        xhr.open('POST', 'host', true);
        xhr.setRequestHeader('message', ip);
        xhr.setRequestHeader('message', username);
        xhr.send();
        lock = false
      }
  }
  
  </script>
</body>

</html>".to_string()
        }
        
        if filename == "recv"{
            contents = fs::read_to_string("C:\\ProgramData\\rsChat\\recv.txt").unwrap();
        }

        
        let length = contents.len();
    
        let response = format!("{status_line}\r\nContent-Length: {length}\r\n\r\n{contents}");
    
        stream.write_all(response.as_bytes()).unwrap();
    }
}