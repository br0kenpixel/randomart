use clearscreen::clear;
use rand::distributions::{Distribution, WeightedIndex};
use rand::rngs::ThreadRng;
use rand::thread_rng;
use std::{
    io::{stdout, Write},
    ops::RangeInclusive,
    thread::sleep,
    time::Duration,
};

/// A possible value can be a range of characters or a specific character.
/// The second value is the weight (chance of the character showing up).
enum PossibleSelection {
    Range(RangeInclusive<char>, i32),
    Value(char, i32),
}
const ALLOWED_CHARS: [PossibleSelection; 9] = [
    PossibleSelection::Range('a'..='z', 8),
    PossibleSelection::Range('A'..='Z', 5),
    PossibleSelection::Range('0'..='9', 4),
    PossibleSelection::Value('+', 10),
    PossibleSelection::Value('-', 1),
    PossibleSelection::Value('*', 1),
    PossibleSelection::Value('=', 400),
    PossibleSelection::Value('.', 500),
    PossibleSelection::Value(' ', 100_000),
];

fn main() {
    let (cols, rows) = if let Some(size) = termsize::get() {
        (size.cols, size.rows)
    } else {
        eprintln!("Failed to get terminal size");
        return;
    };
    let mut rng = thread_rng(); // threaded random number generator
    let possibilities = build_possibilities();
    clear().unwrap();

    loop {
        let image = generate_image(cols, rows, &possibilities, &mut rng);
        write_image(image);

        sleep(Duration::from_secs(1));
        clear().unwrap();
    }
}

fn build_possibilities() -> Vec<(char, i32)> {
    let mut result = Vec::new();

    for item in ALLOWED_CHARS {
        match item {
            PossibleSelection::Range(range, weight) => {
                for item in range {
                    result.push((item, weight));
                }
            }
            PossibleSelection::Value(value, weight) => result.push((value, weight)),
        }
    }

    result
}

/// Generate the randomart image
fn generate_image(
    cols: u16,
    rows: u16,
    possible_chars: &[(char, i32)],
    rng: &mut ThreadRng,
) -> Vec<Vec<char>> {
    let mut framebuf: Vec<Vec<char>> = Vec::with_capacity(cols as usize);
    let weights = WeightedIndex::new(possible_chars.iter().map(|item| item.1)).unwrap();

    for _ in 0..cols {
        let mut line = Vec::with_capacity(rows as usize);
        for _ in 0..rows {
            let randchar = possible_chars[weights.sample(rng)].0;
            line.push(randchar);
        }
        framebuf.push(line);
    }

    framebuf
}

/// Displays the randomart image
fn write_image(img: Vec<Vec<char>>) {
    for line in img {
        for col in line {
            print!("{col}");
        }
    }
    stdout().flush().unwrap();
}
