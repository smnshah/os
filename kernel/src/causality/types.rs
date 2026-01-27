
#[derive(Clone, Copy, Debug)]
pub enum Origin {
    Caused(EventId, Option<EventId>),
    Root(RootCause)
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

// cpu core + sequence number provide a gloablly unique EventId
#[derive(Clone, Copy, Debug)]
pub struct EventId {
    pub core: u16,
    pub sequence: u64,
}

#[derive(Debug)]
pub struct Event {
    pub id: EventId,  
    pub kind: EventKind,
    pub origin: Origin,
    pub data: EventData,
}




