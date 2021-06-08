use std::io;
use tui::Terminal;
use tui::backend::CrosstermBackend;
use tui::text::{ Span, Spans, Text };
use tui::style::{ Style, Color, Modifier };
use tui::widgets::{ Block, Borders, Paragraph, BorderType, List, ListItem, Wrap };
use tui::layout::{ Layout, Constraint, Direction, Rect, Alignment };
fn main() -> Result<(), io::Error>{
    let stdout = io::stdout();
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;
    terminal.clear()?;
    loop {                
        terminal.draw(|f| {                         
            let chunks = Layout::default()
                .margin(1)
                .direction(Direction::Horizontal)
                .constraints([
                        Constraint::Percentage(20),
                        Constraint::Percentage(80)
                    ].as_ref())
                .split(f.size());

            let copyright = Paragraph::new("pet-CLI 2020 - all rights reserved")
                .style(Style::default().fg(Color::LightCyan))
                .alignment(Alignment::Center)
                .block(
                    Block::default()
                    .title("Block")
                    .borders(Borders::LEFT | Borders::RIGHT)
                    .border_style(Style::default().fg(Color::White))
                    .border_type(BorderType::Rounded)
                    .style(Style::default().bg(Color::Black))                       
                );  
            // let text = vec![
            //         Spans::from(vec![
            //             Span::raw("Battle"),
            //             Span::styled("Ship",Style::default().add_modifier(Modifier::ITALIC)),
            //             Span::raw("."),
            //         ]),
            //         Spans::from(Span::styled("Rusteado", Style::default().fg(Color::Red))),
            //     ];
            // let titulo = Paragraph::new(text)
            //         .block(Block::default().borders(Borders::ALL))
            //         .style(Style::default().fg(Color::White).bg(Color::Black))
            //         .alignment(Alignment::Center)
            //         .wrap(Wrap { trim: true });  
            f.render_widget(copyright, chunks[0]);                                                                   
                              
            let chunks2 = Layout::default()
                .constraints([Constraint::Percentage(500), Constraint::Percentage(50)].as_ref())
                .direction(Direction::Horizontal)
                .split(chunks[1] ); 
            let items = [ListItem::new("Item 1"), ListItem::new("Item 2"), ListItem::new("Item 3")];
                
            let tasks = List::new(items)
                .block(Block::default().borders(Borders::ALL).title("List"))
                .highlight_style(Style::default().add_modifier(Modifier::BOLD))
                .highlight_symbol("> ");

            
            f.render_widget(tasks, chunks[1]);                                         
        })?;
        terminal.autoresize()?;
    }    
}