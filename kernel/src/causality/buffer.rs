use crate::arch::x86_64::cpu;

use super::types::{Cause, Event, EventData, EventId, EventKind};

/// Fixed per-CPU slot count and IDs are indexed by APIC ID for now.
pub const MAX_CPUS: usize = 16;
pub const CAPACITY: usize = 4096;

static mut EVENT_RING_BUFFERS: [EventRingBuffer; MAX_CPUS] = [EventRingBuffer::new(); MAX_CPUS];
static mut IS_INITIALIZED: bool = false;

#[derive(Clone, Copy)]
struct EventRingBuffer {
    ring_buffer: [Option<Event>; CAPACITY],
    write_idx: usize,
    next_sequence: u64,
    count: usize,
}

impl EventRingBuffer {
    const fn new() -> Self {
        Self {
            ring_buffer: [None; CAPACITY],
            write_idx: 0,
            next_sequence: 0,
            count: 0,
        }
    }

    fn record(&mut self, core_id: u16, kind: EventKind, cause: Cause, data: EventData) -> EventId {
        let event_id = EventId::new(core_id, self.next_sequence);
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

/// One-time initialization barrier to make future-extensible. Buffers array is already statically allocated.
pub fn init() {
    unsafe {
        let initialized = &raw mut IS_INITIALIZED;
        if *initialized {
            panic!("causality::init called more than once");
        }

        *initialized = true;
    }
}

/// Record an event into the CPU's event ring buffer.
/// init() must be called before any record() calls.
/// Current APIC ID must be < MAX_CPUS.
pub fn record(kind: EventKind, cause: Cause, data: EventData) -> EventId {
    unsafe {
        let core_id = cpu::current_core_id();
        let core_idx = core_id as usize;
        if core_idx >= MAX_CPUS {
            panic!(
                "Core id ({}) is greater than max number of cpus ({})",
                core_id, MAX_CPUS
            );
        }

        if !IS_INITIALIZED {
            panic!("Causality event ring buffer not initialized. Call causality::init() first");
        }

        let buffers = &raw mut EVENT_RING_BUFFERS;
        let slots = &mut *buffers;
        let buffer = &mut slots[core_idx];

        buffer.record(core_id, kind, cause, data)
    }
}
