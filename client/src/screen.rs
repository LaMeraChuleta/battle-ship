use dialoguer::{ Select, theme::ColorfulTheme};
use console::Term;
use console::Emoji;

pub struct Screen {
    currentView: Term
}
pub enum ScreenType {
    screenInitGame,
    screenSearchPlayer,
    screenOut
}
impl Screen {
    pub fn new() -> Self {
        Self {
            currentView: Term::stdout()
        }      
    } 
    pub fn screen_new_game(&mut self) -> ScreenType {     
        let title = format!("{}  Bienvenido  {}", Emoji("🌊",""), Emoji("🌊",""));
        self.currentView
            .write_line(&title)
            .unwrap();
        let opcion_game_view = vec!["📺 Nuevo Juego", "🚀 Salir"];
        let selection = Select::with_theme(&ColorfulTheme::default())
            .items(&opcion_game_view)
            .default(0)
            .interact_on_opt(&Term::stderr()).unwrap();            
        match selection {
            Some(index) => {
                match index {
                    0 => {
                        self.currentView.clear_screen().unwrap();
                        println!("Conectando Jugador.... {}", opcion_game_view[index]);
                        return ScreenType::screenInitGame;                     
                    },
                    _ => {
                        return ScreenType::screenOut
                    }
                }
            },
            None =>{                 
                return ScreenType::screenOut         
            }
        }                  
    }
}



