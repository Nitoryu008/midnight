use bevy::ecs::resource::Resource;

#[derive(Resource)]
pub struct Playback {
    secs: f64,
    state: PlaybackState,
}

enum PlaybackState {
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

    pub fn set_secs(&mut self, secs: f64) {
        self.secs = secs;
        self.state = PlaybackState::Stopped;
    }

    pub fn secs(&self) -> f64 {
        self.secs
    }

    pub fn stop(&mut self) {
        self.secs = 0.;
    }

    pub fn pause(&mut self) {
        self.state = PlaybackState::Paused;
    }

    pub fn add_secs(&mut self, delta_secs: f64) {
        self.secs += delta_secs;
    }
}
