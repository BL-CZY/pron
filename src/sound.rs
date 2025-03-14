use std::path::PathBuf;

use hound::{WavReader, WavWriter};

pub fn murph_sound(
    sound1: &PathBuf,
    sound2: &PathBuf,
) -> Result<PathBuf, Box<dyn std::error::Error>> {
    // Open the input files
    let mut sound1 = WavReader::open(sound1)?;
    let mut sound2 = WavReader::open(sound2)?;

    // Get specifications
    let spec = sound1.spec();

    // Check if specs match
    if spec != sound2.spec() {
        println!("Warning: Audio files have different formats!");
    }

    // Read all samples into vectors
    let l_samples: Vec<i16> = sound1
        .samples::<i16>()
        .collect::<std::result::Result<Vec<i16>, _>>()?;
    let an_samples: Vec<i16> = sound2
        .samples::<i16>()
        .collect::<std::result::Result<Vec<i16>, _>>()?;

    // Create a writer for the output file
    let mut writer = WavWriter::create("temp/combined.wav", spec)?;

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
    Ok(PathBuf::from("temp/combined.wav"))
}
