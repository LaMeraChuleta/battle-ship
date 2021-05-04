use std::{fmt::format, thread};
use std::time::Duration;
use console::Term;
use console::Style;
use dialoguer::Confirm;
use console::{style};
use dialoguer::Editor;
use console::Emoji;
use dialoguer::{
    Select,
    theme::ColorfulTheme
};
use dialoguer::Input;
use dialoguer::Password;
fn write_chars() -> std::io::Result<()> {
    let term = console::Term::stdout();
    let (heigth, width) = term.size();
    for x in 0..width {
        for y in 0..heigth {
            term.move_cursor_to(x as usize, y as usize)?;
            let text = if (x + y) % 3 == 0 {
                //format!("{}", style(x % 10).black().on_red())
            } else {                
                format!("{}", Emoji::new("🚚","").to_string())
            };
            term.write_str(&text)?;
            
        }
    }
    Ok(())
}


fn main() {
    write_chars();
    let term = Term::stdout();
    println!("{}", term.features().wants_emoji());
    //let msg = format!("[3/4] {} Downloading ...", Emoji::new("🚚", "").to_string());
//println!("[4/4] {} Done!", Emoji("✨", ":-)"));
    term.write_line(&msg).unwrap();    
    thread::sleep(Duration::from_millis(2000));
    term.clear_line().unwrap();
    //term.move_cursor_right(10);

    let password = Password::new().with_prompt("New Password")
        .with_confirmation("Confirm password", "Passwords mismatching")
        .interact().unwrap();
    println!("Length of the password is: {}", password.len());

    let child = std::thread::spawn(move || {
        let term2 = Term::stdout();
        println!("{:?}", term2.features().colors_supported());
        // if Confirm::new().with_prompt("Do you want to continue?").interact().unwrap() {
        //     println!("Looks like you want to continue");
        // } else {
        //     println!("nevermind then :(");
        // }
        

        
        let items = vec!["Item 1", "item 2"];
        let selection = Select::with_theme(&ColorfulTheme::default())
            .items(&items)
            .default(0)
            .interact_on_opt(&Term::stderr()).unwrap();
    
        match selection {
            Some(index) => println!("User selected item : {}", items[index]),
            None => println!("User did not select anything")
        }

        // let input : String = Input::new()
        //     .with_prompt("Tea or coffee?")
        //     .with_initial_text("Yes")
        //     .default("No".into())
        //     .interact_text()
        //     .unwrap();

        term2.write_line("Hello Word 2!!").unwrap();
        let  cyan  =  Style::new().cyan();
        println ! ( "Esto es {} ordenado" , cyan.apply_to( "quite" ));
        thread::sleep(Duration::from_millis(5000));
        term2.clear_line().unwrap();
    });

    let child = child.join();

}