use std::vec;

pub fn thread() {
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
