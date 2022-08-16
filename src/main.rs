use cheat_sheet::*;
mod cheat_sheet;

use std::vec;

fn main() {
    // thread();
    // server();
    // structure_calc();
    cheat_sheat();
}

#[allow(dead_code)]
fn thread() {
    use std::thread;

    let mut handles: Vec<thread::JoinHandle<()>> = vec![];

    for _ in 0..10 {
        let handle: thread::JoinHandle<()> = thread::spawn(move || println!("Hello world"));
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    return;
}

#[allow(dead_code)]
fn server() {
    use std::fs;
    use std::io::prelude::*;
    use std::net::TcpListener;
    use std::net::TcpStream;

    TcpListener::bind("127.0.0.1:3001")
        .unwrap()
        .incoming()
        .for_each(|stream: Result<std::net::TcpStream, std::io::Error>| {
            let stream: std::net::TcpStream = stream.unwrap();
            handle_connection(stream);
        });

    fn handle_connection(mut stream: TcpStream) {
        let mut buffer: [u8; 1024] = [0; 1024];
        stream.read(&mut buffer).unwrap();

        let (status, file) = if buffer.starts_with(b"GET / HTTP/1.1\r\n") {
            ("HTTP/1.1 200 OK", "index.html")
        } else {
            ("HTTP/1.1 404 NOT FOUND", "404.html")
        };

        let contents: String = fs::read_to_string(file).unwrap();

        let res: String = format!(
            "{}\r\nContent_Lenght: {}\r\n\r\n{}",
            status,
            contents.len(),
            contents
        );

        stream.write(res.as_bytes()).unwrap();
        stream.flush().unwrap();

        // println!("Test: {}", String::from_utf8_lossy(&buffer[..]))
    }
}

#[allow(dead_code)]
fn structure_calc() {
    struct Structure {
        number_one: u32,
        number_two: u32,
    }

    impl Structure {
        fn area(&self) -> u32 {
            self.number_one * self.number_two
        }
    }

    let rect: Structure = Structure {
        number_one: 55,
        number_two: 44,
    };

    println!("rect: {}", rect.area());
}
