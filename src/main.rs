fn main() {
    use isahc::prelude::*;
    use serde::Deserialize;
    
    let mut response = isahc::get("https://statsapi.mlb.com/api/v1/people/545361").unwrap();
    let mike_trout_bio = response.text().unwrap();

    #[derive(Deserialize)]
    struct Player {
        
    }
    
    println!("Mike Trout's Bio: {}", mike_trout_bio);
}
