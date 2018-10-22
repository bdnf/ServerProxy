use std::sync::mpsc; //creates a channel
use std::thread;

use std::sync::Arc; //type will let multiple workers own the receiver
use std::sync::Mutex; //will ensure that only one worker gets a job from the receiver at a time

enum Message {
    NewJob(Job),
    Terminate,
}

//adding a new trait FnBox to work around the current limitations of Box<FnOnce()>
trait FnBox {
    fn call_box(self: Box<Self>);
    //to take ownership of self and move the value out of the Box<T>.
}

impl<F: FnOnce()> FnBox for F {
    //uses (*self)() to move the closure out of the Box<T> and call the closure
    fn call_box(self: Box<F>) {
     (*self)()
    }
}

//type alias
//type Job = Box<FnBox + Send + 'static>;
type Job = Box<FnBox + Send + 'static>;  //with taking ownership of self

pub struct ThreadPool {
    workers: Vec<Worker>,
    sender: mpsc::Sender<Message>,
}

impl ThreadPool {
    pub fn new(size: usize) -> ThreadPool {
        assert!(size > 0);

        let (sender, receiver) = mpsc::channel(); //create channel
        let receiver = Arc::new(Mutex::new(receiver));

        let mut workers = Vec::with_capacity(size);

           for id in 0..size {
               // create some threads and store them in the vector

               //workers.push(Worker::new(id, receiver)); //cannot be distributed among workers
               //solution
               workers.push(Worker::new(id, Arc::clone(&receiver)));
           }

           ThreadPool {
               workers,
               sender,
           }
    }

    pub fn execute<F>(&self, f: F)
       where
           F: FnOnce() + Send + 'static
       {
           let job = Box::new(f);
           self.sender.send(Message::NewJob(job)).unwrap(); // send job down the sending end of the channel
       }
}


struct Worker {
       id: usize,
       thread: Option<thread::JoinHandle<()>>, //the value out of the Some variant and leave a None variant in its place.
}

impl Worker {

    fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Message>>>) -> Worker {
     // let thread = thread::spawn(|| {
     //     receiver;
     // });

     let thread = thread::spawn(move || {
            loop {
                let message = receiver.lock().unwrap().recv().unwrap();

                println!("Worker {} got a job; executing.", id);

                match message {
                    Message::NewJob(job) => {
                        println!("Worker {} got a job; executing.", id);

                        //(*job)(); does not compile. need a wrapper
                        job.call_box();
                    },
                    Message::Terminate => {
                        println!("Worker {} was told to terminate.", id);
                        break;
                    },
                }


            }
        });

       Worker {
        id,
        //thread,
        thread: Some(thread), //to leave an empty thread upon shutting server down
       }
   }
}

impl Drop for ThreadPool {
    fn drop(&mut self) {
        println!("Sending terminate message to all workers.");
        for _ in &mut self.workers {
          self.sender.send(Message::Terminate).unwrap();
        }
        println!("Shutting down all workers.");
        
        for worker in &mut self.workers {
         println!("Shutting down worker {}", worker.id);

         //worker.thread.join().unwrap();
         if let Some(thread) = worker.thread.take() {
             thread.join().unwrap();
         } //takes the Some variant out and leaves None
        }
    }
}
