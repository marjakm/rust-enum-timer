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

use std::thread;

#[test]
fn manual_timer() {
    let t = Timer::new::<EventStorage, _>(|e| println!("Event triggered: {:?}", e));
    t.start(Event::Yks, 700);
    t.start(Event::Kaks, 200);
    thread::sleep_ms(4000);
}
