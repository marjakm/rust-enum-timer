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

use time::SteadyTime;
use ::timer::TimerAction;
use std::fmt;


#[derive(Debug)]
pub struct TimerEvent<T: fmt::Debug> {
    pub variant: T,
    pub when:    SteadyTime,
}

pub trait TimerStorage<T> where T: Clone+fmt::Debug {
    fn new() -> Self;
    fn clear(&mut self, variant: &T);
    fn set(&mut self, variant: &T, when: SteadyTime);
    fn next(&mut self) -> Option<TimerEvent<T>>;
    fn reset_next(&mut self);

    fn next_action(&mut self) -> TimerAction<T> {
        let mut timeout = 60_000;
        let mut trigger = None;
        let now = SteadyTime::now();
        self.reset_next();
        while let Some(evt) = self.next() {
            let time_to_event = (evt.when-now).num_milliseconds();
            if time_to_event <= 0 {
                trigger = Some(evt.variant.clone());
                break
            } else if time_to_event < timeout {
                timeout = time_to_event;
            }
        };
        match trigger {
            Some(x) => { self.clear(&x); TimerAction::Trigger(x) },
            None    => TimerAction::Wait(timeout as u32)
        }
    }
}
