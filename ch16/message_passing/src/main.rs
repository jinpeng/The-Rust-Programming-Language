use std::thread;
use std::sync::mpsc;
use std::time::Duration;

fn main() {
    let (tx, rx) = mpsc::channel();
    let tx1 = tx.clone();

    thread::spawn(move || {
        let v = vec![
            "how",
            "are",
            "you",
        ];
        for msg in v {
            tx.send(msg).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    thread::spawn(move || {
        let v = vec![
            "吃",
            "了",
            "么",
        ];
        for msg in v {
            tx1.send(msg).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });


    for received in rx {
        println!("Got {}", received);
    }
}
