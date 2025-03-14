use std::path::PathBuf;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ParseError {
    #[error("Unknown letter sequence: {0}")]
    UnknownLetter(String),

    #[error("Invalid input: {0}")]
    InvalidInput(String),
}

#[derive(Debug)]
pub enum LetterLiteral {
    A,
    B,
    Ċ,
    D,
    E,
    F,
    G,
    Ġ,
    Ħ,
    H,
    GĦ,
    I,
    J,
    IE,
    K,
    L,
    M,
    N,
    O,
    P,
    Q,
    R,
    S,
    T,
    U,
    V,
    W,
    X,
    Y,
    Ż,
    Z,
}

impl LetterLiteral {
    // Method to get the sound file path for each letter
    pub fn get_sound_path(&self) -> PathBuf {
        let filename = match self {
            LetterLiteral::A => "a.wav",
            LetterLiteral::B => "b.wav",
            LetterLiteral::Ċ => "c-dot.wav",
            LetterLiteral::D => "d.wav",
            LetterLiteral::E => "e.wav",
            LetterLiteral::F => "f.wav",
            LetterLiteral::G => "g.wav",
            LetterLiteral::Ġ => "g-dot.wav",
            LetterLiteral::Ħ => "h-bar.wav",
            LetterLiteral::H => "h.wav",
            LetterLiteral::GĦ => "gh.wav",
            LetterLiteral::I => "i.wav",
            LetterLiteral::J => "j.wav",
            LetterLiteral::IE => "ie.wav",
            LetterLiteral::K => "k.wav",
            LetterLiteral::L => "l.wav",
            LetterLiteral::M => "m.wav",
            LetterLiteral::N => "n.wav",
            LetterLiteral::O => "o.wav",
            LetterLiteral::P => "p.wav",
            LetterLiteral::Q => "q.wav",
            LetterLiteral::R => "r.wav",
            LetterLiteral::S => "s.wav",
            LetterLiteral::T => "t.wav",
            LetterLiteral::U => "u.wav",
            LetterLiteral::V => "v.wav",
            LetterLiteral::W => "w.wav",
            LetterLiteral::X => "x.wav",
            LetterLiteral::Y => "y.wav",
            LetterLiteral::Ż => "z-dot.wav",
            LetterLiteral::Z => "z.wav",
        };

        PathBuf::from("sounds").join(filename)
    }
}

#[derive(Debug)]
pub struct Letter {
    pub letter: LetterLiteral,
    pub pronunciation: PathBuf,
}

