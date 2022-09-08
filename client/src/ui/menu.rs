use tui::{
    backend::Backend,
    layout::Rect,
    style::{Color, Modifier, Style},
    text::{Span, Spans},
    widgets::{Block, Borders, Tabs, Widget},
    Frame,
};

pub enum MenuItem {
    RoomList,
    Room { index: usize, name: String },
}

impl MenuItem {
    pub fn get_tab_text(&self) -> String {
        match self {
            MenuItem::RoomList => String::from("Rooms"),
            MenuItem::Room { index, name } => format!("{}. {}", index + 1, name),
        }
    }
}

pub struct Menu {
    active_index: usize, // index of item in items
    items: Vec<MenuItem>,
}

impl Default for Menu {
    fn default() -> Self {
        Self {
            active_index: 0,
            items: vec![MenuItem::RoomList],
        }
    }
}

impl Menu {
    pub fn render<B: Backend>(&self, frame: &mut Frame<B>, area: Rect) {
        let menu_titles: Vec<String> = self
            .items
            .iter()
            .map(|menu_item| menu_item.get_tab_text())
            .collect();

        let menu = menu_titles
            .iter()
            .map(|t| {
                let (first, rest) = t.split_at(1);
                Spans::from(vec![
                    Span::styled(
                        first,
                        Style::default()
                            .fg(Color::Yellow)
                            .add_modifier(Modifier::UNDERLINED),
                    ),
                    Span::styled(rest, Style::default().fg(Color::White)),
                ])
            })
            .collect();

        let tabs = Tabs::new(menu)
            .select(self.active_index)
            .block(Block::default().title("Menu").borders(Borders::ALL))
            .style(Style::default().fg(Color::White))
            .highlight_style(Style::default().fg(Color::Yellow))
            .divider(Span::raw("|"));

        frame.render_widget(tabs, area);
    }
}
