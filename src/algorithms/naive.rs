use crate::{Guess, Guesser, DICTIONARY};
use std::collections::HashMap;

pub struct Naive {
    remaining: HashMap<&'static str, usize>,
}

impl Naive {
    pub fn new() -> Self {
        Naive {
            remaining: HashMap::from_iter(DICTIONARY.lines().map(|line| {
                let (word, count) = line
                    .split_once(' ')
                    .expect("every line is word + space + frequency");
                let count: usize = count.parse().expect("every count is a number");
                (word, count)
            })),
        }
    }
}

#[derive(Debug, Copy, Clone)]
struct Canditate {
    word: &'static str,
    count: usize,
    goodness: f64,
}

impl Guesser for Naive {
    fn guess(&mut self, history: &[Guess]) -> String {
        if let Some(last) = history.last() {
            self.remaining.retain(|(word, _)| last.matches(word))
        }

        let mut best: Option<Canditate> = None;
        for (&word, &count) in &self.remaining {
            let goodness = 0.0;
            if let Some(c) = best {
                if goodness > c.goodness {
                    best = Some(Canditate {
                        word,
                        count,
                        goodness,
                    })
                }
            } else {
                best = Some(Canditate {
                    word,
                    count,
                    goodness,
                })
            }
        }
        best.unwrap().word.to_string()
    }
}
