/*
what is needed to build concurrent channel: Mutex, Arc, and Condvar
TODO: check MutexGuard (10:53)
TODO: why Arc
TODO: Why Condvar
*/
use std::sync::{Arc, Condvar, Mutex};

pub struct Sender<T> {
    inner: Arc<inner<T>>,
}

impl<T> Sender<T> {
    pub fn send(&mut self, t: T) {
        let inner = self.inner.queue.lock().unwrap();
        queue.push(t);
    }
}

pub struct Receiver<T> {
    inner: Arc<inner<T>>,
}

impl<T> Receiver<T> {
    pub fn recv(&mut self) -> T {
        let queue = self.inner.queue.lock().unwrap();
        queue.pop();
    }
}

// common pattern in rust when there are multiple things that are shared and point to the same thing
struct Inner<T> {
    queue: Mutex<Vec<T>>,
}

pub fn channel<T>() -> (
    Sender<T>,
    Receiver<T>, /*by convention the Sender type comes first*/
) {
    let inner = Inner {
        queue: Mutex::default(),
    };
    let inner = Arc::new(inner);
    (
        Sender {
            inner: inner.clone(),
        },
        Receiver {
            inner: inner.clone(),
        },
    )
}
