//use hound::{WavSpec, WavWriter};
//use std::f32::consts::PI;
//
//// Function to generate a sine wave with a specific frequency
//fn generate_sine_wave(freq: f32, duration_ms: u32, amplitude: f32, sample_rate: u32) -> Vec<i16> {
//    let num_samples = (duration_ms as f32 * sample_rate as f32 / 1000.0) as usize;
//    let mut samples = Vec::with_capacity(num_samples);
//
//    for i in 0..num_samples {
//        let t = i as f32 / sample_rate as f32;
//        let sample = (amplitude * (2.0 * PI * freq * t).sin()) as i16;
//        samples.push(sample);
//    }
//
//    samples
//}
//
//// Function to apply an amplitude envelope
//fn apply_envelope(samples: &mut Vec<i16>, attack_ms: u32, release_ms: u32, sample_rate: u32) {
//    let attack_samples = (attack_ms as f32 * sample_rate as f32 / 1000.0) as usize;
//    let release_samples = (release_ms as f32 * sample_rate as f32 / 1000.0) as usize;
//
//    // Apply attack (fade in)
//    for i in 0..attack_samples.min(samples.len()) {
//        let factor = i as f32 / attack_samples as f32;
//        samples[i] = (samples[i] as f32 * factor) as i16;
//    }
//
//    // Apply release (fade out)
//    let release_start = samples.len().saturating_sub(release_samples);
//    for i in release_start..samples.len() {
//        let factor = (samples.len() - i) as f32 / release_samples as f32;
//        samples[i] = (samples[i] as f32 * factor) as i16;
//    }
//}
//
//// Generate /l/ phoneme WAV file
//fn generate_l_phoneme(output_file: &str) -> Result<(), Box<dyn std::error::Error>> {
//    // WAV specifications
//    let spec = WavSpec {
//        channels: 1,
//        sample_rate: 16000,
//        bits_per_sample: 16,
//        sample_format: hound::SampleFormat::Int,
//    };
//
//    let mut writer = WavWriter::create(output_file, spec)?;
//
//    // /l/ phoneme is characterized by formants around 250-500Hz for F1 and 1200Hz for F2
//    // Generate a complex tone with these frequencies
//    let mut samples1 = generate_sine_wave(350.0, 300, 8000.0, spec.sample_rate);
//    let mut samples2 = generate_sine_wave(1200.0, 300, 4000.0, spec.sample_rate);
//
//    // Apply envelope to make it sound more natural
//    apply_envelope(&mut samples1, 50, 100, spec.sample_rate);
//    apply_envelope(&mut samples2, 50, 100, spec.sample_rate);
//
//    // Combine the formants
//    let mut combined_samples = Vec::with_capacity(samples1.len());
//    for i in 0..samples1.len() {
//        combined_samples.push(samples1[i] / 2 + samples2[i] / 2);
//    }
//
//    // Write to file
//    for sample in combined_samples {
//        writer.write_sample(sample)?;
//    }
//
//    writer.finalize()?;
//    println!("Generated {}", output_file);
//    Ok(())
//}
//
//// Generate /an/ phoneme WAV file
//fn generate_an_phoneme(output_file: &str) -> Result<(), Box<dyn std::error::Error>> {
//    // WAV specifications
//    let spec = WavSpec {
//        channels: 1,
//        sample_rate: 16000,
//        bits_per_sample: 16,
//        sample_format: hound::SampleFormat::Int,
//    };
//
//    let mut writer = WavWriter::create(output_file, spec)?;
//
//    // First generate /a/ sound
//    // /a/ phoneme has formants around 800Hz for F1 and 1200Hz for F2
//    let mut a_samples1 = generate_sine_wave(800.0, 200, 10000.0, spec.sample_rate);
//    let mut a_samples2 = generate_sine_wave(1200.0, 200, 5000.0, spec.sample_rate);
//
//    apply_envelope(&mut a_samples1, 40, 30, spec.sample_rate);
//    apply_envelope(&mut a_samples2, 40, 30, spec.sample_rate);
//
//    // Then generate /n/ sound
//    // /n/ phoneme has formants around 300Hz for F1 and 1450Hz for F2 with some nasal resonance
//    let mut n_samples1 = generate_sine_wave(300.0, 200, 8000.0, spec.sample_rate);
//    let mut n_samples2 = generate_sine_wave(1450.0, 200, 4000.0, spec.sample_rate);
//    let mut n_samples3 = generate_sine_wave(2500.0, 200, 2000.0, spec.sample_rate); // Nasal resonance
//
//    apply_envelope(&mut n_samples1, 30, 60, spec.sample_rate);
//    apply_envelope(&mut n_samples2, 30, 60, spec.sample_rate);
//    apply_envelope(&mut n_samples3, 30, 60, spec.sample_rate);
//
//    // Combine the /a/ formants
//    let mut a_combined = Vec::with_capacity(a_samples1.len());
//    for i in 0..a_samples1.len() {
//        a_combined.push(a_samples1[i] / 2 + a_samples2[i] / 2);
//    }
//
//    // Combine the /n/ formants
//    let mut n_combined = Vec::with_capacity(n_samples1.len());
//    for i in 0..n_samples1.len() {
//        n_combined.push(n_samples1[i] / 3 + n_samples2[i] / 3 + n_samples3[i] / 3);
//    }
//
//    // Combine /a/ and /n/ with a small crossfade
//    let crossfade_samples = 400;
//    let mut an_combined = Vec::new();
//
//    // Add /a/ samples
//    for i in 0..(a_combined.len() - crossfade_samples) {
//        an_combined.push(a_combined[i]);
//    }
//
//    // Crossfade
//    for i in 0..crossfade_samples {
//        let a_weight = (crossfade_samples - i) as f32 / crossfade_samples as f32;
//        let n_weight = i as f32 / crossfade_samples as f32;
//
//        let sample = (a_combined[a_combined.len() - crossfade_samples + i] as f32 * a_weight
//            + n_combined[i] as f32 * n_weight) as i16;
//
//        an_combined.push(sample);
//    }
//
//    // Add remaining /n/ samples
//    for i in crossfade_samples..n_combined.len() {
//        an_combined.push(n_combined[i]);
//    }
//
//    // Write to file
//    for sample in an_combined {
//        writer.write_sample(sample)?;
//    }
//
//    writer.finalize()?;
//    println!("Generated {}", output_file);
//    Ok(())
//}
//
//fn main() -> Result<(), Box<dyn std::error::Error>> {
//    generate_l_phoneme("l.wav")?;
//    generate_an_phoneme("an.wav")?;
//    println!("Successfully generated both phoneme files!");
//    Ok(())
//}
//

