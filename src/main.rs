fn main() {
    use isahc::prelude::*;
    use serde::Deserialize;
    
        
    let mut response = isahc::get("https://statsapi.mlb.com/api/v1/people/545361").unwrap();
    let mike_trout_bio = response.text().unwrap();

    #[derive(Debug, Deserialize)]
    struct Players {
        people: Vec<Person>,
    }

    #[derive(Debug, Deserialize)]
    struct Person {
        id: u32,
    }

    let bio_deserialized: Players = serde_json::from_str(&mike_trout_bio).unwrap();
    dbg!(bio_deserialized);
}
