use client::Player;
use client::Screen;
use client::ScreenType;

use std::option::Option;
pub struct GameBattleShip<'a> {
    player_ship: Player<'a>,
    screen_game: Screen,
    type_view: ScreenType   
}
impl<'a> GameBattleShip<'a> {
    pub fn new() -> Option<Self>{
        let player = match Player::new("Emi") {
            Some(player) => {
                Some(Self {
                    player_ship: player,
                    screen_game: Screen::new(),
                    type_view: ScreenType::screenInitGame
                })
            },
            None => None
        };
        player
    }
    pub fn init_game(&mut self) {        
        loop {
            match self.type_view {
                ScreenType::screenInitGame => {
                   self.type_view = self.screen_game.screen_new_game();
                   ()                                            
                },                                                                            
                ScreenType::screenSearchPlayer => (),
                ScreenType::screenOut => break
            }            
        }
    }
}