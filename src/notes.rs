use crate::seqdatastruct::{SeqData};

const A4: f64 = 440.0;
const SEMITONES_PER_OCTAVE: usize = 12;
const NOTES_PER_OCTAVE: i32 = 7;
const CENTS_PER_SEMITONE: f64 = 100.0;
const NOTE_NAMES: [&str; SEMITONES_PER_OCTAVE] = [
    "A", "A#", "B","C", "C#", "D", "D#", "E", "F", "F#", "G", "G#"
];

struct Pitch {
    name: String,
    frequency: f64,
    cents: f64
}

impl Pitch {
    pub fn new(name: &str, frequency: f64) -> Pitch {
        Pitch {
            name: String::from(name),
            frequency,
            cents: 0.0
        }
    }

    fn all_notes() -> SeqData<Pitch> {
        let mut notes = SeqData::new();
        let reference_frequency = A4 * 2.0f64.powi(-4);
        for octave in 0..9 {
            for (i, name) in NOTE_NAMES.iter().enumerate() {
                let semitones_above_reference = i + SEMITONES_PER_OCTAVE * octave;
                let frequency = reference_frequency * 2.0f64.powf(semitones_above_reference as f64 / SEMITONES_PER_OCTAVE as f64);
                let fullname = format!("{}{}", name, octave);
                let note = Pitch::new(fullname.as_str(), frequency);
                println!("{} @{}", fullname, frequency);
                notes.add(fullname.as_str(),note);
            }
        }
        notes
    }

    fn guess(notes: SeqData<Pitch>, frequency: f64, last_guess: Option<Pitch>) -> Pitch {
        last_guess.unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_all_notes() {
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
}
