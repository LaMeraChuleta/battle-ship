use std::str::FromStr;

pub struct Player {    
    player_name: String,
}
impl Default for Player {
    fn default() -> Self {
        Self{
            player_name: String::from_str(&"Emiliano").unwrap()
        }
    }
}



