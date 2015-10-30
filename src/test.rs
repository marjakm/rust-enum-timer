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
}

struct EventIterator<'a> {
    storage: &'a EventStorage,
    idx:     u32,
}
impl<'a> EventIterator<'a> {
    pub fn new(storage: &'a EventStorage) -> EventIterator<'a> {
        EventIterator { storage: storage, idx: 0}
    }
}
impl<'a> Iterator for EventIterator<'a> {
    type Item = Option<TimerEvent<Event>>;

    fn next(&mut self) -> Option<Self::Item> {
        let res = match self.idx {
            0 => if self.storage.yks.is_some() { Some(Some(TimerEvent {variant: Event::Yks, when: self.storage.yks.as_ref().unwrap()} ))} else { Some(None) },
            1 => if self.storage.kaks.is_some() { Some(Some(TimerEvent {variant: Event::Kaks, when: self.storage.kaks.as_ref().unwrap()} ))} else { Some(None) },
            _ => None
        };
        self.idx += 1;
        res
    }

}

impl<'a> TimerStorage<Event, EventIterator<'a>> for EventStorage {

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

    fn iter(&'a self) -> EventIterator {
        EventIterator::new(self)
    }

}


#[test]
fn test_name() {
    unimplemented!()
}
