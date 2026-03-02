use core::cmp::{max, min};

use super::{
    buffer::{self, BufferReadError},
    types::{Cause, Event, EventData, EventId, EventKind, RootCause},
};

const MAX_DRAIN_BATCH_SIZE: usize = 64;

const SCRATCH_EVENT: Event = Event {
    id: EventId::new(0, 0),
    kind: EventKind::Boot,
    cause: Cause::Root(RootCause::Boot),
    data: EventData::None,
};

#[derive(Debug)]
pub struct DrainCursor {
    pub core_id: u16,
    pub next_seq: u64,
}

#[derive(Debug)]
pub struct SeqWindow {
    pub oldest_seq: u64,
    pub next_seq: u64,
}

#[derive(Debug)]
pub struct LossRecord {
    pub core_id: u16,
    pub dropped_start_seq: u64,
    pub dropped_end_seq: u64,
}

#[derive(Debug)]
pub struct DrainResult {
    pub records_written: usize,
    pub next_cursor: DrainCursor,
    pub window: SeqWindow,
}

#[derive(Debug)]
pub enum DrainRecord {
    Event(Event),
    Loss(LossRecord),
}

#[derive(Debug)]
pub enum DrainError {
    CorruptState,
    InvalidCore,
    OutputTooSmall,
    Uninitialized,
}

/// Drains causal events for one core provided by `cursor` into `out`.
///
/// Behavior:
/// Drains up to `MAX_DRAIN_BATCH_SIZE` events into `out`, starting from the max of the provided
/// next sequence and the core's oldest retained sequence. If there are events that are lost,
/// stores a `LossRecord` into the first element of the `out` array that stores the range of
/// events which were dropped.
///
/// Returns `DrainResult` with:
/// `records_written`, the number of records written into `out`.
/// `next_cursor`, a `DrainCursor` with the next sequence to be captured for the core.
/// `window`, reports the source retention observed during the call.
///
/// Errors:
/// `CorruptState` if an expected retained slot of the causality buffer is unexpectedly empty.
/// `InvalidCore` if the `core_id` provided by `cursor` is out of range.
/// `OutputTooSmall` if the `out` array is empty.
/// `Uninitialized` if causality buffers are not initialized.
pub fn drain_from(cursor: DrainCursor, out: &mut [DrainRecord]) -> Result<DrainResult, DrainError> {
    if out.is_empty() {
        return Err(DrainError::OutputTooSmall);
    }

    let mut events = [SCRATCH_EVENT; MAX_DRAIN_BATCH_SIZE];
    let event_cap = min(out.len(), MAX_DRAIN_BATCH_SIZE);

    let drain_res =
        buffer::read_core_events(cursor.core_id, cursor.next_seq, &mut events[..event_cap])
            .map_err(map_buffer_err)?;

    let loss_occurred = cursor.next_seq < drain_res.oldest_seq;
    let loss_written = if loss_occurred { 1 } else { 0 };
    let out_start = loss_written;
    let event_budget = event_cap - loss_written;
    let events_to_copy = min(event_budget, drain_res.events_written);

    if loss_occurred {
        out[0] = DrainRecord::Loss(LossRecord {
            core_id: cursor.core_id,
            dropped_start_seq: cursor.next_seq,
            dropped_end_seq: drain_res.oldest_seq - 1,
        });
    }

    for i in 0..events_to_copy {
        out[out_start + i] = DrainRecord::Event(events[i]);
    }

    let effective_start_seq = max(cursor.next_seq, drain_res.oldest_seq);

    Ok(DrainResult {
        records_written: loss_written + events_to_copy,
        next_cursor: DrainCursor {
            core_id: cursor.core_id,
            next_seq: effective_start_seq + events_to_copy as u64,
        },
        window: SeqWindow {
            oldest_seq: drain_res.oldest_seq,
            next_seq: drain_res.next_seq,
        },
    })
}

fn map_buffer_err(err: BufferReadError) -> DrainError {
    match err {
        BufferReadError::CorruptSlot => DrainError::CorruptState,
        BufferReadError::InvalidCore => DrainError::InvalidCore,
        BufferReadError::Uninitialized => DrainError::Uninitialized,
    }
}
