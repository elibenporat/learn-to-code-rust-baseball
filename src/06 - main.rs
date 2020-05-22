fn main() {
    use isahc::prelude::*;
    use serde::Deserialize;
       
    let mut response = isahc::get("http://statsapi.mlb.com/api/v1/people/?personIds=545361,458015,614177").unwrap();
    let mike_trout_bio = response.text().unwrap();

    #[derive(Debug, Deserialize)]
    struct Players {
        people: Vec<PersonTemp>,
    }

    #[derive(Debug, Deserialize, Clone)]
    #[serde(rename_all="camelCase")]
    struct PersonTemp {
        id: u32,
        full_name: String,
        height: String,
        weight: u16,
        birth_date: String,
        mlb_debut_date: String,
        birth_city: String,
        birth_state_province: Option<String>,
        birth_country: String,
        bat_side: Side,
        pitch_hand: Side,
    }

    #[derive(Debug)]
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
        bat_side_code: SideCode,
        bat_side_description: SideDescription,
        pitch_hand_code: SideCode,
        pitch_hand_description: SideDescription,
    }

    impl From<PersonTemp> for Person {
        fn from (person_temp: PersonTemp) -> Person {
            Person {
                id: person_temp.id,
                full_name: person_temp.full_name,
                height: person_temp.height,
                weight: person_temp.weight,
                birth_date: person_temp.birth_date,
                mlb_debut_date: person_temp.mlb_debut_date,
                birth_city: person_temp.birth_city,
                birth_state_province: person_temp.birth_state_province,
                birth_country: person_temp.birth_country,
                bat_side_code: person_temp.bat_side.code,
                bat_side_description: person_temp.bat_side.description,
                pitch_hand_code: person_temp.pitch_hand.code,
                pitch_hand_description: person_temp.pitch_hand.description,
            }
        }
    }

    #[derive(Debug, Deserialize, Clone)]
    enum SideCode {
        R,
        L,
        S,
    }

    #[derive(Debug, Deserialize, Clone)]
    enum SideDescription {
        Right,
        Left,
        Switch,
        Either,
    }

    #[derive(Debug, Deserialize, Clone)]
    struct Side {
        code: SideCode,
        description: SideDescription,
    }

    let bio_deserialized: Players = serde_json::from_str(&mike_trout_bio).unwrap();

    let from = bio_deserialized.people[0].clone();
    dbg!(&from);

    let into: Person = from.into();
    dbg!(&into);
}
