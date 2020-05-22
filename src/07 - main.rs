fn main() {
    use isahc::prelude::*;
    use serde::Deserialize;

    let players: Vec<u32> = vec![
        400124, 400180, 425844, 434564, 435043, 435261,
        435638, 444541, 444886, 446359, 451506, 474029,
        476127, 488722, 501660, 501785, 501922, 502083,
        502154, 518963, 519412, 534584, 534708, 534730,
        543041, 543044, 543059, 543211, 543809, 543819,
        543936, 544253, 545346, 554430, 572227, 581662,
        581683, 594943, 595025, 600350, 605148, 605200,
        605530, 605543, 607389, 608339, 621107, 621545,
        623698, 623967, 626925, 630242, 641470, 641558,
        642066, 643299, 643327, 643335, 650638, 656643,
        657020, 657446, 661457, 661521, 656716, 661827,
        663672, 663749, 664215, 664686, 661841, 666923,
        668676, 668678, 669174, 669342, 667493, 669733,
        673863, 676422, 676646, 676913, 670097, 685358,
        689787,
    ];

    fn to_person (text: &str) -> Person {
        let person_temp: Players = serde_json::from_str(text).unwrap();
        person_temp.people[0].clone().into()
    }

    let persons: Vec<Person> = players.into_iter()
        .map(|player| format!("http://statsapi.mlb.com/api/v1/people/{}", player))
        .map(|url| isahc::get(url).unwrap().text().unwrap())
        .map(|text| to_person(&text) )
        .collect();

    dbg!(&persons[0]);


    #[derive(Debug, Deserialize)]
    struct Players {
        people: Vec<PersonTemp>,
    }

    #[derive(Debug, Deserialize, Clone)]
    #[serde(rename_all="camelCase")]
    struct PersonTemp {
        id: u32,
        full_name: String,
        height: Option<String>,
        weight: Option<u16>,
        birth_date: Option<String>,
        mlb_debut_date: Option<String>,
        birth_city: Option<String>,
        birth_state_province: Option<String>,
        birth_country: Option<String>,
        bat_side: Side,
        pitch_hand: Side,
    }

    #[derive(Debug)]
    struct Person {
        id: u32,
        full_name: String,
        height: Option<String>,
        weight: Option<u16>,
        birth_date: Option<String>,
        mlb_debut_date: Option<String>,
        birth_city: Option<String>,
        birth_state_province: Option<String>,
        birth_country: Option<String>,
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


}
