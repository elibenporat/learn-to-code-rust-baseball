fn main() {
    use isahc::prelude::*;
    
    let mut response = isahc::get("https://statsapi.mlb.com/api/v1/people/545361").unwrap();
    let mike_trout_bio = response.text().unwrap();
    
    println!("Mike Trout's Bio: {}", mike_trout_bio);
}
