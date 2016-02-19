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
use time::SteadyTime;
use ::timer::TimerAction;


#[derive(Debug)]
pub struct TimerEvent<T: fmt::Debug> {
    pub variant: T,
    pub when:    SteadyTime,
}

pub trait TimerStorage<T> where T: Clone+fmt::Debug {
    fn new() -> Self;
    fn clear(&mut self, variant: &T);
    fn set(&mut self, variant: &T, when: SteadyTime);
    fn next_action(&mut self) -> TimerAction<T>;
}

impl<T: fmt::Debug+Clone+PartialOrd> TimerStorage<T> for Vec<TimerEvent<T>> {
    fn new() -> Self {
        Vec::new()
    }

    fn clear(&mut self, variant: &T) {
        self.retain(|e| e.variant != *variant);
    }

    fn set(&mut self, variant: &T, when: SteadyTime) {
        TimerStorage::clear(self, variant);
        let evt = TimerEvent {variant:variant.clone(), when:when};
        let idx = match self.get(0) {
            Some(x) if x.when < evt.when => {
                match self.iter().enumerate().find(|&(_,v)| v.when > evt.when) {
                    Some((i, _)) => i,
                    None => self.len()
                }
            },
            _ => 0
        };
        self.insert(idx, evt);
    }

    fn next_action(&mut self) -> TimerAction<T> {
        let first = self.get(0).map(|x| x.clone());
        match first {
            Some(evt) => match (evt.when-SteadyTime::now()).num_milliseconds() {
                t if t < 0 => TimerAction::Trigger(evt.variant.clone()),
                t          => TimerAction::Wait(t as u32)
            },
            None      => TimerAction::Wait(60_000)
        }
    }
}
