use std::{
    fs::File,
    io::{self, BufRead, BufReader},
    path::Path,
};

use rand::seq::SliceRandom;

const DEFAULT_WORD: &str = "test";

#[derive(Default)]
pub(crate) struct WordLookup {
    words: Vec<String>,
    index: usize,
}

impl WordLookup {
    pub(crate) fn populate_from_file(&mut self, path: &Path) -> io::Result<()> {
        let file = File::open(path)?;
        let reader = BufReader::new(file);
        for line in reader.lines() {
            self.words.push(line?.to_lowercase());
        }
        self.words.shuffle(&mut rand::thread_rng());
        Ok(())
    }

    pub(crate) fn get(&mut self) -> String {
        if self.words.is_empty() {
            return DEFAULT_WORD.to_string();
        }
        let word = self.words[self.index].clone();
        self.index += 1;
        if self.index == self.words.len() {
            self.index = 0;
        }
        word
    }
}