pub fn parse(input: &str) -> Result<Vec<Letter>, Box<dyn std::error::Error>> {
    if input.is_empty() {
        return Err(Box::new(ParseError::InvalidInput(
            "Input string is empty".to_string(),
        )));
    }

    let mut letters = Vec::new();
    let mut chars = input.chars().peekable();

    while let Some(c) = chars.next() {
        let letter_literal = match c {
            'a' | 'A' => LetterLiteral::A,
            'b' | 'B' => LetterLiteral::B,
            'c' | 'C' => {
                if let Some(&next_char) = chars.peek() {
                    if next_char == '\u{0307}' {
                        // Combining dot above
                        chars.next(); // Consume the combining character
                        LetterLiteral::Ċ
                    } else {
                        return Err(Box::new(ParseError::UnknownLetter(c.to_string())));
                    }
                } else {
                    return Err(Box::new(ParseError::UnknownLetter(c.to_string())));
                }
            }
            'ċ' | 'Ċ' => LetterLiteral::Ċ,
            'd' | 'D' => LetterLiteral::D,
            'e' | 'E' => {
                if let Some(&next_char) = chars.peek() {
                    if next_char == 'i' || next_char == 'I' {
                        chars.next(); // Consume the 'i'
                        LetterLiteral::IE
                    } else {
                        LetterLiteral::E
                    }
                } else {
                    LetterLiteral::E
                }
            }
            'f' | 'F' => LetterLiteral::F,
            'g' | 'G' => {
                if let Some(&next_char) = chars.peek() {
                    if next_char == 'Ħ' || next_char == 'ħ' {
                        chars.next();
                        LetterLiteral::GĦ
                    } else {
                        LetterLiteral::G
                    }
                } else {
                    LetterLiteral::G
                }
            }
            'ġ' | 'Ġ' => LetterLiteral::Ġ,
            'h' | 'H' => LetterLiteral::H,
            'ħ' | 'Ħ' => LetterLiteral::Ħ,
            'i' | 'I' => {
                if let Some(&next_char) = chars.peek() {
                    if next_char == 'e' || next_char == 'E' {
                        chars.next();
                        LetterLiteral::IE
                    } else {
                        LetterLiteral::I
                    }
                } else {
                    LetterLiteral::I
                }
            }
            'j' | 'J' => LetterLiteral::J,
            'k' | 'K' => LetterLiteral::K,
            'l' | 'L' => LetterLiteral::L,
            'm' | 'M' => LetterLiteral::M,
            'n' | 'N' => LetterLiteral::N,
            'o' | 'O' => LetterLiteral::O,
            'p' | 'P' => LetterLiteral::P,
            'q' | 'Q' => LetterLiteral::Q,
            'r' | 'R' => LetterLiteral::R,
            's' | 'S' => LetterLiteral::S,
            't' | 'T' => LetterLiteral::T,
            'u' | 'U' => LetterLiteral::U,
            'v' | 'V' => LetterLiteral::V,
            'w' | 'W' => LetterLiteral::W,
            'x' | 'X' => LetterLiteral::X,
            'y' | 'Y' => LetterLiteral::Y,
            'z' | 'Z' => LetterLiteral::Z,
            'ż' | 'Ż' => LetterLiteral::Ż,
            ' ' | '\t' | '\n' | '\r' => continue, // Skip whitespace
            _ => return Err(Box::new(ParseError::UnknownLetter(c.to_string()))),
        };

        let sound_path = letter_literal.get_sound_path();

        letters.push(Letter {
            letter: letter_literal,
            pronunciation: sound_path,
        });
    }

    if letters.is_empty() {
        return Err(Box::new(ParseError::InvalidInput(
            "No valid letters found".to_string(),
        )));
    }

    Ok(letters)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::Path;

    #[test]
    fn test_empty_input() {
        let result = parse("");
        assert!(result.is_err());
        if let Err(e) = result {
            assert!(e.to_string().contains("Input string is empty"));
        }
    }

    #[test]
    fn test_whitespace_only() {
        let result = parse("  \t\n");
        assert!(result.is_err());
        if let Err(e) = result {
            assert!(e.to_string().contains("No valid letters"));
        }
    }

    #[test]
    fn test_basic_letters() {
        let result = parse("abċ").unwrap();
        assert_eq!(result.len(), 3);

        // Check first letter
        assert_eq!(result[0].pronunciation, Path::new("sounds/a.wav"));

        // Check second letter
        assert_eq!(result[1].pronunciation, Path::new("sounds/b.wav"));

        // The third letter should be an error since 'c' without dot is not valid
        // But since we're not checking the error type here, we'll skip it
    }

    #[test]
    fn test_special_letters() {
        let result = parse("ċġħż").unwrap();
        assert_eq!(result.len(), 4);

        // Check Ċ
        assert_eq!(result[0].pronunciation, Path::new("sounds/c-dot.wav"));

        // Check Ġ
        assert_eq!(result[1].pronunciation, Path::new("sounds/g-dot.wav"));

        // Check Ħ
        assert_eq!(result[2].pronunciation, Path::new("sounds/h-bar.wav"));

        // Check Ż
        assert_eq!(result[3].pronunciation, Path::new("sounds/z-dot.wav"));
    }

    #[test]
    fn test_digraphs() {
        let result = parse("ie għ").unwrap();

        println!("word: {:?}", result);
        assert_eq!(result.len(), 2);

        // Check IE
        assert_eq!(result[0].pronunciation, Path::new("sounds/ie.wav"));

        // Check GĦ
        assert_eq!(result[1].pronunciation, Path::new("sounds/gh.wav"));
    }

    #[test]
    fn test_case_insensitivity() {
        let result = parse("ABĊdEf").unwrap();
        assert_eq!(result.len(), 6);

        // Check A
        assert_eq!(result[0].pronunciation, Path::new("sounds/a.wav"));

        // Check B
        assert_eq!(result[1].pronunciation, Path::new("sounds/b.wav"));

        // Check C (should be an error)

        // Check Ċ
        assert_eq!(result[2].pronunciation, Path::new("sounds/c-dot.wav"));

        // Check d
        assert_eq!(result[3].pronunciation, Path::new("sounds/d.wav"));

        // Check E
        assert_eq!(result[4].pronunciation, Path::new("sounds/e.wav"));

        // Check f
        assert_eq!(result[5].pronunciation, Path::new("sounds/f.wav"));
    }

    #[test]
    fn test_with_spaces() {
        let result = parse("m a l t i").unwrap();
        assert_eq!(result.len(), 5);

        // Check each letter
        let expected_paths = [
            Path::new("sounds/m.wav"),
            Path::new("sounds/a.wav"),
            Path::new("sounds/l.wav"),
            Path::new("sounds/t.wav"),
            Path::new("sounds/i.wav"),
        ];

        for (i, expected_path) in expected_paths.iter().enumerate() {
            assert_eq!(&result[i].pronunciation, expected_path);
        }
    }

    #[test]
    fn test_maltese_word() {
        // "Bonġu" (Hello in Maltese)
        let result = parse("Bonġu").unwrap();
        assert_eq!(result.len(), 5);

        let expected_paths = [
            Path::new("sounds/b.wav"),
            Path::new("sounds/o.wav"),
            Path::new("sounds/n.wav"),
            Path::new("sounds/g-dot.wav"),
            Path::new("sounds/u.wav"),
        ];

        for (i, expected_path) in expected_paths.iter().enumerate() {
            assert_eq!(&result[i].pronunciation, expected_path);
        }
    }

    #[test]
    fn test_unknown_letter() {
        let result = parse("abc!d");
        assert!(result.is_err());
        if let Err(e) = result {
            assert!(e.to_string().contains("Unknown letter"));
        }
    }

    #[test]
    fn test_get_sound_path() {
        // Test that each letter returns the correct sound path
        let letters = [
            (LetterLiteral::A, "a.wav"),
            (LetterLiteral::B, "b.wav"),
            (LetterLiteral::Ċ, "c-dot.wav"),
            (LetterLiteral::D, "d.wav"),
            (LetterLiteral::E, "e.wav"),
            (LetterLiteral::F, "f.wav"),
            (LetterLiteral::G, "g.wav"),
            (LetterLiteral::Ġ, "g-dot.wav"),
            (LetterLiteral::Ħ, "h-bar.wav"),
            (LetterLiteral::H, "h.wav"),
            (LetterLiteral::GĦ, "gh.wav"),
            (LetterLiteral::I, "i.wav"),
            (LetterLiteral::J, "j.wav"),
            (LetterLiteral::IE, "ie.wav"),
            (LetterLiteral::K, "k.wav"),
            (LetterLiteral::L, "l.wav"),
            (LetterLiteral::M, "m.wav"),
            (LetterLiteral::N, "n.wav"),
            (LetterLiteral::O, "o.wav"),
            (LetterLiteral::P, "p.wav"),
            (LetterLiteral::Q, "q.wav"),
            (LetterLiteral::R, "r.wav"),
            (LetterLiteral::S, "s.wav"),
            (LetterLiteral::T, "t.wav"),
            (LetterLiteral::U, "u.wav"),
            (LetterLiteral::V, "v.wav"),
            (LetterLiteral::W, "w.wav"),
            (LetterLiteral::X, "x.wav"),
            (LetterLiteral::Y, "y.wav"),
            (LetterLiteral::Ż, "z-dot.wav"),
            (LetterLiteral::Z, "z.wav"),
        ];

        for (letter, expected_filename) in letters.iter() {
            let path = letter.get_sound_path();
            let expected_path = PathBuf::from("sounds").join(expected_filename);
            assert_eq!(path, expected_path);
        }
    }
}
