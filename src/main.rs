use std::net::TcpListener;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:5566").unwrap();

    for stream in listener.incoming() {
        match stream {
            Ok(_) => {
                println!("Connection Successful");
            }
            Err(_) => todo!(),
        }
    }
}
