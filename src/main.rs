use bevy::prelude::*;
use midly::TrackEvent;
use midly::{MetaMessage, MidiMessage, Smf, Timing, TrackEventKind};
use std::fs;
use std::str;

mod midi;
use midi::{Channel, Channels, Tempo, Track};
mod player;
use player::Playback;

use crate::player::PlaybackState;

fn main() {
    let raw = fs::read("test.mid").expect("cannot open test file").leak();
    let smf = Smf::parse(raw).expect("cannot parse midi file");
    let timing_unit = match smf.header.timing {
        Timing::Metrical(u) => u.as_int(),
        _ => panic!("SMPTE timing is not supported"),
    };

    App::new()
        .add_plugins(DefaultPlugins)
        .insert_resource(SmfData {
            smf,
            timing_unit: timing_unit as f64,
        })
        .insert_resource(midi::Tempo::from_bpm(120.0))
        .insert_resource(Playback::new())
        .add_systems(Startup, setup)
        .add_systems(
            Update,
            (update_midi_events, update_key_events, update_texts).chain(),
        )
        .run();
}

#[derive(Resource)]
struct SmfData {
    smf: Smf<'static>,
    timing_unit: f64,
}

#[derive(Component)]
struct ChannelText(u8);

#[derive(Component)]
struct TempoText;

#[derive(Component)]
struct Note {
    key: u8,
    vel: u8,
}

fn setup(mut commands: Commands, smf_data: Res<SmfData>, tempo: Res<Tempo>, time: Res<Time>) {
    commands.spawn(Camera2d);

    let mut title: &str = "";

    for (index, track) in smf_data.smf.tracks.iter().enumerate() {
        for event in track {
            match event.kind {
                TrackEventKind::Meta(meta) => match meta {
                    MetaMessage::TrackName(track_name) => {
                        let track_name = str::from_utf8(track_name).expect("invalid track name");
                        println!("track name: {}", track_name);
                        if title == "" {
                            title = track_name;
                        }
                    }
                    _ => {}
                },
                _ => {}
            }
        }

        commands.spawn(Track {
            number: index,
            delta_secs: 0.,
            next_event_index: 0,
        });
    }

    commands.spawn((
        Text::new(format!("Title: {}", title)),
        TextFont {
            font_size: 16.0,
            ..default()
        },
        TextShadow::default(),
        TextLayout::new_with_justify(JustifyText::Left),
        Node {
            position_type: PositionType::Absolute,
            top: Val::Px(5.0),
            left: Val::Px(5.0),
            ..default()
        },
    ));

    for i in 0..16 {
        commands
            .spawn((
                Text::new(format!("Ch {0: >02}: ", i)),
                TextFont {
                    font_size: 16.0,
                    ..default()
                },
                TextShadow::default(),
                TextLayout::new_with_justify(JustifyText::Left),
                Node {
                    position_type: PositionType::Absolute,
                    top: Val::Px(25.0 + 20.0 * (i as f32)),
                    left: Val::Px(5.0),
                    ..default()
                },
            ))
            .with_child((
                TextSpan::default(),
                TextFont {
                    font_size: 16.0,
                    ..default()
                },
                ChannelText(i),
            ));
    }

    commands.spawn(Channels::new());

    commands
        .spawn((
            Text::new("Tempo: "),
            TextFont {
                font_size: 16.0,
                ..default()
            },
            TextShadow::default(),
            TextLayout::new_with_justify(JustifyText::Left),
            Node {
                position_type: PositionType::Absolute,
                top: Val::Px(25.0 + 20.0 * 17.0),
                left: Val::Px(5.0),
                ..default()
            },
        ))
        .with_child((
            TextSpan::new(format!("{:.2}", tempo.bpm())),
            TextFont {
                font_size: 16.0,
                ..default()
            },
            TempoText,
        ));
}

fn update_key_events(keys: Res<ButtonInput<KeyCode>>, mut playback: ResMut<Playback>) {
    if keys.just_pressed(KeyCode::Space) {
        match playback.state() {
            PlaybackState::Playing => {
                playback.pause();
            }
            PlaybackState::Paused | PlaybackState::Stopped => {
                playback.play();
            }
        }
    }
}

fn update_midi_events(
    mut commands: Commands,
    smf_data: ResMut<SmfData>,
    time: Res<Time>,
    mut channels: Query<&mut Channels>,
    tracks: Query<&mut Track>,
    mut tempo: ResMut<Tempo>,
    mut playback: ResMut<Playback>,
) {
    let channels = &mut channels.single_mut().unwrap().0;

    for mut track in tracks {
        let events: &Vec<TrackEvent<'static>> = &smf_data.smf.tracks[track.number];
        track.delta_secs += time.delta_secs_f64();

        loop {
            if track.next_event_index >= events.len() {
                break;
            }

            let event = &events[track.next_event_index];
            let midi_delta_secs = midi::get_delta_secs(
                smf_data.timing_unit,
                event.delta.as_int() as f64,
                tempo.secs(),
            );

            if track.delta_secs >= midi_delta_secs {
                on_track_event(event, channels, &mut tempo);
                track.next_event_index += 1;
                track.delta_secs -= midi_delta_secs;
            } else {
                break;
            }
        }
    }
}

fn on_track_event(event: &TrackEvent, channels: &mut [Channel; 16], tempo: &mut ResMut<Tempo>) {
    match event.kind {
        TrackEventKind::Midi { channel, message } => {
            let channel = &mut channels[channel.as_int() as usize];
            print!("Channel {} : ", channel.number);
            match message {
                MidiMessage::NoteOn { key, vel } => {
                    if vel == 0 {
                        channel.active_notes.remove(&key.as_int());
                    } else {
                        channel.active_notes.insert(key.as_int());
                    }
                    println!("ON {} Vel {}", key, vel);
                }
                MidiMessage::NoteOff { key, vel } => {
                    channel.active_notes.remove(&key.as_int());
                    println!("OFF {} Vel {}", key, vel);
                }
                MidiMessage::ProgramChange { program } => {
                    println!("Program Change");
                }
                MidiMessage::Aftertouch { key, vel } => {
                    println!("Aftertouch {}", key);
                }
                MidiMessage::ChannelAftertouch { vel } => {
                    println!("ChannelAftertouch");
                }
                MidiMessage::Controller { controller, value } => {
                    println!("Controller {} {}", controller, value);
                }
                MidiMessage::PitchBend { bend } => {
                    println!("PitchBend");
                }
            }
        }
        TrackEventKind::Meta(meta_message) => match meta_message {
            MetaMessage::Tempo(tempo_micros) => {
                tempo.set_secs(tempo_micros.as_int() as f64 / 1000000.0);
            }
            MetaMessage::TrackName(track_name) => {}
            _ => {
                println!("MetaMessage");
            }
        },
        _ => {}
    }
}

fn update_texts(
    channel_text: Query<(&mut TextSpan, &ChannelText)>,
    channels: Query<&Channels>,
    mut tempo_text: Query<&mut TextSpan, (With<TempoText>, Without<ChannelText>)>,
    tempo: Res<Tempo>,
) {
    let channels = &channels.single().unwrap().0;

    for (mut text_span, channel_text) in channel_text {
        **text_span = channels[channel_text.0 as usize]
            .active_notes
            .iter()
            .map(|&n| format!("{}", n))
            .collect::<Vec<String>>()
            .join(", ");
    }

    let mut tempo_text = tempo_text.single_mut().unwrap();
    **tempo_text = format!("{:.2}", tempo.bpm());
}
