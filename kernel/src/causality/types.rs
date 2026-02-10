/// Causal relationship for an event
#[derive(Clone, Copy, Debug)]
pub enum Cause {
    /// No parent event, starts a new causal chain
    Root(RootCause),
    /// Direct immediate causal parent event
    CausedBy(EventId),
}

#[derive(Clone, Copy, Debug)]
pub enum RootCause {
    Boot,
}

#[derive(Clone, Copy, Debug)]
pub enum EventKind {
    Boot,
}

#[derive(Clone, Copy, Debug)]
pub enum EventData {
    None,
}

// cpu core + sequence number provide a globally unique EventId
#[derive(Clone, Copy, Debug)]
pub struct EventId {
    core: u16,
    sequence: u64,
}

impl EventId {
    pub const fn new(core: u16, sequence: u64) -> Self {
        Self { core, sequence}
    }

    pub const fn core(&self) -> u16 {
        self.core
    }

    pub const fn sequence(&self) -> u64 {
        self.sequence
    }
}

#[derive(Debug)]
pub struct Event {
    pub id: EventId,  
    pub kind: EventKind,
    pub cause: Cause,
    pub data: EventData,
}




