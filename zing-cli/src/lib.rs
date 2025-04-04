use std::time::Duration;
use zing_protocol::Chord;

pub use error::{Error, Result};

pub mod error;

const OCTAVES: usize = 9;

const OCTAVES_A: [u16; OCTAVES] = [27, 55, 110, 220, 440, 880, 1760, 3520, 7040];
const OCTAVES_A_SHARP: [u16; OCTAVES] = [29, 58, 116, 233, 466, 932, 1864, 3729, 7458];
const OCTAVES_B: [u16; OCTAVES] = [30, 61, 123, 246, 493, 987, 1975, 3951, 7902];
const OCTAVES_C: [u16; OCTAVES] = [16, 32, 65, 130, 261, 523, 1046, 2093, 4186];
const OCTAVES_C_SHARP: [u16; OCTAVES] = [17, 34, 69, 138, 277, 554, 1108, 2217, 4434];
const OCTAVES_D: [u16; OCTAVES] = [18, 36, 73, 146, 293, 587, 1174, 2349, 4698];
const OCTAVES_D_SHARP: [u16; OCTAVES] = [19, 38, 77, 155, 311, 622, 1244, 2489, 4978];
const OCTAVES_E: [u16; OCTAVES] = [20, 41, 82, 164, 329, 659, 1318, 2637, 5274];
const OCTAVES_F: [u16; OCTAVES] = [21, 43, 87, 174, 349, 698, 1396, 2793, 5587];
const OCTAVES_F_SHARP: [u16; OCTAVES] = [23, 46, 92, 185, 369, 739, 1479, 2959, 5919];
const OCTAVES_G: [u16; OCTAVES] = [24, 49, 98, 196, 392, 783, 1567, 3135, 6271];
const OCTAVES_G_SHARP: [u16; OCTAVES] = [25, 51, 103, 207, 415, 830, 1661, 3322, 6644];

pub fn chords_from_notemap(notemap: &str, chord_duration: &Duration) -> Result<Vec<Chord>> {
    let sections = get_sections(notemap);
    let chords: Vec<Chord> = get_all_chords(sections)?;
    Ok(compress_chords(chords, chord_duration))
}

fn compress_chords(mut chords: Vec<Chord>, chord_duration: &Duration) -> Vec<Chord> {
    let mut last_valid_index: usize = 0;
    let mut racked_up_time: Duration = Duration::ZERO;

    // After having filled all chords, we count the none valid ones, add those times to the valid ones, and filter out the non valid ones
    for index in 0..chords.len() {
        if index > 0 && chords[index].notes.is_empty() {
            racked_up_time += *chord_duration;
            continue;
        }

        chords[last_valid_index].extended_duration += racked_up_time;
        racked_up_time = Duration::ZERO;
        last_valid_index = index;
    }

    // Filter the empty chords, since the full ones now have the time that the empty ones occupied
    chords.retain(|chord| !chord.notes.is_empty());
    chords
}

/// Find all the sections by normalizing the newlines and splitting them by double newlines
fn get_sections(notemap: &str) -> Vec<String> {
    let binding = notemap.replace("\r\n", "\n");
    binding
        .split("\n\n")
        .map(str::trim)
        .filter(|s| !s.is_empty())
        .map(String::from)
        .collect()
}

/// Computes all of the chords, disgregarding accumulated time
fn get_all_chords(sections: Vec<String>) -> Result<Vec<Chord>> {
    let mut chords: Vec<Chord> = Vec::new();

    // Go through all the sections to parse the lines into notes, octaves and chords
    for section in sections {
        let mut section_chords = get_chords(&section)?;
        chords.append(&mut section_chords);
    }

    Ok(chords)
}

fn get_chords(section: &str) -> Result<Vec<Chord>> {
    let lines = section.split('\n');

    let mut section_chords: Vec<Chord> = Vec::new();

    for line in lines {
        // Remove the left-hand, right-hand part, it is irrelevant
        let value = match line.split(':').last() {
            Some(val) => val.trim(),
            None => continue,
        };

        if value.is_empty() {
            continue;
        }

        // The actual octave and note parts
        let parts: Vec<&str> = value
            .split('|')
            .map(str::trim)
            .filter(|s| !s.is_empty())
            .collect();

        // Extract the octave from the first part
        let octave: usize = match parts.first().ok_or(Error::OctaveNotSpecified)?.parse() {
            Ok(octave) => octave,
            Err(e) => {
                return Err(Error::InvalidOctave(e.to_string()));
            }
        };

        // Get the notes
        let Some(notes) = parts.last() else {
            return Err(Error::NotesNotSpecified);
        };

        // Resize with default chords to the size of the notes
        section_chords.resize_with(notes.len(), Chord::default);

        // Go through all the notes and add them to the respecive chords
        for (index, note) in notes.as_bytes().iter().enumerate() {
            // Ignore dashes, they are not valid notes
            if *note == b'-' {
                continue;
            }

            section_chords[index]
                .notes
                .push(get_note(char::from(*note), octave)?); // Finally, add the note to the respective octave
        }
    }

    Ok(section_chords)
}

fn get_note(char: char, octave: usize) -> Result<u16> {
    if octave >= OCTAVES {
        return Err(Error::OctaveDoesNotExist(octave));
    }

    match char {
        'a' => Ok(OCTAVES_A[octave]),
        'A' => Ok(OCTAVES_A_SHARP[octave]),

        'b' => Ok(OCTAVES_B[octave]),

        'c' => Ok(OCTAVES_C[octave]),
        'C' => Ok(OCTAVES_C_SHARP[octave]),

        'd' => Ok(OCTAVES_D[octave]),
        'D' => Ok(OCTAVES_D_SHARP[octave]),

        'e' => Ok(OCTAVES_E[octave]),

        'f' => Ok(OCTAVES_F[octave]),
        'F' => Ok(OCTAVES_F_SHARP[octave]),

        'g' => Ok(OCTAVES_G[octave]),
        'G' => Ok(OCTAVES_G_SHARP[octave]),

        _ => Err(Error::NoteDoesNotExist(char)),
    }
}
