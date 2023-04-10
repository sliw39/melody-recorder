use std::fmt::Display;

use serde::{Serialize, Deserialize};

use crate::{tuning::std_tuning, seqdatastruct::SeqData};

pub const A4: f64 = 440.0;
pub const SEMITONES_PER_OCTAVE: usize = 12;
pub const NOTES_PER_OCTAVE: i32 = 7;
pub const CENTS_PER_SEMITONE: f64 = 100.0;
pub const NOTE_NAMES: [&str; SEMITONES_PER_OCTAVE] = [
    "C", "C#", "D", "D#", "E", "F", "F#", "G", "G#", "A", "A#", "B"
];

#[derive(Clone, Serialize, Deserialize)]
pub struct Pitch {
    name: String,
    frequency: f64,
    cents: f64,
    midi: usize
}

impl Pitch {
    pub fn new(name: &str, frequency: f64, midi: usize) -> Pitch {
        Pitch {
            name: String::from(name),
            frequency,
            cents: 0.0,
            midi
        }
    }

    pub fn silence() -> Pitch {
        Pitch {
            name: String::from("S"),
            frequency: 0.0,
            cents: 0.0,
            midi: 0
        }
    }

    pub fn all_notes() -> SeqData<Pitch> {
        // call std_tuning to get a map of all notes
        let mut notes_map = std_tuning();
        let mut notes = SeqData::new();

        for octave in 0..10 {
            for (i, name) in NOTE_NAMES.iter().enumerate() {
                let semitones_above_reference = i + SEMITONES_PER_OCTAVE * octave;
                // skip notes below A0
                if semitones_above_reference < 9 {
                    continue;
                }

                let fullname = format!("{}{}", name, octave);
                let midi = semitones_above_reference + 12;
                let note = Pitch::new(fullname.as_str(), notes_map[fullname.as_str()], midi);
                notes.add(fullname.as_str(),note);
            }
        }
        notes
    }

    pub fn guess(notes: &SeqData<Pitch>, frequency: f64, last_guess: Option<Pitch>) -> Option<Pitch> {
        let reference = last_guess.unwrap_or_else(|| Pitch::new("A4", A4, 69));
        if frequency < reference.frequency {
            let mut start = notes.iter_backward(&reference.name).unwrap();
            let mut last_note = &reference;
            while let Some(note) = start.next() {
                if frequency > note.frequency {
                    let mut note_res: Pitch;
                    if (last_note.frequency - frequency).abs() < (note.frequency - frequency).abs() {
                        note_res = last_note.clone();
                    } else {
                        note_res = note.clone();
                    }
                    note_res.frequency = frequency;
                    return Some(note_res);
                }
                last_note = note;
            }
        } else {
            let mut start = notes.iter_forward(&reference.name).unwrap();
            let mut last_note = &reference;
            while let Some(note) = start.next() {
                if frequency < note.frequency {
                    let mut note_res: Pitch;
                    if (last_note.frequency - frequency).abs() < (note.frequency - frequency).abs() {
                        note_res = last_note.clone();
                    } else {
                        note_res = note.clone();
                    }
                    note_res.frequency = frequency;
                    return Some(note_res);
                }
                last_note = note;
            }
        }
        return None
    }

    pub fn frequency(&self) -> f64 {
        self.frequency
    }

    pub fn name(&self) -> &str {
        self.name.as_ref()
    }

    pub fn cents(&self) -> f64 {
        self.cents
    }

    pub fn midi(&self) -> usize {
        self.midi
    }

    pub fn from_str(name: &str) -> Option<Pitch> {
        let notes = Pitch::all_notes();
        notes.get(name).cloned()
    }
}

impl PartialEq<Pitch> for Pitch {
    fn eq(&self, note: &Pitch) -> bool { 
        self.name == note.name
    }
}

impl Display for Pitch {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}@{} {}", self.name, self.frequency.round(), self.cents)
    }
}

#[derive(Clone, Serialize, Deserialize)]
pub struct PhiNote {
    pub pitch: Pitch,
    pub start: f64,
    pub end: f64,
}

impl Display for PhiNote {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[{}->{} : {}]", self.start, self.end, self.pitch)
    }
}

impl PhiNote {
    pub fn time_offset(&mut self, value: f64) {
        self.start += value;
        self.end += value;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generated_notes() {
        let notes = Pitch::all_notes();
        assert_eq!(27.5, notes.get("A0").unwrap().frequency);
        assert_eq!(55.0, notes.get("A1").unwrap().frequency);
        assert_eq!(110.0, notes.get("A2").unwrap().frequency);
        assert_eq!(220.0, notes.get("A3").unwrap().frequency);
        assert_eq!(440.0, notes.get("A4").unwrap().frequency);
        assert_eq!(880.0, notes.get("A5").unwrap().frequency);
        assert_eq!(1760.0, notes.get("A6").unwrap().frequency);
        assert_eq!(3520.0, notes.get("A7").unwrap().frequency);
    }

    #[test]
    fn test_guess() {
        let dataset  = [
            ("C2", "A5", 874.0),
            ("B5", "A5", 874.0),
            ("A2", "A5", 880.0),
            ("A4", "A4", 440.0)
        ];
        let notes = Pitch::all_notes();

        for data in dataset {
            let guessed = Pitch::guess(&notes, data.2, notes.get(data.0).cloned());
            assert_eq!(data.1, guessed.unwrap().name);
        }
    }
}
