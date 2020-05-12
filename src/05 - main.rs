fn main() {
    use isahc::prelude::*;
    use serde::Deserialize;
       
    let mut response = isahc::get("http://statsapi.mlb.com/api/v1/people/?personIds=545361,458015,614177").unwrap();
    let mike_trout_bio = response.text().unwrap();

    #[derive(Debug, Deserialize)]
    struct Players {
        people: Vec<Person>,
    }

    #[derive(Debug, Deserialize, Clone)]
    #[serde(rename_all="camelCase")]
    struct Person {
        id: u32,
        full_name: String,
        height: String,
        weight: u16,
        birth_date: String,
        mlb_debut_date: String,
        birth_city: String,
        birth_state_province: Option<String>,
        birth_country: String,
        bat_side: BatSide,
    }

    #[derive(Debug, Deserialize, Clone)]
    enum BatSideCode {
        R,
        L,
        S,
    }

    #[derive(Debug, Deserialize, Clone)]
    enum BatSideDescription {
        Right,
        Left,
        Switch,
    }

    #[derive(Debug, Deserialize, Clone)]
    struct BatSide {
        code: BatSideCode,
        description: BatSideDescription,
    }

    let bio_deserialized: Players = serde_json::from_str(&mike_trout_bio).unwrap();
    dbg!(bio_deserialized);
}
