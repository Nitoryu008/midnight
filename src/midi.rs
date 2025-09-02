use bevy::{platform::collections::HashSet, prelude::*};
use midly::Smf;

#[derive(Resource)]
pub struct SmfData {
    pub smf: Smf<'static>,
    pub timing_unit: f64,
}

#[derive(Component)]
pub struct Track {
    pub number: usize,
    pub delta_secs: f64,
    pub next_event_index: usize,
}

#[derive(Debug)]
pub struct Channel {
    pub number: u8,
    pub active_notes: HashSet<u8>,
}
#[derive(Component)]
pub struct Channels(pub [Channel; 16]);

impl Channels {
    pub fn new() -> Self {
        let channels = (0..16)
            .map(|i| Channel {
                number: i as u8,
                active_notes: HashSet::new(),
            })
            .collect::<Vec<_>>()
            .try_into()
            .unwrap();

        Self(channels)
    }
}

#[derive(Resource)]
pub struct Tempo {
    secs: f64,
}

impl Tempo {
    pub fn from_secs(secs: f64) -> Self {
        Self { secs }
    }

    pub fn from_bpm(bpm: f64) -> Self {
        Self { secs: 60.0 / bpm }
    }

    /// Returns the duration of a quarter note.
    pub fn secs(&self) -> f64 {
        self.secs
    }

    pub fn set_secs(&mut self, secs: f64) {
        self.secs = secs;
        println!("Tempo(secs): {}", secs);
        println!("Tempo(bpm): {}", self.bpm());
    }

    pub fn bpm(&self) -> f64 {
        60.0 / self.secs
    }

    pub fn set_bpm(&mut self, bpm: f64) {
        self.secs = 60.0 / bpm;
    }
}

pub fn get_delta_secs(timing_unit: f64, delta_time: f64, tempo_secs: f64) -> f64 {
    let delta = delta_time / timing_unit * tempo_secs;
    delta
}
