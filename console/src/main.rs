use std::io;
use tui::Terminal;
use tui::backend::CrosstermBackend;
use tui::style::{ Style, Color };
use tui::widgets::{ Block, Borders, Paragraph, BorderType, Row, Cell, Table };
use tui::layout::{ Layout, Constraint, Direction, Alignment };
fn main() -> Result<(), io::Error>{
    let stdout = io::stdout();
    let backend = CrosstermBackend::new(stdout);    
    let mut terminal = Terminal::new(backend)?;    
    terminal.clear()?;
    loop {  
        let screen_size = terminal.size().expect("Error al calcular el tamaño de la pantalla");        

        terminal.draw(|f| {           
            //Principal
            let chunks = Layout::default()
                .margin(1)                   
                .direction(Direction::Vertical)                
                .constraints([Constraint::Percentage(10),Constraint::Percentage(85),Constraint::Percentage(20)].as_ref())
                .split(f.size());

            let copyright = Paragraph::new("BATTLE SHIP RUSTEADO")
                .style(Style::default().fg(Color::LightCyan))
                .alignment(Alignment::Center)
                .block(
                    Block::default()                    
                    .borders(Borders::ALL)
                    .title("BATTLE SHIP RUSTEADO")                    
                    .border_style(Style::default().fg(Color::White))
                    .border_type(BorderType::Rounded)
                    .style(Style::default().bg(Color::Black))                                           
                );            
            f.render_widget(copyright, chunks[0]);                                                                   
                              
            let chunks2 = Layout::default()
                .constraints([Constraint::Percentage(40), Constraint::Percentage(30)].as_ref())
                .direction(Direction::Horizontal)
                .split(chunks[1]);
            {
                let chunks3 = Layout::default()
                    .constraints([Constraint::Percentage(100)].as_ref())
                    .direction(Direction::Horizontal)
                    .split(chunks2[0]);
                {
                  
                    let mut body_row: Vec<Row> = vec![];
                    let mut y_axis_init = [32,32,97];        
                    let  x_axis_init = [32,32];                                               
        
                    for y_axis_letter in 97..106 {
                                                 
                        let mut body_cell: Vec<Cell> = vec![];
                        y_axis_init[2] = y_axis_letter;
        
                        match String::from_utf16(&y_axis_init) {
                            Ok(str_y) => body_cell.push(Cell::from(str_y).style(Style::default().bg(Color::DarkGray))),
                            Err(..) => ()
                        } 
                        let mut alter = true;                        
                        for num in 1..10 {                         
                            match String::from_utf16(&x_axis_init) {
                                Ok(str_x) => {
                                    if alter {
                                        body_cell.push(Cell::from(str_x).style(Style::default().bg(Color::Blue)));
                                        alter = false;
                                    }
                                    else{                                    
                                        body_cell.push(Cell::from(str_x).style(Style::default().bg(Color::Green)));
                                        alter = true;
                                    }
                                },
                                Err(..) => ()                                
                            } 
                                                        
                        }
        
                        body_row.push(Row::new(body_cell).height(3));
                    }

                    let tabla = Table::new(body_row)            
                    .style(Style::default().fg(Color::White))            
                    .header(
                        Row::new(vec!["", "1", "2", "3"," 4","5","6","7","8","9"])
                        .style(Style::default().fg(Color::White).bg(Color::DarkGray)).height(3)                                 
                    )
                    .column_spacing(0)            
                    .block(Block::default().title("Table")    
                    .borders(Borders::ALL)
                    .border_style(Style::default().fg(Color::White))
                    .border_type(BorderType::Rounded)
                    .style(Style::default().bg(Color::Black)))                                                            
                    .widths(&[Constraint::Length(6), Constraint::Length(6), Constraint::Length(6),Constraint::Length(6), Constraint::Length(6), Constraint::Length(6),Constraint::Length(6), Constraint::Length(6), Constraint::Length(6),Constraint::Length(6)]);                                                                                  
                    f.render_widget(tabla, chunks3[0]);
                }  
                let copyright2 = Paragraph::new("Version 1.0")
                .style(Style::default().fg(Color::LightCyan))
                .alignment(Alignment::Center)
                .block(
                    Block::default()                    
                    .borders(Borders::ALL)
                    .title("Estadisticas")                    
                    .border_style(Style::default().fg(Color::White))
                    .border_type(BorderType::Rounded)
                    .style(Style::default().bg(Color::Black))                                           
                );                
                let chunks4 = Layout::default()
                .constraints([Constraint::Percentage(100)].as_ref())
                .direction(Direction::Horizontal)
                .split(chunks2[1]);
                f.render_widget(copyright2, chunks4[0]);
            }                   

        })?;
        terminal.autoresize()?;
    }    
}