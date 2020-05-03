fn main() {
    use isahc::prelude::*;
    use serde::Deserialize;
       
    let mut response = isahc::get("http://statsapi.mlb.com/api/v1/people/?personIds=545361,458015").unwrap();
    let mike_trout_bio = response.text().unwrap();

    #[derive(Debug, Deserialize)]
    struct Players {
        people: Vec<Person>,
    }

    #[derive(Debug, Deserialize)]
    #[serde(rename_all="camelCase")]
    struct Person {
        id: u32,
        full_name: String,
        height: String,
        weight: u16,
        birth_date: String,
        mlb_debut_date: String,
        birth_city: String,
        birth_state_province: String,
        birth_country: Country,
    }

    #[derive(Debug, Deserialize)]
    enum Country {
        Canada,
        USA,
        #[serde(other)]
        Other,
    }

    let bio_deserialized: Players = serde_json::from_str(&mike_trout_bio).unwrap();
    dbg!(bio_deserialized);
}
