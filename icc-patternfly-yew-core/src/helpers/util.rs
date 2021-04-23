
use rand::{distributions::Alphanumeric, Rng};


pub fn utils_get_unique_id(prefix: Option<String>) -> String
{
    let pf = prefix.unwrap_or(String::from("pf"));

    // Generate random UID
    let uid: String = rand::thread_rng()
        .sample_iter(&Alphanumeric)
        .take(24)
        .map(char::from)
        .collect();

    format!("{}-{}", pf, uid)
}