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



pub struct TimerEvent<T> {
    pub variant: T,
    pub when:    SteadyTime,
}

pub trait TimerStorage<T, E>
    where T: Clone,
          E: Iterator<Item=Option<TimerEvent<T>>> {
    fn clear(&mut self, variant: &T);
    fn set(&mut self, variant: &T, when: SteadyTime);
    fn iter(&self) -> E;

    fn next_action(&mut self) -> TimerAction<T> {
        let mut timeout = 60_000;
        let mut trigger = None;
        let now = SteadyTime::now();
        for evt in self.iter().filter_map(|x| x) {
            let time_to_event = (now-evt.when).num_milliseconds();
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
