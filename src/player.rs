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
}
