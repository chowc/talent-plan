use std::error::Error;
use std::io::Stderr;
use std::sync::{Arc, mpsc, Mutex};
use std::sync::mpsc::Sender;
use std::thread;
use std::thread::JoinHandle;

fn main() {
    let pool = ThreadPool::new(10);
    for _ in 0..100 {
        pool.spawn(|| {
            println!("spawn job...")
        });
    }
}

struct ThreadPool {
    threadSize: u32,
    producer: mpsc::Sender<Message>,
    workers: Vec<Option<JoinHandle<()>>>,
}

type Job = Box<dyn FnOnce() + Send + 'static>;
enum Message {
    OK(Job),
    None,
}

impl ThreadPool {
    fn new(threads: u32) -> ThreadPool {
        if threads == 0 {
            ()
        }
        let (sender, receiver) = mpsc::channel();
        let receiver = Arc::new(Mutex::new(receiver));
        let mut workers = Vec::with_capacity(threads as usize);
        for i in 0..threads {
            let receviverClone = Arc::clone(&receiver);
            let jh = thread::spawn(move || {
                loop {
                    let data: Message = receviverClone.lock().unwrap().recv().unwrap();
                    match data {
                        Message::OK(job) => {
                            println!("running job");
                            job();
                        },
                        Message::None => {
                            println!("worker exiting...");
                            break
                        }
                    }
                };
            });
            workers.push(Some(jh));
        };
        ThreadPool{
            threadSize: threads,
            producer: sender,
            workers,
        }
    }

    fn spawn<F>(&self, job: F) where F: FnOnce() + Send + 'static {
        self.producer.clone().send(Message::OK(Box::new(job)));
    }
}

impl Drop for ThreadPool {
    fn drop(&mut self) {
        println!("exiting threadpool...");
        for i in 0..self.workers.len() {
            self.producer.send(Message::None);
        }
        for worker in &mut self.workers {
            worker.take().unwrap().join().unwrap();
        }
    }
}