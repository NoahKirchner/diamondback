/*
Queue Object

Essentially a wrapper around VecDeque that implements thread
safety via mutexes and allows for empty checks to avoid
having to deal with options during a loop.

P.S. I know there is a lot of copy pasted code, but I figured
it would be easier to read than a bunch of nested functions.

*/

// Internal queue structure.
use std::collections::VecDeque;
// Used for thread safety.
use std::sync::{Mutex,Condvar};

pub trait QueueMethods<T>{
    // Constructor
    fn new()->Self;

    // Acquires lock on the queue and pushes an item into it.
    fn push(&self, input:T);

    // Pull an item from the queue. Should be impossible to pull
    // an empty item.
    fn pop (&self)->T;

    // Number of elements in the queue
    fn length(&self)->usize;

    // Check to see if the queue is empty to update status.
    fn is_empty(&self)->bool;

}

pub struct Queue<T> {
    // Allows easy checking of the queue so you don't
    // have to call length every time. Using this over
    // a boolean because it apparently blocks the CPU.
    condition: Condvar,
    // Mutex allows for thread safety.
    data: Mutex<VecDeque<T>>,
    
}

impl<T> QueueMethods<T> for Queue<T> {

    fn new()-> Self{
        Self {
        condition:Condvar::new(),
        data: Mutex::new(VecDeque::new()),
        }
    }

    fn push(&self, input:T){
        let mut data = self.data.lock()
                    .expect("Queue lock has failed on push.");
        // Pushes data now that lock has been acquired.
        data.push_back(input);

        // Sets condition to populated.
        self.condition.notify_one();
    }

    fn pop(&self)->T {
        let mut data = self.data.lock()
                    .expect("Queue lock has failed on pop.");

        while data.is_empty() {
            data = self.condition.wait(data)
                    .expect("Queue failed condition check");
            
        }
        return data.pop_front()
                .expect("Queue Failed pop after lock secured, return value was empty.
                Issue with empty check.");

    }

    fn length(&self)->usize{
        let data = self.data.lock()
                    .expect("Queue lock has failed on length check.");
        data.len()
    }

    fn is_empty(&self)->bool {
        let data = self.data.lock()
                    .expect("Queue lock has failed on empty check.");
        data.is_empty()

    }



}
