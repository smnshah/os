use super::types::{Cause, Event, EventData, EventId, EventKind};

pub const CAPACITY: usize = 4096;

struct EventRingBuffer {
    ring_buffer: [Option<Event>; CAPACITY],
    write_idx: usize,
    next_sequence: u64,
    count: usize,
}

impl EventRingBuffer {
    pub fn new() -> Self {
        Self {
            ring_buffer: [None; CAPACITY],
            write_idx: 0,
            next_sequence: 0,
            count: 0,
        }
    }

    pub fn record(&mut self, kind: EventKind, cause: Cause, data: EventData) -> EventId {
        let event_id = EventId::new(0, self.next_sequence);
        let event = Event {
            id: event_id,
            kind: kind,
            cause: cause,
            data: data,
        };

        self.ring_buffer[self.write_idx] = Some(event);

        self.write_idx = (self.write_idx + 1) % CAPACITY;
        self.next_sequence += 1;

        if self.count < CAPACITY {
            self.count += 1;
        }

        event_id
    }
}
