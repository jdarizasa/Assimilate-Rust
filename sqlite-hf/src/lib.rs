/* Hugging face rust library to analyze lyrics songs and pust them into sqlite database */

use rust_bert::pipelines::sequence_classification::Label;
use rust_bert::pipelines::zero_shot_classification::ZeroShotClassificationModel;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;

//create zero shot classification candidates
fn create_db() -> sqlite::Connection {
    let db = sqlite::open(":memory:").unwrap();
    db.execute("CREATE TABLE zeroshotcandidates (id INTEGER PRIMARY KEY, label TEXT)")
        .unwrap();
    db.execute("INSERT INTO zeroshotcandidates (label) VALUES ('rock')")
        .unwrap();
    db.execute("INSERT INTO zeroshotcandidates (label) VALUES ('reggaeton')")
        .unwrap();
    db.execute("INSERT INTO zeroshotcandidates (label) VALUES ('salsa')")
        .unwrap();
    db
}

//return all zero shot classification candidates as a vector of strings
pub fn get_zero_shot_candidates() -> Vec<String> {
    let db = create_db();
    let query = "SELECT label FROM zeroshotcandidates";
    let mut candidates = Vec::new();
    db.iterate(query, |pairs| {
        for &(_column, value) in pairs.iter() {
            let value = value.unwrap();
            candidates.push(value.to_string());
        }
        true
    })
    .unwrap();
    candidates
}

//read lyrics from a file and return a vector of strings
pub fn read_lyrics(file_path: &str) -> std::io::Result<Vec<String>> {
    let file = File::open(file_path)?;
    let reader = BufReader::new(file);
    let mut lyrics = Vec::new();
    for line in reader.lines() {
        lyrics.push(line?);
    }
    Ok(lyrics)
}

// use hugging face rust library to classify lyrics
pub fn classify_lyrics(lyrics: Vec<String>) -> Result<Vec<Label>, Box<dyn std::error::Error>> {
    let candidates = get_zero_shot_candidates();
    let lyric_refs = lyrics.iter().map(String::as_str).collect::<Vec<_>>();
    let candidate_refs = candidates.iter().map(String::as_str).collect::<Vec<_>>();
    let model = ZeroShotClassificationModel::new(Default::default())?;
    Ok(model.predict(&lyric_refs, &candidate_refs, None, 128)?)
}
