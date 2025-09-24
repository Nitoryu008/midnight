use bevy::prelude::*;
use midly::num::{u4, u7};

#[derive(Component)]
struct Note;

#[derive(Component)]
struct Channel(u4);

#[derive(Component)]
struct Key(u7);

#[derive(Component)]
struct Velocity(u7);

#[derive(Component)]
struct Duration(f64);

#[derive(Component)]
struct StartTime(f64);

#[derive(Bundle)]
struct NoteBundle {
    ch: Channel,
    key: Key,
    vel: Velocity,
    duration: Duration,
    start_time: StartTime,
}
