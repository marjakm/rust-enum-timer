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

use std::thread;
use time::SteadyTime;
use ::{Timer, TimerStorage, TimerEvent};

#[derive(Debug, Clone)]
enum Event {
    Yks,
    Kaks,
}

struct EventStorage {
    yks:  Option<SteadyTime>,
    kaks: Option<SteadyTime>,
    idx:  Option<Event>,
}

impl TimerStorage<Event> for EventStorage {
    fn new() -> Self {
        EventStorage {
            yks:  None,
            kaks: None,
            idx:  Some(Event::Yks),
        }
    }

    fn clear(&mut self, variant: &Event) {
        match *variant {
            Event::Yks  => {self.yks  = None;},
            Event::Kaks => {self.kaks = None;},
        }
    }

    fn set(&mut self, variant: &Event, when: SteadyTime) {
        match *variant {
            Event::Yks  => {self.yks  = Some(when);},
            Event::Kaks => {self.kaks = Some(when);},
        }
    }

    fn reset_next(&mut self) {
        self.idx = Some(Event::Yks);
    }

    fn next(&mut self) -> Option<TimerEvent<Event>> {
        loop {
            match self.idx {
                Some(Event::Yks)  => {
                    self.idx = Some(Event::Kaks);
                    if self.yks.is_some() {
                        return Some(TimerEvent {variant: Event::Yks, when: self.yks.as_ref().unwrap().clone()})
                    }
                },
                Some(Event::Kaks) => {
                    self.idx = None;
                    if self.kaks.is_some() {
                        return Some(TimerEvent {variant: Event::Kaks, when: self.kaks.as_ref().unwrap().clone()})
                    }
                },
                _ => {
                    self.idx = Some(Event::Yks);
                    return None
                }
            };
        }
    }

}

et_create_enum_timer! { GEventStorage;
    #[derive(Debug, Clone)]
    pub enum GEvent {
        Yks,
        Kaks
    }
}



#[test]
fn manual_timer() {
    let t = Timer::new::<EventStorage, _>(|e| println!("Manual event triggered: {:?}", e));
    t.start(Event::Yks, 700);
    t.start(Event::Kaks, 200);
    thread::sleep_ms(1000);
}

#[test]
fn generated_timer() {
    let t = Timer::new::<GEventStorage, _>(|e| println!("Generated event triggered: {:?}", e));
    t.start(GEvent::Yks, 700);
    t.start(GEvent::Kaks, 200);
    thread::sleep_ms(1000);
}
