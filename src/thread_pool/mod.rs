use std::{
    sync::{
        mpsc::{channel, Receiver, Sender},
        Arc, Mutex,
    },
    thread::{self, JoinHandle},
};

type Job = Box<dyn FnOnce() + Send + 'static>;

pub struct ThreadPool {
    workers: Vec<Worker>,
    sender: Sender<Job>,
}

impl ThreadPool {
    pub fn new(number: usize) -> Self {
        let mut workers = Vec::with_capacity(number);

        let (tx, rx) = channel::<Job>();

        let receiver = Arc::new(Mutex::new(rx));

        for id in 0..number {
            let receiver = Arc::clone(&receiver);

            let worker = Worker::new(id, receiver);
            workers.push(worker);
        }

        Self {
            workers,
            sender: tx,
        }
    }

    pub fn exec(&self, job: Job) {
        let _ = self.sender.send(job);
    }
}

struct Worker {
    id: usize,
    handle: JoinHandle<()>,
}

impl Worker {
    fn new(id: usize, receiver: Arc<Mutex<Receiver<Job>>>) -> Self {
        let handle = thread::spawn(move || loop {
            let message = receiver.lock().unwrap().recv();

            if let Ok(job) = message {
                println!("Worker {} is handling a connection", id);
                job();
            } else {
                println!("Worker {} is disconnecting", id);
                break;
            }
        });

        Self { id, handle }
    }
}
