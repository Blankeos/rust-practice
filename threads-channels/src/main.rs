fn main() {
    let (tx, rx) = std::sync::mpsc::channel();

    std::thread::spawn(move || {
        std::thread::sleep(std::time::Duration::from_millis(1000));
        let data = "✅ icons loaded".to_string();
        tx.send(data).unwrap();
    });

    println!("Hello, world!");

    let messages = match rx.try_recv() {
        Ok(msg) => msg,
        Err(_) => "No message yet".to_string(),
    };

    println!("{}", messages);
    // loop {
    //     match rx.try_recv() {
    //         Ok(messages) => {
    //             println!("{}", messages);
    //             break;
    //         }
    //         Err(std::sync::mpsc::TryRecvError::Empty) => {
    //             println!("Waiting for message...");
    //             std::thread::sleep(std::time::Duration::from_millis(100));
    //         }
    //         Err(std::sync::mpsc::TryRecvError::Disconnected) => {
    //             println!("Channel disconnected!");
    //             break;
    //         }
    //     }
    // }

    println!("Ending!")
}
