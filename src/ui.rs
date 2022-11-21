use crate::app::App;
use tui::{
    backend::Backend,
    layout::{Constraint, Layout},
    style::{Color, Modifier, Style},
    widgets::{Block, Borders, Cell, Row, Table},
    Frame,
};

pub fn ui<B: Backend>(f: &mut Frame<B>, app: &mut App) {
    let rects = Layout::default()
        .constraints([Constraint::Percentage(100)].as_ref())
        .margin(10)
        .split(f.size());

    let selected_style = Style::default().add_modifier(Modifier::REVERSED);
    let normal_style = Style::default().bg(Color::Gray).fg(Color::Black);

    // TODO improve handling of current sheet
    let current = app.current.as_ref().expect("sheet to be selected").clone();
    let header_cells = current.data[0]
        .iter()
        .map(|h| Cell::from(h.as_str()).style(Style::default().fg(Color::Black).bg(Color::Green)));
    let header = Row::new(header_cells)
        .style(normal_style)
        .height(1)
        .bottom_margin(1);
    let mut constraints: Vec<Constraint> = vec![];

    let t = if let Some(current) = &app.current {
        let rows = current
            .iter()
            .enumerate()
            .filter(|(i, _)| *i != 0)
            .map(|(_, item)| {
                let height = item
                    .iter()
                    .map(|content| content.chars().filter(|c| *c == '\n').count())
                    .max()
                    .unwrap_or(0)
                    + 1;
                let cells = item.iter().map(|c| Cell::from(c.clone()));
                Row::new(cells).height(height as u16).bottom_margin(1)
            });
        constraints = current
            .iter()
            .map(|_| Constraint::Min(10))
            .collect::<Vec<Constraint>>();
        let t = Table::new(rows)
            .header(header)
            .block(Block::default().borders(Borders::ALL).title("Table"))
            .highlight_style(selected_style)
            .highlight_symbol(">> ")
            .widths(&constraints);
        t
    } else {
        Table::new(vec![])
    };

    f.render_stateful_widget(t, rects[0], &mut app.state);
}
