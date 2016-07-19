use std::sync::mpsc::{sync_channel, SyncSender};
use std::sync::{Arc, Mutex};
use std::sync::atomic::{AtomicUsize, Ordering};
use std::fs::File;
use std::thread;
use std::time::Duration;

use byteorder::{LittleEndian, ReadBytesExt, WriteBytesExt};

pub struct TrollCount {
    mailbox: Mutex<Option<SyncSender<()>>>,
    value: Arc<AtomicUsize>,
    joiner: Option<thread::JoinHandle<()>>,
}

impl TrollCount {
    pub fn create() -> TrollCount {
        let count = match File::open("troll-count.txt") {
            Ok(mut file) => {
                file.read_u64::<LittleEndian>()
                    .map(|u| u as usize)
                    .expect("trollcount.txt is bad")
            }
            Err(_) => 0
        };

        let (sender, receiver) = sync_channel(0);
        let mailbox = Mutex::new(Some(sender));
        let value = Arc::new(AtomicUsize::new(count));
        let value_clone = value.clone();

        let joiner = thread::spawn(move || {
            if receiver.recv().is_err() {
                return;
            }

            // Give it a little time in case a bunch of requests come in
            thread::sleep(Duration::from_millis(100));
            let value = value.load(Ordering::Relaxed) as u64;

            match File::create("troll-count.txt") {
                Ok(mut file) => {
                    file.write_u64::<LittleEndian>(value)
                        .expect("Failure while writing to troll-count.txt");
                },
                Err(e) => println!("[WARNING]: Failed to write to troll-count.txt: {:?}", e),
            }
        });

        TrollCount {
            mailbox: mailbox,
            value: value_clone,
            joiner: Some(joiner),
        }
    }

    pub fn tick(&self) {
        self.value.fetch_add(1, Ordering::SeqCst);
        self.mailbox.lock().unwrap().as_ref().map(|sender| sender.try_send(()).ok());
    }

    pub fn value(&self) -> usize {
        self.value.load(Ordering::SeqCst)
    }
}

impl Drop for TrollCount {
    fn drop(&mut self) {
        *self.mailbox.lock().unwrap() = None;
        self.joiner.take().unwrap().join().ok();
    }
}
