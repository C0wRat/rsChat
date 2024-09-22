use std::io::{prelude::*, Write, BufReader};
use std::net::TcpStream;
use std::thread;
use std::fs;
use std::time::Duration;


pub fn connect(ip: String){

    let server_address = format!("{ip}:33777");
    println!("JOINING: {server_address}");
    if let Ok(mut stream) = TcpStream::connect(server_address){
        // We have connected to the server
        println!("Connected to server!");

        // User Buffer
        let mut user_buffer = String::new();
        let recvstream = stream.try_clone().unwrap();
        thread::spawn(|| {
            msg_recv(recvstream);
        });
        

        loop{
            thread::sleep(Duration::from_secs(1));
            let mut data_file = fs::File::open("send.txt").unwrap();
            // Create an empty mutable string
            let mut message_buffer = String::new();

            // Copy contents of file to a mutable string
            data_file.read_to_string(&mut message_buffer).unwrap();
            // println!("{message_buffer}");
            let _ = fs::write("send.txt", "");
            if message_buffer != ""{
                // println!("Sending: {}", message_buffer);
                stream.write_all(message_buffer.as_bytes()).unwrap();
                user_buffer.clear();
                stream.flush().unwrap();
            }            
        }

    }else{
        println!("Couldn't connect to server...")
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
        println!("size: {}", recv_size.trim());
        let size: u32 = recv_size.trim().parse().expect("Bad Input!");

        while i < (size-1){
            reader.read_line(&mut server_buffer).unwrap();
            i+= 1;
        }
        println!("----msg----\n{server_buffer}");
        println!("-----------");
        let _ = fs::write("recv.txt", "");
        let _ = fs::write("recv.txt", &server_buffer);
    }
}


// }
    // loop{

    //     reader.read_line(&mut server_buffer).unwrap();
    //     recv_buffer.clear();
    //     loop{
    //         reader.read_line(&mut server_buffer).unwrap();
    //         println!("Recv: {}", server_buffer);
    //         if server_buffer.trim() == ""{
    //             println!("END!");
    //             break;
    //         }
    //         recv_buffer = recv_buffer + &server_buffer;
    //         server_buffer.clear();

    //     }
    //     println!("Writing: {}", server_buffer.trim());
    //     let _ = fs::write("recv.txt", "");
    //     println!("Client got: {}", server_buffer);
    //     let _ = fs::write("recv.txt", &server_buffer);

//     }
// }

    // loop{
    //     reader.read_line(&mut server_buffer).unwrap();
    //     println!("RECV: {}", server_buffer.trim());
    //     if server_buffer == "-"{
    //         println!("!!!!!!END!!!!!!!!!");
    //     }

    //     if server_buffer.trim() != ""{
    //         let _ = fs::write("recv.txt", "");
    //         println!("Client got: {}", server_buffer);
    //         let _ = fs::write("recv.txt", &server_buffer);
    //     }
    //     // server_buffer.clear();
    // }   







// -----------------

// loop{
//     println!("------ client recv ---------");
//     server_buffer.clear();
//     reader.read_line(&mut recv_size).unwrap();
//     println!("recv {}", recv_size);
//     println!("------ client recv end ---------");

//     if server_buffer.trim() != ""{
//         let size: u32 = recv_size.trim().parse().expect("Bad Input!");
//         let mut i = 0;
//         server_buffer.clear();
//         while i < (size-1){
//             reader.read_line(&mut server_buffer).unwrap();
//             println!("msg data");
//             println!("recv {}", server_buffer);
//             i+=1;
//         }
//     // println!("Writing: {}", server_buffer.trim());
//     let _ = fs::write("recv.txt", "");
//     let _ = fs::write("recv.txt", &server_buffer);
//     thread::sleep(Duration::from_millis(2000));
//     // println!("-------------- end recv --------------");
//     }
// }