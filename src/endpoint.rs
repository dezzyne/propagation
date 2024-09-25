use std::fmt;

#[derive(Debug)]
pub enum EndpointKind {
    // Input
    // Recording(RecordingEndpoint),
    Recording,
    // Output
    // Playback(PlaybackEndpoint),
    Playback,
}

#[derive(Debug)]
pub struct Endpoint {
    name: String,
    kind: EndpointKind,
    index: usize,
}

#[derive(Debug)]
struct RecordingEndpoint {
    name: String,
    index: usize,
}

#[derive(Debug)]
struct PlaybackEndpoint {
    name: String,
    index: usize,
}

impl Endpoint {
    pub fn new(name: String, kind: EndpointKind, index: usize) -> Endpoint {
        Endpoint { name, kind, index }
    }

    pub fn new_recording(name: String, index: usize) -> Endpoint {
        Endpoint::new(name, EndpointKind::Recording, index)
    }

    pub fn new_playback(name: String, index: usize) -> Endpoint {
        Endpoint::new(name, EndpointKind::Playback, index)
    }
}

impl fmt::Display for EndpointKind {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            EndpointKind::Recording => f.write_str("Recording"),
            EndpointKind::Playback => f.write_str("Playback"),
        }
    }
}

impl fmt::Display for Endpoint {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "Endpoint {{ name: {:?}, kind: {:?}, index: {:?} }}",
            self.name, self.kind, self.index
        )
    }
}
