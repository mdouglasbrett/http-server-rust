use crate::Result;
use log::{error, info};
use std::sync::{mpsc, Arc, Mutex};
use std::thread;

type Job = Box<dyn FnOnce() + Send + 'static>;

enum Message {
    NewJob(Job),
    Terminate,
}

pub struct ThreadPool {
    workers: Vec<Worker>,
    sender: mpsc::Sender<Message>,
}

impl ThreadPool {
    pub fn new(size: usize) -> Self {
        assert!(size > 0, "The thread pool requires a count greater than 0");

        let (sender, receiver) = mpsc::channel();
        let receiver = Arc::new(Mutex::new(receiver));
        let mut workers = Vec::with_capacity(size);

        for id in 0..size {
            workers.push(Worker::new(id, Arc::clone(&receiver)));
        }

        Self { workers, sender }
    }

    pub fn execute<F: FnOnce() + Send + 'static>(&self, f: F) -> Result<()> {
        let job = Box::new(f);
        self.sender.send(Message::NewJob(job))?;
        Ok(())
    }
}

impl Drop for ThreadPool {
    fn drop(&mut self) {
        for _ in &self.workers {
            self.sender
                .send(Message::Terminate)
                .unwrap_or_else(|e| error!("Error sending termination message: {:?}", e));
        }

        for worker in &mut self.workers {
            if let Some(thread) = worker.thread.take() {
                info!("Attempting to complete worker {}", worker.id);
                if let Err(e) = thread.join() {
                    error!("Error completing worker {}: {:?}", worker.id, e);
                }
            }
        }
    }
}

pub struct Worker {
    id: usize,
    thread: Option<thread::JoinHandle<Result<()>>>,
}

impl Worker {
    fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Message>>>) -> Self {
        let thread = thread::spawn(move || -> Result<()> {
            loop {
                let message = receiver.lock()?.recv()?;

                match message {
                    Message::NewJob(job) => {
                        info!("Worker {} got a job; executing.", id);
                        job();
                    }
                    Message::Terminate => {
                        info!("Worker {} terminating.", id);
                        break;
                    }
                }
            }

            Ok(())
        });
        Self {
            id,
            thread: Some(thread),
        }
    }
}