use hound::{WavReader, WavWriter};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Open the input files
    let mut l_reader = WavReader::open("l.wav")?;
    let mut an_reader = WavReader::open("an.wav")?;

    // Get specifications
    let spec = l_reader.spec();

    // Check if specs match
    if spec != an_reader.spec() {
        println!("Warning: Audio files have different formats!");
    }

    // Read all samples into vectors
    let l_samples: Vec<i16> = l_reader
        .samples::<i16>()
        .collect::<std::result::Result<Vec<i16>, _>>()?;
    let an_samples: Vec<i16> = an_reader
        .samples::<i16>()
        .collect::<std::result::Result<Vec<i16>, _>>()?;

    // Create a writer for the output file
    let mut writer = WavWriter::create("lan.wav", spec)?;

    // Determine crossfade length (in samples)
    // Experiment with this value based on your specific audio files
    let crossfade_len = 500; // About 30ms at 16kHz

    // Find where to start the crossfade
    // Trim the /l/ file to remove trailing silence (if any)
    let mut l_end = l_samples.len();
    while l_end > 0 && l_samples[l_end - 1].abs() < 500 {
        l_end -= 1;
    }

    // Find where the /an/ phoneme actually starts (detect onset)
    let mut an_start = 0;
    while an_start < an_samples.len() && an_samples[an_start].abs() < 500 {
        an_start += 1;
    }

    // Write the first part of l.wav (before crossfade)
    let crossfade_start = if l_end > crossfade_len {
        l_end - crossfade_len
    } else {
        0
    };
    for i in 0..crossfade_start {
        writer.write_sample(l_samples[i])?;
    }

    // Perform crossfade
    for i in 0..crossfade_len {
        if crossfade_start + i < l_samples.len() && an_start + i < an_samples.len() {
            let l_weight = (crossfade_len - i) as f32 / crossfade_len as f32;
            let an_weight = i as f32 / crossfade_len as f32;

            let blended_sample = (l_samples[crossfade_start + i] as f32 * l_weight
                + an_samples[an_start + i] as f32 * an_weight)
                as i16;

            writer.write_sample(blended_sample)?;
        }
    }

    // Write the rest of an.wav (after crossfade)
    for i in (an_start + crossfade_len)..an_samples.len() {
        writer.write_sample(an_samples[i])?;
    }

    writer.finalize()?;
    println!("Successfully created lan.wav with smooth transition!");
    Ok(())
}
