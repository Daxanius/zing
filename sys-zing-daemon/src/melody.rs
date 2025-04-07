use crate::{Error, Result};
use beep::beep;
use std::time::Duration;
use zing_protocol::{Chord, PlayData};

pub struct Melody {
    chord_duration: Duration,
    chords: Vec<Chord>,
    position: usize,
    is_playing: bool,
    was_stopped: bool,
}

impl Melody {
    pub fn from_data(data: PlayData) -> Result<Self> {
        if data.chords.is_empty() {
            return Err(Error::NoChordsProvided);
        }

        Ok(Self {
            position: 0,
            is_playing: false,
            was_stopped: false,
            chord_duration: data.chord_duration,
            chords: data.chords,
        })
    }

    #[must_use]
    pub fn is_playing(&self) -> bool {
        self.is_playing
    }

    pub fn resume(&mut self) {
        self.is_playing = true;
    }

    pub fn pause(&mut self) {
        self.is_playing = false;
        let _ = beep(0);
    }

    #[must_use]
    pub fn is_finished(&self) -> bool {
        self.position >= self.chords.len() - 1
    }

    #[must_use]
    pub fn was_stopped(&self) -> bool {
        self.was_stopped
    }

    pub fn stop(&mut self) {
        self.is_playing = false;
        self.was_stopped = true;
        self.position = 0;
        let _ = beep(0);
    }

    pub fn next_chord(&mut self) {
        self.position = (self.position + 1) % self.chords.len();
    }

    #[must_use]
    pub fn get_chord(&self) -> Chord {
        self.chords[self.position].clone()
    }

    #[must_use]
    pub fn get_chord_duration(&self) -> Duration {
        self.chord_duration
    }
}
