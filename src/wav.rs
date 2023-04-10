use std::{fs::File, f64::consts::PI};
use std::io::Write;

use byteorder::{WriteBytesExt, LittleEndian};

use crate::notes::PhiNote;

pub const SAMPLE_RATE: f64 = 44100.0;

pub fn write_to_file(filename: &str, data: &Vec<u8>) -> std::io::Result<()> {
    let mut file = File::create(filename)?;
    write_wav_header(&mut file, data, 16, 1, SAMPLE_RATE as u32);
    file.flush()?;
    Ok(())
}

pub fn write_wav_header(output: &mut File, data: &Vec<u8>, bits_per_sample: u16, channels: u16, sample_rate: u32) {
    let data_len = data.len() as u32;
    let byte_rate = sample_rate * (bits_per_sample as u32 / 8) * channels as u32;
    let block_align = (bits_per_sample / 8) * channels;
    
    std::io::Write::write_all(output,b"RIFF").unwrap();
    WriteBytesExt::write_u32::<LittleEndian>(output, 36 + data_len).unwrap();
    std::io::Write::write_all(output,b"WAVE").unwrap();
    std::io::Write::write_all(output,b"fmt ").unwrap();
    WriteBytesExt::write_u32::<LittleEndian>(output, 16).unwrap();
    WriteBytesExt::write_u16::<LittleEndian>(output, 1).unwrap();
    WriteBytesExt::write_u16::<LittleEndian>(output, channels).unwrap();
    WriteBytesExt::write_u32::<LittleEndian>(output, sample_rate).unwrap();
    WriteBytesExt::write_u32::<LittleEndian>(output, byte_rate).unwrap();
    WriteBytesExt::write_u16::<LittleEndian>(output, block_align).unwrap();
    WriteBytesExt::write_u16::<LittleEndian>(output, bits_per_sample).unwrap();
    std::io::Write::write_all(output,b"data").unwrap();
    WriteBytesExt::write_u32::<LittleEndian>(output, data_len).unwrap();
    std::io::Write::write_all(output,data).unwrap();
}


pub enum Oscilator {
    SINE,
    SQUARE,
    SAWTOOTH,
    TRIANGLE,
}

pub fn generate_wav(note: &PhiNote, oscilator: Oscilator) -> Vec<u8> {
    let samples_per_cycle = SAMPLE_RATE / note.pitch.frequency();
    let number_of_samples = ((note.end - note.start) * SAMPLE_RATE) as usize;
    let max_amplitude = i16::MAX;
    let min_amplitude = i16::MIN;

    let mut wave = vec![0i16; number_of_samples];

    //println!("Start generating {} samples with {} samples_per_cycle min={} max={}", number_of_samples, samples_per_cycle, min_amplitude, max_amplitude);

    match oscilator {
        Oscilator::SINE => {
            // generate a sine wave
            for i in 0..number_of_samples {
                let angle = 2.0 * PI  * (i as f64 / samples_per_cycle);
                let amplitude = (angle.sin() * max_amplitude as f64) as i16;
                wave[i] = amplitude;
            }
        },
        Oscilator::SQUARE => {
            // generate a square wave
            let step = ((max_amplitude as f64 - min_amplitude as f64) / samples_per_cycle as f64) as i16;
            let mut amplitude = min_amplitude;
            for i in 0..number_of_samples {
                if i as f64 % (samples_per_cycle).round() == 0.0 {
                    amplitude = min_amplitude;
                }
                wave[i] = amplitude;
                if step as i32 + amplitude as i32 <= i16::MAX as i32 {
                    amplitude += step;
                }
            }
        },
        Oscilator::SAWTOOTH => {
            // generate a sawtooth wave
            let step = ((max_amplitude as f64 - min_amplitude as f64) / samples_per_cycle as f64) as i16;
            let mut amplitude = min_amplitude;
            for i in 0..number_of_samples {
                if i as f64 % (samples_per_cycle).round() == 0.0 {
                    amplitude = min_amplitude;
                }
                wave[i] = amplitude;
                amplitude += step;
            }
        },
        Oscilator::TRIANGLE => {
            // generate a triangle wave
            let step = ((max_amplitude as f64 - min_amplitude as f64) / samples_per_cycle as f64) as i16;
            let mut amplitude = min_amplitude;
            let mut up = true;
            for i in 0..number_of_samples {
                if i as f64 % (samples_per_cycle).round() == 0.0 {
                    amplitude = min_amplitude;
                    up = true;
                }
                wave[i] = amplitude;
                if up {
                    amplitude += step;
                    if amplitude >= max_amplitude {
                        up = false;
                    }
                } else {
                    amplitude -= step;
                }
            }
        }
    }

    wave.iter().flat_map(|&x| x.to_le_bytes().to_vec()).collect()
}

