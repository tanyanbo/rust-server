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
    sender: Option<Sender<Job>>,
}

impl Drop for ThreadPool {
    fn drop(&mut self) {
        drop(self.sender.take());

        for worker in &mut self.workers {
            if let Some(handle) = worker.handle.take() {
                handle.join().unwrap();
            }
        }
    }
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
            sender: Some(tx),
        }
    }

    pub fn exec(&self, job: Job) {
        let _ = self.sender.as_ref().unwrap().send(job);
    }
}

struct Worker {
    id: usize,
    handle: Option<JoinHandle<()>>,
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

        Self {
            id,
            handle: Some(handle),
        }
    }
}
