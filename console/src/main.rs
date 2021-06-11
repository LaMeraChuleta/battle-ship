use std::io;
use tui::Terminal;
use tui::backend::CrosstermBackend;
use tui::text::{ Span, Spans, Text };
use tui::style::{ Style, Color, Modifier };
use tui::symbols;
use tui::widgets::{ Block, Borders, Paragraph, BorderType, List, ListItem, Wrap, Axis, Dataset, Chart, GraphType, Row, Cell, Table };
use tui::layout::{ Layout, Constraint, Direction, Rect, Alignment };
fn main() -> Result<(), io::Error>{
    let stdout = io::stdout();
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;
    terminal.clear()?;
    loop {                
        terminal.draw(|f| {           
            //Pricnipal
            let chunks = Layout::default()
                .margin(1)                   
                .direction(Direction::Vertical)                
                .constraints([
                        Constraint::Percentage(20),
                        Constraint::Percentage(80)
                    ].as_ref())
                .split(f.size());

            let copyright = Paragraph::new("BATTLE SHIP RUSTEADO")
                .style(Style::default().fg(Color::LightCyan))
                .alignment(Alignment::Center)
                .block(
                    Block::default()                    
                    .borders(Borders::ALL)
                    .title("Block")                    
                    .border_style(Style::default().fg(Color::White))
                    .border_type(BorderType::Rounded)
                    .style(Style::default().bg(Color::Black))                       
                );            
            f.render_widget(copyright, chunks[0]);                                                                   
                              
            let chunks2 = Layout::default()
                .constraints([Constraint::Percentage(70), Constraint::Percentage(30)].as_ref())
                .direction(Direction::Horizontal)
                .split(chunks[1]);
            {
                let chunks2 = Layout::default()
                    .constraints([Constraint::Percentage(120), Constraint::Percentage(50)].as_ref())
                    .direction(Direction::Horizontal)
                    .split(chunks2[0]);
                {
                    let chunks2 = Layout::default()
                        .constraints([Constraint::Percentage(120), Constraint::Percentage(50)].as_ref())
                        .direction(Direction::Horizontal)
                        .split(chunks2[0]);  
                    
                    let datasets = vec![
                        Dataset::default()
                            .name("data1")
                            .marker(symbols::Marker::Block)
                            .graph_type(GraphType::Scatter)
                            .style(Style::default().fg(Color::Cyan))
                            .data(&[(0.0, 5.0),(0.0, 6.0)]),
                        Dataset::default()
                            .name("data2")
                            .marker(symbols::Marker::Block)
                            .graph_type(GraphType::Scatter)
                            .style(Style::default().fg(Color::Magenta))
                            .data(&[(2.0, 5.0),(2.0, 7.0)]),
                    ];

                    let tablero = Chart::new(datasets)       
                    .block(      Block::default()                    
                        .borders(Borders::ALL)
                        .title("Tablero")                    
                        .border_style(Style::default().fg(Color::White))
                        .border_type(BorderType::Rounded)
                        .style(Style::default().bg(Color::Black))
                    )
                    .x_axis(Axis::default()
                        // .title(Span::styled("X Axis", Style::default().fg(Color::Red)))
                        .style(Style::default().fg(Color::White))
                        .bounds([0.0, 10.0])
                        .labels(["0.0", "1.0", "2.0", "3.0", "4.0", "5.0", "6.0", "7.0", "8.0", "9.0", "10.0"].iter().cloned().map(Span::from).collect()))
                    .y_axis(Axis::default()
                        // .title(Span::styled("Y Axis", Style::default().fg(Color::Red)))
                        .style(Style::default().fg(Color::White))
                        .bounds([0.0, 10.0])
                        .labels(["0.0", "1.0", "2.0", "3.0", "4.0", "5.0", "6.0", "7.0", "8.0", "9.0", "10.0"].iter().cloned().map(Span::from).collect()));

                    f.render_widget(tablero, chunks2[0]);             
                }                   
            }  
            let tabla = Table::new(vec![
                // Row can be created from simple strings.
                Row::new(vec!["", "1", "2", "3","4","5","6","7","8","9"]).style(Style::default().fg(Color::Blue)).height(1),
                // You can style the entire row.
                
                // If you need more control over the styling you may need to create Cells directly
                Row::new(vec![
                    Cell::from("Row31"),
                    Cell::from("Row32").style(Style::default().fg(Color::Yellow)),                    
                    Cell::from("Row32").style(Style::default().fg(Color::Yellow)),
                    Cell::from("Row32").style(Style::default().fg(Color::Yellow)),
                    Cell::from("Row32").style(Style::default().fg(Color::Yellow)),
                    Cell::from("Row32").style(Style::default().fg(Color::Yellow)),
                    Cell::from("Row32").style(Style::default().fg(Color::Yellow)),
                    Cell::from("Row32").style(Style::default().fg(Color::Yellow)),
                    Cell::from("Row32").style(Style::default().fg(Color::Yellow)),
                ]),
                // If a Row need to display some content over multiple lines, you just have to change
                // its height.
                Row::new(vec![
                    Cell::from("Row\n41"),
                    Cell::from("Row\n42"),
                    Cell::from("Row\n43"),
                    Cell::from("Row32").style(Style::default().fg(Color::Yellow)),
                    Cell::from("Row32").style(Style::default().fg(Color::Yellow)),
                    Cell::from("Row32").style(Style::default().fg(Color::Yellow)),
                    Cell::from("Row32").style(Style::default().fg(Color::Yellow)),
                    Cell::from("Row32").style(Style::default().fg(Color::Yellow)),
                    Cell::from("Row32").style(Style::default().fg(Color::Yellow)),
                    Cell::from("Row32").style(Style::default().fg(Color::Yellow)),
                ]).height(2),
            ])
            // You can set the style of the entire Table.
            .style(Style::default().fg(Color::White))
            // It has an optional header, which is simply a Row always visible at the top.
            .header(
                Row::new(vec!["", "1", "2", "3","4","5","6","7","8","9"])
                    .style(Style::default().fg(Color::Yellow))
                    // If you want some space between the header and the rest of the rows, you can always
                    // specify some margin at the bottom.
                    .bottom_margin(1)
            )
            // As any other widget, a Table can be wrapped in a Block.
            .block(Block::default().title("Table"))
            // Columns widths are constrained in the same way as Layout...
            .widths(&[Constraint::Length(5), Constraint::Length(5), Constraint::Length(5),Constraint::Length(5), Constraint::Length(5), Constraint::Length(5),Constraint::Length(5), Constraint::Length(5), Constraint::Length(5),Constraint::Length(5)])
            // ...and they can be separated by a fixed spacing.
            .column_spacing(1)
            // If you wish to highlight a row in any specific way when it is selected...
            .highlight_style(Style::default().add_modifier(Modifier::BOLD))
            // ...and potentially show a symbol in front of the selection.
            .highlight_symbol(">>");
            
            let block = Block::default().borders(Borders::ALL).title("Graphs");     
            f.render_widget(tabla, chunks2[1]);
        })?;
        terminal.autoresize()?;
    }    
}