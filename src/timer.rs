/*
 * The MIT License (MIT)
 *
 * Copyright (c) 2015 Mattis Marjak (mattis.marjak@gmail.com)
 *
 * Permission is hereby granted, free of charge, to any person obtaining a copy
 * of this software and associated documentation files (the "Software"), to deal
 * in the Software without restriction, including without limitation the rights
 * to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
 * copies of the Software, and to permit persons to whom the Software is
 * furnished to do so, subject to the following conditions:
 *
 * The above copyright notice and this permission notice shall be included in all
 * copies or substantial portions of the Software.
 *
 * THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
 * IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
 * FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
 * AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
 * LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
 * OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
 * SOFTWARE.
 */

use std::fmt;
use std::thread::spawn;
use std::sync::{Arc, Mutex, Condvar};
use std::sync::mpsc::{channel, Sender, Receiver};
use time::{SteadyTime, Duration};
use ::storage::TimerStorage;

pub struct Timer<T: fmt::Debug> {
    sender: Sender<TimerRequest<T>>,
    sync:   Arc<(Mutex<bool>, Condvar)>,
}
impl<T: fmt::Debug> fmt::Debug for Timer<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Timer<{:?}>", stringify!(T))
    }
}

impl<T> Timer<T> where T: Clone+Send+'static+fmt::Debug {
    pub fn new<S, F>(process: F) -> Self
        where S: TimerStorage<T>+Send+'static,
              F: Fn(T) -> (),
              F: Send + 'static {

        let (sender, receiver) = channel::<TimerRequest<T>>();
        let sync  = Arc::new((Mutex::new(false), Condvar::new()));
        let sync2 = sync.clone();
        spawn(move || timer_thread(receiver, process, sync2, S::new()));
        Timer { sender: sender, sync: sync }
    }

    pub fn start(&self, variant: T, timeout: u32) {
        self.sender.send(TimerRequest::Start(variant, SteadyTime::now()+Duration::milliseconds(timeout as i64)))
            .unwrap_or_else(|e| panic!("Start timer send error: {:?}", e));
        let &(ref mtex, ref cvar) = &*self.sync;
        let _m = mtex.lock().unwrap();
        cvar.notify_one();
    }

    pub fn stop(&self, variant: T) {
        self.sender.send(TimerRequest::Stop(variant))
            .unwrap_or_else(|e| panic!("Stop timer send error: {:?}", e));
            let &(ref mtex, ref cvar) = &*self.sync;
            let _m = mtex.lock().unwrap();
            cvar.notify_one();
    }
}

impl<T: fmt::Debug> Drop for Timer<T> {
    fn drop(&mut self) {
        let &(ref mtex, ref cvar) = &*self.sync;
        let mut should_stop = mtex.lock().unwrap();
        *should_stop = true;
        cvar.notify_one();
    }
}

enum TimerRequest<T> {
    Start(T, SteadyTime),
    Stop(T),
}

#[derive(Debug)]
pub enum TimerAction<T> {
    Trigger(T),
    Wait(u32),
}

fn timer_thread<T, S, F>(receiver:     Receiver<TimerRequest<T>>,
                         process:      F,
                         sync:         Arc<(Mutex<bool>, Condvar)>,
                         mut storage:  S )
                where T: Clone+fmt::Debug,
                      S: TimerStorage<T>,
                      F: Fn(T) -> (),
                      F: Send + 'static {
    let &(ref mtex, ref cvar) = &*sync;
    let mut mutex = mtex.lock().unwrap();
    while !*mutex {
        while let Ok(request) = receiver.try_recv() {
            match request {
                TimerRequest::Start(var, when) => storage.set(&var, when),
                TimerRequest::Stop(var)        => { storage.clear(&var); }
            };
        };
        match storage.next_action() {
            TimerAction::Trigger(var)  => {
                process(var);
            },
            TimerAction::Wait(timeout) => {
                mutex = cvar.wait_timeout(mutex, ::std::time::Duration::from_millis(timeout as u64)).unwrap().0;
            }
        }
    }
}
