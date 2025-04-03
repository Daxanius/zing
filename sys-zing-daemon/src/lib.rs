use std::sync::{ Arc, RwLock };
use std::thread::{ self, JoinHandle };
use log::{ info, trace, error, warn };
use std::time::Duration;
use zing_protocol::Chord;
use zing_protocol::Command;
use zing_protocol::Command::*;
use crate::melody::Melody;
use beep::beep;

pub use error::{ Error, Result };

pub mod melody;
pub mod error;

pub struct MelodyPlayer {
    melody: Option<Arc<RwLock<Melody>>>,
    play_handle: Option<JoinHandle<()>>,
}

impl MelodyPlayer {
    pub fn new() -> Self {
        Self {
            melody: None,
            play_handle: None
        }
    }

    pub fn handle_command(&mut self, command: Command) {
        trace!("Processing command");

        match command {
            Play(data) => {
                if let Ok(melody) = Melody::from_data(data) {
                    self.play(melody).unwrap_or_else(|e| error!("{e}"));
                } else {
                    error!("Failed to create melody");
                }
            },
            Stop => self.stop().unwrap_or_else(|e| error!("{e}")),
            Pause => self.pause().unwrap_or_else(|e| error!("{e}")),
            Resume => self.resume().unwrap_or_else(|e| error!("{e}")),
        };
    }

    pub fn play(&mut self, mut melody: Melody) -> Result<()> {
        trace!("Playing melody");

        // Stop any previous songs from playing
        self.stop()?;

        // Make sure the melody can play in the first instance
        melody.resume();

        let melody_ref = Arc::new(RwLock::new(melody));
        self.melody = Some(melody_ref.clone());
        self.play_handle = Some(thread::spawn(|| Self::play_melody(melody_ref)));
        info!("Started melody");

        Ok(())
    }

    pub fn stop(&mut self) -> Result<()> {
        trace!("Stopping melody");

        if let Some(melody) = &self.melody {
             let mut melody = match melody.write() {
                Ok(melody) => melody,
                Err(_) => {
                    return Err(Error::LockPoisoned);
                }
            };
            
            melody.stop();
            info!("Melody stopped");
        } else {
            info!("No melody to stop");
        }

        trace!("Joining thread");
        if let Some(play_handle) = self.play_handle.take() {
            play_handle.join().map_err(|_| Error::CouldNotJoinThread)?;
            info!("Thread joined"); 
        } else {
            info!("No thread to join");
        }

        Ok(())
    }

    pub fn pause(&mut self) -> Result<()> {
        trace!("Pausing melody");

        if let Some(melody) = &self.melody {
             let mut melody = match melody.write() {
                Ok(melody) => melody,
                Err(_) => {
                    return Err(Error::LockPoisoned);
                }
            };
            
            melody.pause();
            info!("Melody paused");
        } else {
            info!("No melody to pause");
        }

        Ok(())
    }
 
    pub fn resume(&mut self) -> Result<()> {
        trace!("Resuming melody");

        if let Some(melody) = &self.melody {
             let mut melody = match melody.write() {
                Ok(melody) => melody,
                Err(_) => {
                    return Err(Error::LockPoisoned);
                }
            };
            
            melody.resume();
            info!("Melody resumed");
        } else {
            info!("No melody to resume");
        }

        Ok(())
    }

    fn play_melody(melody: Arc<RwLock<Melody>>) {
        loop {
            let chord;
            let chord_duration;

            // Melody reader has its own scope
            {
                let melody = match melody.read() {
                    Ok(melody) => melody,
                    Err(e) => {
                        error!("Could not read melody: {}", e.to_string());
                        return;
                    }
                };

                if melody.was_stopped() {
                    info!("Melody stopped");
                    return;
                }
                
                if melody.is_finished() {
                    info!("Melody finished");
                    return;
                }

                if !melody.is_playing() && !melody.was_stopped() {
                    continue;
                }
               
                chord = melody.get_chord();
                chord_duration = melody.get_chord_duration();
            }

            match Self::play_chord(chord, chord_duration) {
                Ok(_) => (),
                Err(e) => warn!("Could not play chord: {e}"),
            }

            // After playing a note, grab a write lock and increment its position counter
            let mut melody = match melody.write() {
                Ok(melody) => melody,
                Err(e) => {
                    error!("Could not lock melody: {}", e.to_string());
                    return;
                }
            };

            melody.next_chord();
        }
    }

    fn play_chord(chord: Chord, chord_duration: Duration) -> Result<()> {
        // Play the chord by quickly iterating over the notes
        for note in &chord.notes {
            beep(*note).map_err(|e| Error::Beep(e))?;
            thread::sleep(chord_duration / chord.notes.len().try_into().map_err(|_| Error::Convert)?);
        }

        // If a note is played out longer, extend the last note
        if let Some(note) = chord.notes.last() {
            beep(*note).map_err(|e| Error::Beep(e))?;
            thread::sleep(chord.extended_duration);
        }

        // Stop playing the note
        beep(0).map_err(|e| Error::Beep(e))?;
        Ok(())
    }
}
