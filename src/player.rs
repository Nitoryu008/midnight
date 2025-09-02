use std::fmt;

use bevy::ecs::resource::Resource;

#[derive(Resource)]
pub struct Playback {
    secs: f64,
    state: PlaybackState,
}

#[derive(Clone, Copy, PartialEq)]
pub enum PlaybackState {
    Stopped,
    Playing,
    Paused,
}

impl Playback {
    pub fn new() -> Self {
        Self {
            secs: 0.,
            state: PlaybackState::Stopped,
        }
    }

    pub fn secs(&self) -> f64 {
        self.secs
    }

    pub fn state(&self) -> PlaybackState {
        self.state
    }

    pub fn stop(&mut self) {
        self.secs = 0.;
        self.state = PlaybackState::Stopped;
    }

    pub fn pause(&mut self) {
        self.state = PlaybackState::Paused;
    }

    pub fn play(&mut self) {
        self.state = PlaybackState::Playing;
    }

    pub fn add_secs(&mut self, delta_secs: f64) {
        self.secs += delta_secs;
    }

    pub fn is_playing(&self) -> bool {
        match self.state {
            PlaybackState::Playing => true,
            _ => false,
        }
    }
}

impl fmt::Display for PlaybackState {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            PlaybackState::Stopped => write!(f, "Stopped"),
            PlaybackState::Playing => write!(f, "Playing"),
            PlaybackState::Paused => write!(f, "Paused"),
        }
    }
}
