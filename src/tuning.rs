use std::collections::HashMap;

pub fn std_tuning() -> HashMap<&'static str, f64> {
    // a map of all notes from A0 to A9 as key and their frequencies as value
    let mut tuning = HashMap::new();
    tuning.insert("A0", 27.5);
    tuning.insert("A#0", 29.135);
    tuning.insert("B0", 30.868);
    tuning.insert("C1", 32.703);
    tuning.insert("C#1", 34.648);
    tuning.insert("D1", 36.708);
    tuning.insert("D#1", 38.891);
    tuning.insert("E1", 41.203);
    tuning.insert("F1", 43.654);
    tuning.insert("F#1", 46.249);
    tuning.insert("G1", 48.999);
    tuning.insert("G#1", 51.913);
    tuning.insert("A1", 55.0);
    tuning.insert("A#1", 58.27);
    tuning.insert("B1", 61.735);
    tuning.insert("C2", 65.406);
    tuning.insert("C#2", 69.296);
    tuning.insert("D2", 73.416);
    tuning.insert("D#2", 77.782);
    tuning.insert("E2", 82.407);
    tuning.insert("F2", 87.307);
    tuning.insert("F#2", 92.499);
    tuning.insert("G2", 97.999);
    tuning.insert("G#2", 103.826);
    tuning.insert("A2", 110.0);
    tuning.insert("A#2", 116.541);
    tuning.insert("B2", 123.471);
    tuning.insert("C3", 130.813);
    tuning.insert("C#3", 138.591);
    tuning.insert("D3", 146.832);
    tuning.insert("D#3", 155.563);
    tuning.insert("E3", 164.814);
    tuning.insert("F3", 174.614);
    tuning.insert("F#3", 184.997);
    tuning.insert("G3", 195.998);
    tuning.insert("G#3", 207.652);
    tuning.insert("A3", 220.0);
    tuning.insert("A#3", 233.082);
    tuning.insert("B3", 246.942);
    tuning.insert("C4", 261.626);
    tuning.insert("C#4", 277.183);
    tuning.insert("D4", 293.665);
    tuning.insert("D#4", 311.127);
    tuning.insert("E4", 329.628);
    tuning.insert("F4", 349.228);
    tuning.insert("F#4", 369.994);
    tuning.insert("G4", 391.995);
    tuning.insert("G#4", 415.305);
    tuning.insert("A4", 440.0);
    tuning.insert("A#4", 466.164);
    tuning.insert("B4", 493.883);
    tuning.insert("C5", 523.251);
    tuning.insert("C#5", 554.365);
    tuning.insert("D5", 587.33);
    tuning.insert("D#5", 622.254);
    tuning.insert("E5", 659.255);
    tuning.insert("F5", 698.456);
    tuning.insert("F#5", 739.989);
    tuning.insert("G5", 783.991);
    tuning.insert("G#5", 830.609);
    tuning.insert("A5", 880.0);
    tuning.insert("A#5", 932.328);
    tuning.insert("B5", 987.767);
    tuning.insert("C6", 1046.502);
    tuning.insert("C#6", 1108.731);
    tuning.insert("D6", 1174.659);
    tuning.insert("D#6", 1244.508);
    tuning.insert("E6", 1318.51);
    tuning.insert("F6", 1396.913);
    tuning.insert("F#6", 1479.978);
    tuning.insert("G6", 1567.982);
    tuning.insert("G#6", 1661.219);
    tuning.insert("A6", 1760.0);
    tuning.insert("A#6", 1864.655);
    tuning.insert("B6", 1975.533);
    tuning.insert("C7", 2093.005);
    tuning.insert("C#7", 2217.461);
    tuning.insert("D7", 2349.318);
    tuning.insert("D#7", 2489.016);
    tuning.insert("E7", 2637.02);
    tuning.insert("F7", 2793.826);
    tuning.insert("F#7", 2959.955);
    tuning.insert("G7", 3135.963);
    tuning.insert("G#7", 3322.438);
    tuning.insert("A7", 3520.0);
    tuning.insert("A#7", 3729.31);
    tuning.insert("B7", 3951.066);
    tuning.insert("C8", 4186.009);
    tuning.insert("C#8", 4434.922);
    tuning.insert("D8", 4698.636);
    tuning.insert("D#8", 4978.032);
    tuning.insert("E8", 5274.041);
    tuning.insert("F8", 5587.652);
    tuning.insert("F#8", 5919.91);
    tuning.insert("G8", 6271.927);
    tuning.insert("G#8", 6644.875);
    tuning.insert("A8", 7040.0);
    tuning.insert("A#8", 7458.62);
    tuning.insert("B8", 7902.132);
    tuning.insert("C9", 8372.018);
    tuning.insert("C#9", 8869.844);
    tuning.insert("D9", 9397.272);
    tuning.insert("D#9", 9956.064);
    tuning.insert("E9", 10548.082);
    tuning.insert("F9", 11175.304);
    tuning.insert("F#9", 11839.82);
    tuning.insert("G9", 12543.854);
    tuning.insert("G#9", 13289.75);
    tuning.insert("A9", 14080.0);
    tuning.insert("A#9", 14917.24);
    tuning.insert("B9", 15804.264);
    
    tuning
}