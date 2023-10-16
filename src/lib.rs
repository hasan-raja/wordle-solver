use std::collections::HashSet;

pub mod algorithms;

const DICTIONARY: &str = include_str!("../dictionary.txt");

pub struct Wordle {
    dictionary: HashSet<&'static str>,
}

impl Wordle {
    pub fn new() -> Self {
        Self
    }

    pub fn play<G: Guesser>(&self, answer: &'static str, mut guesser: G) -> Option<usize> {
        //play six rounds where it invokes guesser each round
        let mut history: Vec<Guess> = Vec::new();
        //wordle only allows six guess
        //we allow more to avoid chopping off the score distribution for stat purposes.
        for i in 1..=32 {
            let guess = guesser.guess(&history[..]);
            if guess == answer {
                return Some(i);
            }
            assert!(self.dictionary.contains(&*guess));
            let correctness = Correctness::compute(answer, &guess);
            history.push(Guess {
                word: guess,
                mask: correctness,
            });
        }
        None
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Correctness {
    ///Green
    CorrectPlaced,
    /// Yellow
    MisPlaced,
    /// Gray
    WrongPlaced,
}

impl Correctness {
    fn compute(answer: &str, guess: &str) -> [Self; 5] {
        assert_eq!(answer.len(), 5);
        assert_eq!(guess.len(), 5);

        let mut c = [Correctness::WrongPlaced; 5];

        //Mark things green
        for (i, (a, g)) in answer.chars().zip(guess.chars()).enumerate() {
            if a == g {
                c[i] = Correctness::CorrectPlaced;
            } else {
            }
        }

        //Mark things yellow
        let mut used = [false; 5];
        for (i, &c) in c.iter().enumerate() {
            if c == Correctness::CorrectPlaced {
                used[i] = true;
            }
        }
        for (i, g) in guess.chars().enumerate() {
            if c[i] == Correctness::CorrectPlaced {
                //Already marked as green
                continue;
            }

            if answer.chars().enumerate().any(|(i, a)| {
                if a == g && !used[i] {
                    used[i] = true;
                    return true;
                }
                false
            }) {
                c[i] = Correctness::MisPlaced;
            }
        }
        c
    }
}

pub struct Guess {
    pub word: String,
    pub mask: [Correctness; 5],
}

impl Guess {
    pub fn matches(&self, word: &str) -> bool {
        //first,check greens
        assert_eq!(self.word.len(), 5);
        assert_eq!(word.len(), 5);
        let mut used =[false;5];
        for (i, ((g, &m), w)) in self
            .word
            .chars()
            .zip(&self.mask)
            .zip(word.chars())
            .enumerate()
        {

            if m == Correctness::CorrectPlaced{
                if g!=w{
                    return false;
                }else {
                    used[i]=true;
                    continue;
                }
            }
            
            // if let Some(j) = self.word.chars().map(&self.mask).enumerate().find_map(|(j,g,m)|{
            //     if unused[j]{

            //     }
            // }){

            // }
            match m {
                Correctness::CorrectPlaced => {
                    if g != w {
                        return false;
                    }
                    used[i]=true; 
                }
                Correctness::MisPlaced => todo!(),
                Correctness::WrongPlaced => {
                    if g == w {
                        return false;
                    }
                }
            }
        }

        for (i,w) in word.chars().enumerate(){

        }
        true
    }
}

pub trait Guesser {
    fn guess(&mut self, history: &[Guess]) -> String;
}

impl Guesser for fn(history: &[Guess]) -> String {
    fn guess(&mut self, history: &[Guess]) -> String {
        (*self)(history)
    }
}

#[cfg(test)]
macro_rules! guesser {
    (|$history:ident| $impl:block) => {{
        struct G;
        impl $crate::Guesser for G {
            fn guess(&mut self, $history: &[Guess]) -> String {
                $impl
            }
        }
        G
    }};
}

#[cfg(test)]
mod tests {

    mod game {
        use crate::{Guess, Wordle};

        #[test]
        fn genius() {
            let w = Wordle::new();
            let guesser = guesser!(|_history| { "moved".to_string() });

            assert_eq!(w.play("moved", guesser), Some(1));
        }

        #[test]
        fn magnificiant() {
            let w = Wordle::new();
            let guesser = guesser!(|history| {
                if history.len() == 1 {
                    return "right".to_string();
                }
                "wrong".to_string()
            });

            assert_eq!(w.play("right", guesser), Some(2));
        }

        #[test]
        fn impressive() {
            let w = Wordle::new();
            let guesser = guesser!(|history| {
                if history.len() == 2 {
                    return "right".to_string();
                }
                "wrong".to_string()
            });

            assert_eq!(w.play("right", guesser), Some(3));
        }

        #[test]
        fn splendid() {
            let w = Wordle::new();
            let guesser = guesser!(|history| {
                if history.len() == 3 {
                    return "right".to_string();
                }
                "wrong".to_string()
            });

            assert_eq!(w.play("right", guesser), Some(4));
        }

        #[test]
        fn great() {
            let w = Wordle::new();
            let guesser = guesser!(|history| {
                if history.len() == 4 {
                    return "right".to_string();
                }
                "wrong".to_string()
            });

            assert_eq!(w.play("right", guesser), Some(5));
        }

        #[test]
        fn phew() {
            let w = Wordle::new();
            let guesser = guesser!(|history| {
                if history.len() == 5 {
                    return "right".to_string();
                }
                "wrong".to_string()
            });

            assert_eq!(w.play("right", guesser), Some(6));
        }

        #[test]
        fn oops() {
            let w = Wordle::new();
            let guesser = guesser!(|_history| { "wrong".to_string() });

            assert_eq!(w.play("moved", guesser), None);
        }
    }

    mod compute {
        use crate::Correctness;

        macro_rules! mask {
            (C) => {Correctness::CorrectPlaced};
            (M) => {Correctness::MisPlaced};
            (W) => {Correctness::WrongPlaced};
            ($($c:tt)+) =>{[
                $(mask!($c)),+
            ]}
        }
        #[test]
        fn all_green() {
            assert_eq!(Correctness::compute("abcde", "abcde"), mask!(C C C C C));
        }
        #[test]
        fn all_gray() {
            assert_eq!(Correctness::compute("jshui", "abcde"), mask!(W W W W W));
        }
        #[test]
        fn all_yellow() {
            assert_eq!(Correctness::compute("edacb", "abcde"), mask!(M M M M M));
        }

        #[test]
        fn repeat_green() {
            assert_eq!(Correctness::compute("aabbb", "aaccc"), mask!(C C W W W));
        }

        #[test]
        fn repeat_yellow() {
            assert_eq!(Correctness::compute("aabbb", "ccaac"), mask!(W W M M W));
        }

        #[test]
        fn repeat_some_green() {
            assert_eq!(Correctness::compute("aabbb", "caacc"), mask!(W C M W W));
        }
    }
}
