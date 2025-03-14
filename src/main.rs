pub mod parser;
pub mod sound;

use hound::{WavReader, WavWriter};
use parser::{Letter, LetterLiteral};
use sound::murph_sound;
use std::path::PathBuf;

// Helper function to check if a letter is a vowel in Maltese
fn is_vowel(letter: &LetterLiteral) -> bool {
    matches!(
        letter,
        LetterLiteral::A
            | LetterLiteral::E
            | LetterLiteral::I
            | LetterLiteral::IE
            | LetterLiteral::O
            | LetterLiteral::U
    )
}

// Helper function to check if a letter is a consonant in Maltese
fn is_consonant(letter: &LetterLiteral) -> bool {
    !is_vowel(letter)
}

// Function to append a sound file to another
fn append_sound(
    target_path: &PathBuf,
    source_path: &PathBuf,
) -> Result<(), Box<dyn std::error::Error>> {
    // Open the source file
    let mut reader = WavReader::open(source_path)?;
    let spec = reader.spec();

    // If the target file doesn't exist, create it with the same spec as the source
    let target_exists = std::path::Path::new(target_path).exists();

    if !target_exists {
        let mut writer = WavWriter::create(target_path, spec)?;

        // Copy samples from source to target
        for sample in reader.samples::<i16>() {
            writer.write_sample(sample?)?;
        }

        writer.finalize()?;
    } else {
        // If the target exists, we need to append to it
        let mut reader_target = WavReader::open(target_path)?;
        let target_spec = reader_target.spec();

        // Ensure the specifications match
        if spec.channels != target_spec.channels
            || spec.sample_rate != target_spec.sample_rate
            || spec.sample_format != target_spec.sample_format
        {
            return Err("Source and target WAV files have different specifications".into());
        }

        // Read all samples from the target file
        let mut target_samples: Vec<i16> = reader_target
            .samples::<i16>()
            .collect::<Result<Vec<i16>, _>>()?;

        // Append samples from the source file
        let source_samples: Vec<i16> = reader.samples::<i16>().collect::<Result<Vec<i16>, _>>()?;
        target_samples.extend(source_samples);

        // Create a new writer to replace the target file
        let mut writer = WavWriter::create(target_path, spec)?;

        // Write all samples back
        for sample in target_samples {
            writer.write_sample(sample)?;
        }

        writer.finalize()?;
    }

    Ok(())
}

// Main function to process words and create sound files
fn process_words(words: Vec<Vec<Letter>>) -> Result<(), Box<dyn std::error::Error>> {
    for (word_index, word) in words.iter().enumerate() {
        // Create a final.wav file for this word
        let final_path = PathBuf::from(format!("word_{}_final.wav", word_index));

        // Delete the file if it exists
        if final_path.exists() {
            std::fs::remove_file(&final_path)?;
        }

        // Process each letter in the word
        let mut i = 0;
        while i < word.len() {
            let current_letter = &word[i].letter;
            let current_sound_path = current_letter.get_sound_path();

            // Check if the current letter is a consonant and the next letter is a vowel
            if i + 1 < word.len() && is_consonant(current_letter) && is_vowel(&word[i + 1].letter) {
                let next_letter = &word[i + 1].letter;
                let next_sound_path = next_letter.get_sound_path();

                // Murph the sounds together
                let murphed_sound_path = murph_sound(&current_sound_path, &next_sound_path)?;

                // Append the murphed sound to final.wav
                append_sound(&final_path, &murphed_sound_path)?;

                // Skip the next letter since we've processed it
                i += 2;
            } else {
                // Just append the current letter's sound to final.wav
                append_sound(&final_path, &current_sound_path)?;

                // Move to the next letter
                i += 1;
            }
        }
    }

    Ok(())
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = std::env::args();
    let words = args.skip(1).try_fold(
        vec![],
        |mut words, ele| -> Result<Vec<Vec<Letter>>, Box<dyn std::error::Error>> {
            let parsed = parser::parse(&ele)?;

            words.push(parsed);
            Ok(words)
        },
    )?;

    // Process the words to create the final sound files
    process_words(words)?;

    Ok(())
}
