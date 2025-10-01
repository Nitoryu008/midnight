use bevy::prelude::*;
use midly::num::{u4, u7};

#[derive(Component)]
pub struct Note;

#[derive(Component)]
pub struct Channel(u4);

#[derive(Component)]
pub struct Key(u7);

#[derive(Component)]
pub struct Velocity(u7);

#[derive(Component)]
pub struct Length(f64);

#[derive(Component)]
pub struct StartTime(f64);

#[derive(Bundle)]
pub struct NoteBundle {
    ch: Channel,
    key: Key,
    vel: Velocity,
    length: Length,
    start_time: StartTime,
    note: Note,
}
