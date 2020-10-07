use anyhow::Error;
use rand::{distributions::Uniform, thread_rng, Rng};
use rayon::prelude::*;
use words::Words;

use std::{
    collections::HashMap,
    fs::File,
    io::{BufRead, BufReader, BufWriter, Write},
    path::Path,
};

fn merge_maps(a: HashMap<String, usize>, b: &mut HashMap<String, usize>) -> HashMap<String, usize> {
    let mut base = a;
    for (key, value) in b.drain() {
        base.entry(key).and_modify(|c| *c += value).or_insert(value);
    }

    base
}

/// 800 MB
const MAX_BUF_SIZE: usize = 800 * 1024 * 1024;

/// Generates a new file with `size` (in bytes) of random words
pub fn create(filename: impl AsRef<Path>, size: usize) -> Result<(), Error> {
    let mut words = Words::new();
    words.populate();
    let words = words.words;

    let mut rng = thread_rng();
    let distr = Uniform::new(0, words.len());

    let file = File::create(filename)?;

    let capacity = (0.2 * size as f64) as usize;
    let capacity = if capacity > MAX_BUF_SIZE {
        MAX_BUF_SIZE
    } else {
        capacity
    };

    let mut writer = BufWriter::with_capacity(capacity, file);

    let mut acc_size = 0;
    while acc_size < size {
        let idx = rng.sample(distr);
        let word = &words[idx];

        let written = writer.write(word.as_bytes())?;
        let _ = writer.write(b" ")?;
        acc_size += written + 1;
    }

    writer.flush()?;
    Ok(())
}

/// Count words (naive version)
pub fn count(filename: impl AsRef<Path>) -> Result<HashMap<String, usize>, Error> {
    let file = File::open(filename)?;
    let size = file.metadata()?.len();

    let capacity = (0.2 * size as f64) as usize;
    let capacity = if capacity > MAX_BUF_SIZE {
        MAX_BUF_SIZE
    } else {
        capacity
    };

    let reader = BufReader::with_capacity(capacity, file);

    let count = reader
        .split(b' ')
        .par_bridge()
        .map(|w| {
            let w = w.unwrap();
            String::from_utf8(w)
                .unwrap()
                .replace(',', "")
                .replace('.', "")
        })
        .fold(HashMap::new, |mut map, w| {
            map.entry(w).and_modify(|c| *c += 1).or_insert(1);
            map
        })
        .reduce(HashMap::new, |a, mut b| merge_maps(a, &mut b));

    Ok(count)
}
