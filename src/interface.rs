use tui::{
    layout::Alignment,
    style::{Color, Modifier, Style},
    symbols,
    text::{Span, Spans},
    widgets::{Block, BorderType, Borders, List, ListItem, ListState, Paragraph, Tabs},
};

pub struct Lists {
    pub selected: usize,
    pub lists: Vec<String>,
}

impl Lists {
    pub fn next(&mut self) {
        if self.selected >= self.lists.len() - 1 {
            self.selected = 0;
        } else {
            self.selected += 1;
        }
    }

    pub fn previous(&mut self) {
        if self.selected == 0 {
            self.selected = self.lists.len() - 1;
        } else {
            self.selected -= 1;
        }
    }

    // returns a string of the current selected item
    pub fn repr(&self) -> String {
        match self.lists.get(self.selected) {
            Some(x) => x.clone(),
            None => "Error".to_string(),
        }
    }

    // Widget which displays the lists the user can choose and highlights the currently selected one
    pub fn render_menu(&self) -> Tabs<'static> {
        // converts the selected list into a vec of spans
        Tabs::new(self.lists.iter().cloned().map(Spans::from).collect())
            .block(
                Block::default()
                    .title(Span::styled(
                        "Lists",
                        Style::default()
                            .fg(Color::Blue)
                            .add_modifier(Modifier::BOLD | Modifier::UNDERLINED),
                    ))
                    .title_alignment(Alignment::Center)
                    .borders(Borders::ALL),
            )
            .style(Style::default().fg(Color::White))
            .highlight_style(
                Style::default()
                    .fg(Color::Blue)
                    .add_modifier(Modifier::BOLD),
            )
            .divider(symbols::DOT)
            .select(self.selected)
    }
}

impl Default for Lists {
    fn default() -> Self {
        Lists {
            selected: 0,
            lists: vec![],
        }
    }
}

// allows you to create a `Lists` from a vec of &str
impl From<Vec<&str>> for Lists {
    fn from(values: Vec<&str>) -> Self {
        Lists {
            selected: 0,
            lists: values.iter().map(|x| x.to_string()).collect(),
        }
    }
}

// allows you to create a `Lists` from a vec of String
impl From<Vec<String>> for Lists {
    fn from(values: Vec<String>) -> Self {
        Lists {
            selected: 0,
            lists: values,
        }
    }
}

// allows you to clone a list to move around
impl Clone for Lists {
    fn clone(&self) -> Lists {
        Lists {
            selected: self.selected,
            lists: self.lists.clone(),
        }
    }
}

pub struct Entries {
    pub state: ListState,
    pub entries: Vec<String>,
}

impl Entries {
    // selects the provided entry
    pub fn select(&mut self, i: usize) {
        self.state.select(Some(i));
    }

    // updates ListState to next
    pub fn next(&mut self) {
        let len = self.entries.len();
        if len == 0 {
            return;
        }
        self.state.select(Some(match self.state.selected() {
            Some(s) => {
                if s >= len - 1 {
                    0
                } else {
                    s + 1
                }
            }
            None => 0, // "Shouldn't ever happen"
        }));
    }

    // updates ListState to previous
    pub fn previous(&mut self) {
        let len = self.entries.len();
        if len == 0 {
            return;
        }
        self.state.select(Some(match self.state.selected() {
            Some(s) => {
                if s == 0 {
                    self.entries.len() - 1
                } else {
                    s - 1
                }
            }
            None => 0, // "Shouldn't ever happen"
        }));
    }

    pub fn render_entries(&self, title: String) -> List {
        let items: Vec<ListItem> = self
            .entries
            .iter()
            .map(|i| ListItem::new(i.as_ref()))
            .collect();

        List::new(items)
            .block(
                Block::default()
                    .title(Span::styled(
                        title,
                        Style::default()
                            .fg(Color::Cyan)
                            .add_modifier(Modifier::BOLD),
                    ))
                    .borders(Borders::ALL),
            )
            .style(Style::default().fg(Color::White))
            .highlight_style(
                Style::default()
                    .fg(Color::Blue)
                    .add_modifier(Modifier::ITALIC),
            )
            .highlight_symbol("> ")
    }
}

impl Default for Entries {
    fn default() -> Self {
        let mut s = ListState::default();
        s.select(Some(0));
        Entries {
            state: s,
            entries: vec![],
        }
    }
}

// allows you to create an `Entries` from a vec of &str
impl From<Vec<&str>> for Entries {
    fn from(values: Vec<&str>) -> Self {
        let mut s = ListState::default();
        s.select(Some(0));
        Entries {
            state: s,
            entries: values.iter().map(|x| x.to_string()).collect(),
        }
    }
}

// allows you to create an `Entries` from a vec of String
impl From<Vec<String>> for Entries {
    fn from(values: Vec<String>) -> Self {
        let mut s = ListState::default();
        s.select(Some(0));
        Entries {
            state: s,
            entries: values,
        }
    }
}

pub struct Copyright {
    pub message: String,
}

impl Copyright {
    pub fn render_copyright(&self) -> Paragraph<'static> {
        Paragraph::new(self.message.clone())
            .style(Style::default().fg(Color::LightCyan))
            .alignment(Alignment::Center)
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .style(Style::default().fg(Color::White))
                    .title("Copyright")
                    .border_type(BorderType::Rounded),
            )
    }
}

// allows you to create a `Copyright` from a &str
impl From<&str> for Copyright {
    fn from(m: &str) -> Self {
        Copyright {
            message: m.to_string(),
        }
    }
}
