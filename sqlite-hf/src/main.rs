// Create a CLI tool to classify music genres based on lyrics in lyrics.txt using the lib module
use sqlite_hf::classify_lyrics;
use sqlite_hf::read_lyrics;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let lyrics = read_lyrics(concat!(env!("CARGO_MANIFEST_DIR"), "/lyrics.txt"))?;
    let genre = classify_lyrics(lyrics)?;
    println!("Predicted genre: {:?}", genre);
    Ok(())
}
