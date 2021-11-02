use std::{io::{Write}, net::{TcpListener, TcpStream}, thread};

fn handle_write(mut stream: TcpStream, ip: String) {
    let response = format!("HTTP/1.1 200 OK\r\nContent-Type: text/plain; charset=UTF-8\r\n\r\n{}\r\n", ip);

    match stream.write(response.as_bytes()) {
        Ok(_) => println!("Response sent to {}", ip),
        Err(e) => println!("Failed sending response: {}", e),
    }
}

fn handle_client(stream: TcpStream) {
    let ip = stream.local_addr().unwrap().ip().to_string();
    handle_write(stream, ip);
}

fn main() -> std::io::Result<()> {
    let port = "80";
    let listener = TcpListener::bind(format!("0.0.0.0:{}", port));

    match listener {
        Ok(listener) => {
            println!("listening on port {}", port);

            for stream in listener.incoming() {
                match stream {
                    Ok(stream) => {
                        thread::spawn(|| {
                            handle_client(stream)
                        });
                        // handle_client(stream);
                    },
                    Err(e) => println!("unable to connect: {}", e),
                }
            }
        },
        Err(e) => println!("unable to setup listener: {}", e),
    }

    Ok(())
}
