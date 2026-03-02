use core::cmp::{max, min};

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
    next_seq: u64,
    count: usize,
}

#[derive(Debug)]
pub(super) struct BufferReadResult {
    pub oldest_seq: u64,
    pub next_seq: u64,
    pub events_written: usize,
}

#[derive(Debug)]
pub(super) enum BufferReadError {
    CorruptSlot,
    InvalidCore,
    Uninitialized,
}

impl EventRingBuffer {
    const fn new() -> Self {
        Self {
            ring_buffer: [None; CAPACITY],
            write_idx: 0,
            next_seq: 0,
            count: 0,
        }
    }

    fn record(&mut self, core_id: u16, kind: EventKind, cause: Cause, data: EventData) -> EventId {
        let event_id = EventId::new(core_id, self.next_seq);
        let event = Event {
            id: event_id,
            kind: kind,
            cause: cause,
            data: data,
        };

        self.ring_buffer[self.write_idx] = Some(event);

        self.write_idx = (self.write_idx + 1) % CAPACITY;
        self.next_seq += 1;

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

/// Record an event into the CPU's event ring buffer. `init()` must be called before any `record()` calls.
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

/// Reads events for `core_id` starting at `start_seq` into `out`.
///
/// Behavior:
/// Returns at most `out.len()` events, in ascending sequence order. Starts reading events from
/// `max(start_seq, oldest_seq)` where oldest_seq is the current oldest retained sequence for this
/// core. Does not allocate, work is bounded by `out.len()`.
///
/// Returns `BufferReadResult` with:
/// `oldest_seq`, the current oldest retained sequence for this core.
/// `next_seq`, the next sequence for this core.
/// `events_written`, the number of events written to `out`.
///
/// Errors:
/// `InvalidCore` if the `core_id` is out of range.
/// `Uninitialized` if causality buffers are not initialized.
/// `CorruptSlot` if an expected retained slot is unexpectedly empty.
pub(super) fn read_core_events(
    core_id: u16,
    start_seq: u64,
    out: &mut [Event],
) -> Result<BufferReadResult, BufferReadError> {
    unsafe {
        let core_idx = core_id as usize;
        if core_idx >= MAX_CPUS {
            return Err(BufferReadError::InvalidCore);
        }

        if !IS_INITIALIZED {
            return Err(BufferReadError::Uninitialized);
        }

        let buffers = &raw mut EVENT_RING_BUFFERS;
        let slots = &mut *buffers;
        let buffer = &mut slots[core_idx];

        let oldest_seq = buffer.next_seq - buffer.count as u64;
        let effective_start_seq = max(start_seq, oldest_seq);
        let available_events = (buffer.next_seq - effective_start_seq) as usize;
        let to_write = min(out.len(), available_events);
        let oldest_idx = (buffer.write_idx + CAPACITY - buffer.count) % CAPACITY;
        let start_offset = (effective_start_seq - oldest_seq) as usize;

        for i in 0..to_write {
            let slot = (oldest_idx + start_offset + i) % CAPACITY;
            let event = match buffer.ring_buffer[slot] {
                Some(e) => e,
                None => return Err(BufferReadError::CorruptSlot),
            };

            out[i] = event;
        }

        Ok(BufferReadResult {
            oldest_seq: oldest_seq,
            next_seq: buffer.next_seq,
            events_written: to_write,
        })
    }
}
