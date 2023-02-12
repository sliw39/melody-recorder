use rustfft::{FftPlanner, num_complex::Complex};

use std::convert::TryInto;

const THRESHOLD_DB: f64 = 60.0;
const SAMPLE_RATE: f64 = 44100.0;

pub fn is_sample_below_threshold(samples: &[f64]) -> bool {
    let rms = samples
        .iter()
        .map(|x| x * x)
        .sum::<f64>()
        / (samples.len() as f64);
    rms < 10.0_f64.powf(THRESHOLD_DB / 20.0)
}

pub fn split_and_process_wav_chunk(wav_chunk: &[u8]) -> Vec<f64> {
    let sample_size = 2; // 16-bit samples
    let chunk_size = (SAMPLE_RATE * sample_size as f64 * 0.02) as usize; // 20ms

    let mut samples = Vec::new();
    for i in (0..wav_chunk.len()).step_by(chunk_size) {
        let chunk = &wav_chunk[i..i + chunk_size];
        let mut sample_chunk = Vec::new();
        for j in (0..chunk.len()).step_by(sample_size) {
            let sample = &chunk[j..j + sample_size];
            let sample = i16::from_le_bytes(sample.try_into().unwrap()) as f64 / 32768.0;
            sample_chunk.push(sample);
        }
        let fundamental_frequency = get_fundamental_frequency(&sample_chunk);
        samples.push(fundamental_frequency);
    }

    samples
}


pub fn get_fundamental_frequency(samples: &[f64]) -> f64 {
    let sample_rate = 44100.0;
    let n = samples.len();
    let mut data: Vec<Complex<f64>> = samples
        .iter()
        .map(|&x| Complex::new(x, 0.0))
        .collect();

    let mut planner = FftPlanner::new();
    let fft = planner.plan_fft_forward(n);
    fft.process(&mut data);

    let fundamental_frequency = data
        .iter()
        .enumerate()
        .max_by_key(|(_, y)| (y.norm() * y.norm()) as i32)
        .map(|(i, _)| (i as f64 * sample_rate) / (n as f64))
        .unwrap_or(0.0);

    fundamental_frequency
}

pub fn analyze_chunk(chunk: &[u8]) -> Chunk {
    // Actual implementation of the WAV data analysis goes here
    let chunk = Chunk {
        notes: vec![]
     };
    return chunk;
}

#[derive(serde::Serialize)]
pub struct PhiNote {
    pitch: f32,
    start: f32,
    end: f32,
}

#[derive(serde::Serialize)]
pub struct Chunk {
    notes: Vec<PhiNote>
}