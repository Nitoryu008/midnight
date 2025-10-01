use std::fs;

use midly::{MetaMessage, MidiMessage, Smf, Timing, TrackEventKind};

use midnight::components::{midi_event::TempoChangeEvent, note::NoteBundle};

/* TODO: 必要なさそうなので消す
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
*/

fn get_delta_secs(timing_unit: f64, delta_time: f64, tempo_secs: f64) -> f64 {
    let delta = delta_time / timing_unit * tempo_secs;
    delta
}

pub fn parse_smf_file(
    file_path: String,
) -> Result<(Vec<NoteBundle>, Vec<TempoChangeEvent>), String> {
    let raw = fs::read("test.mid").expect("cannot open test file").leak();
    let smf = Smf::parse(raw).expect("cannot parse midi file");
    let timing_unit = match smf.header.timing {
        Timing::Metrical(u) => u.as_int() as f64,
        _ => return Err("SMPTE timing is not supported".to_string()),
    };
    let note_bundles = Vec::new();
    let tempo_change_events = Vec::new();

    for track in smf.tracks {
        let mut elapsed_time = 0.;
        let mut tempo_secs = 0.;

        for event in track {
            elapsed_time += get_delta_secs(timing_unit, event.delta.as_int() as f64, tempo_secs);

            match event.kind {
                TrackEventKind::Midi { channel, message } => match message {
                    MidiMessage::NoteOn { key, vel } => {}
                    MidiMessage::NoteOff { key, vel } => {}
                    MidiMessage::ProgramChange { program } => {}
                    MidiMessage::Controller { controller, value } => {}
                    MidiMessage::Aftertouch { key, vel } => {}
                    MidiMessage::ChannelAftertouch { vel } => {}
                    MidiMessage::PitchBend { bend } => {}
                },
                TrackEventKind::SysEx(bytes) => {}
                TrackEventKind::Meta(meta_message) => match meta_message {
                    MetaMessage::Tempo(tempo_micros) => {
                        tempo_secs = tempo_micros.as_int() as f64 / 1000000.0
                    }
                    _ => {}
                },
                TrackEventKind::Escape(bytes) => {}
            }
        }
    }

    Ok((note_bundles, tempo_change_events))
}
