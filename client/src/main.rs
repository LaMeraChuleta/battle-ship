mod game;
use game::GameBattleShip;


fn main() {
    
    if let Some(mut game) = GameBattleShip::new() {
        game.init_game()        
    }   
}