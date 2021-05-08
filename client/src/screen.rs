use std::{thread, time::Duration};
use dialoguer::{ Select, theme::ColorfulTheme};
use indicatif::{ProgressBar, ProgressStyle};
use console::Term;
use console::Emoji;
pub struct Screen {
    current_view: Term
}
pub enum ScreenType {
    ScreenInitGame,
    ScreenSearchPlayer,
    ScreenWait,
    ScreenOut
}
impl Default for Screen {
    fn default() -> Self {
        Screen { current_view: Term::stdout() }  
    }
}
impl Screen {   
    pub fn screen_new_game(&mut self) -> ScreenType {     
        let title = format!("{}  BattleShip Rusteado  {}", Emoji("🌊",""), Emoji("🌊",""));
        let opcion_game_view = vec!["📺 Nuevo Juego", "🚀 Salir"];
        let selection = Select::with_theme(&ColorfulTheme::default())
            .with_prompt(&title)
            .items(&opcion_game_view)
            .default(0)
            .interact_on_opt(&Term::stdout()).unwrap();           
        let screen = match selection {
            Some(index) => {
                if index == 0{
                    return ScreenType::ScreenSearchPlayer;
                }
                ScreenType::ScreenOut
            },
            None => ScreenType::ScreenOut            
        };                                                 
        screen
    }
    pub fn screen_load_oponnent(&mut self){                     
        let bar = ProgressBar::new(20);
        bar.set_style(ProgressStyle::default_bar()
            .template("[{elapsed_precise}] {bar:40.cyan/blue} {pos:>7}/{len:7} {msg}")
            .progress_chars("##-"));
        for _ in 0..20 {
            bar.inc(1);
            thread::sleep(Duration::from_millis(300));                        
        }
        bar.finish();  
        self.current_view.clear_to_end_of_screen().unwrap()                  
    }
}



