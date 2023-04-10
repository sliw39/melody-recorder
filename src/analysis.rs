use rustfft::{FftPlanner, num_complex::Complex};
use serde::{Serialize, Deserialize};

use std::{convert::TryInto};

use crate::{notes::{Pitch, PhiNote}, wav::{generate_wav, SAMPLE_RATE}};

const THRESHOLD_DB: f64 = 60.0;
const CHUNK_SIZE: f64 = 1.0;

pub fn is_sample_below_threshold(samples: &Vec<f64>) -> bool {
    let rms = (samples.iter().map(|x| x * x).sum::<f64>() / (samples.len() as f64)).sqrt();
    let rms_db = 20.0 * rms.log10();
    rms_db < THRESHOLD_DB
}


pub fn split_and_process_wav_chunk(wav_chunk: &[u8], sample_rate: u32, channels: u16, bits_per_sample: u16) -> Vec<f64> {
    let bytes_per_sample = bits_per_sample/8;
    let chunk_size = (CHUNK_SIZE * sample_rate as f64 * bytes_per_sample as f64) as usize;

    let mut samples = Vec::new();
    for i in (0..wav_chunk.len()).step_by(chunk_size) {
        // if we are at the end of the chunk, we need to break 
        if i + chunk_size > wav_chunk.len() {
            break;
        }
        let chunk_u8 = &wav_chunk[i..i+chunk_size];

        let mut sample_chunk = Vec::new();
        for j in (0..chunk_u8.len()).step_by((bytes_per_sample).into()) {
            let sample = &chunk_u8[j..j + bytes_per_sample as usize];
            let sample = i16::from_le_bytes(sample.try_into().unwrap()) as f64;
            sample_chunk.push(sample);
        }
        if is_sample_below_threshold(&sample_chunk) {
           samples.push(0.0);
        } else {
            let fundamental_frequency = get_fundamental_frequency(&sample_chunk).unwrap();
            samples.push(fundamental_frequency);
        }
    }

    samples
}

pub fn get_fundamental_frequency(samples: &Vec<f64>) -> Option<f64> {
    let n = samples.len();
    let bin = SAMPLE_RATE / n as f64;
    let mut data: Vec<Complex<f64>> = samples
        .iter()
        .map(|&x| Complex::new(x, 0.0))
        .collect();

    let mut planner = FftPlanner::new();
    let fft = planner.plan_fft_forward(n);
    fft.process(&mut data);

    // take the fist half of data, sort it by magnitude and take the 5 firsts elements
    let elements = data.iter().take(n/2).map(|x| x.norm()).collect::<Vec<_>>();
    // convert to a vector of tuples (index, value)
    let mut tuples = elements.iter().enumerate().collect::<Vec<_>>();
    // sort frequencies by magnitude
    tuples.sort_by(|(_, a), (_, b)| b.partial_cmp(a).unwrap());
    // take the 5 firsts elements, extract their key and multiply by bin. store the result into a vector
    let elements = tuples.iter().take(5).map(|(i, _)| *i as f64 * bin).collect::<Vec<_>>();
    println!("top freqs : {:?}", elements);

    Some(elements[0])

}

pub fn analyze_chunk(chunk: &[u8]) -> Chunk {
    // Actual implementation of the WAV data analysis goes here
    let mut result = Chunk {
        notes: vec![]
    };
    let frequencies = split_and_process_wav_chunk(chunk, 44100, 1, 16);
    let all_notes = Pitch::all_notes();
    let last_guess = all_notes.get("A4");
    let pitches: Vec<Pitch> = frequencies.iter().map(|f| {
        if *f == 0.0 {
            return Pitch::silence();
        } else {
            return Pitch::guess(&all_notes, *f, last_guess.cloned()).unwrap_or_else(|| Pitch::silence());
        }
    }).collect();
    
    let mut time_cursor = 0.0;
    let mut current_note = PhiNote{
        pitch: pitches[0].clone(),
        start: time_cursor,
        end: 0.0
    };

    for pitch in pitches {
        time_cursor += CHUNK_SIZE;
        if current_note.pitch != pitch {
            current_note.end = time_cursor;
            result.notes.push(current_note);
            current_note = PhiNote {
                pitch,
                start: time_cursor,
                end: 0.0
            }
        }
    }
    current_note.end = time_cursor;
    result.notes.push(current_note);

    return result;
}


#[derive(Clone, Serialize, Deserialize)]
pub struct Chunk {
    pub notes: Vec<PhiNote>
}

impl Chunk {

    pub fn from_str(string: &str) -> Chunk {
        let mut notes = Vec::new();
        let mut time_cursor = 0.0;
        for line in string.lines() {
            // skip empty lines and lines starting with #
            if line.is_empty() || line.starts_with("#") {
                continue;
            }

            let mut parts = line.split(" ");
            let pitch = parts.next().unwrap();
            let duration = 1.0 / parts.next().unwrap().parse::<f64>().unwrap();
            let note = PhiNote {
                pitch: Pitch::from_str(pitch).unwrap(),
                start: time_cursor,
                end: time_cursor + duration
            };
            time_cursor += duration;
            notes.push(note);
        }
        Chunk {
            notes
        }
    }
}


#[cfg(test)]
mod tests {
    use std::{fs::File, io::Write};

    use byteorder::{LittleEndian, WriteBytesExt};

    use crate::wav::Oscilator;

    use super::*;

    impl Chunk {
        pub fn to_wav(&self) -> Vec<u8> {
            let mut wav = Vec::new();
            for note in &self.notes {
                let mut note_wav = generate_wav(note, Oscilator::SINE);
                wav.append(&mut note_wav);
            }
            wav
        }
    }

    #[test]
    fn test_generated_notes() -> Result<(), String> {
        let notes = Pitch::all_notes();
        let mut item = notes.iter_backward("A9").unwrap();
        while let Some(note) = item.next() {
            if note.name() == "A0" {
                break;
            }

            let wav = generate_wav(&PhiNote { 
                pitch: note.clone(), 
                start: 0.0, 
                end: 1.0 
            }, Oscilator::SINE);

            //write_to_file(&format!("test_{}.wav", note.name()), &wav).unwrap();
    
            let chunk = analyze_chunk(&wav);
    
            assert_eq!(1, chunk.notes.len());
            println!("given {}, got {}", note, chunk.notes.iter().map(|x| format!("{}", x)).collect::<String>());
            assert_eq!(note.name(), chunk.notes[0].pitch.name());
        }

        Ok(())
    }

    // generate a melody and analyse it
    #[test]
    fn test_generated_melody() -> Result<(), String> {
        let melody = Chunk::from_str("C4 4\nC4 4\nC4 4\nD4 4\nE4 2\nD4 2\nC4 4\nE4 4\nD4 4\nD4 4\nC4 2");

        //write_to_file("test_melody.wav", &melody.to_wav()).unwrap();

        let chunk = analyze_chunk(&melody.to_wav());

        assert_eq!(melody.notes.len(), chunk.notes.len());
        for i in 0..melody.notes.len() {
            println!("given {}, got {}", melody.notes[i], chunk.notes[i]);
            assert_eq!(melody.notes[i].pitch.name(), chunk.notes[i].pitch.name());
            assert_eq!(melody.notes[i].start, chunk.notes[i].start);
            assert_eq!(melody.notes[i].end, chunk.notes[i].end);
        }

        Ok(())
    }

}

