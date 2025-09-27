use ratatui::{
    buffer::Buffer,
    layout::{Alignment, Constraint, Layout, Rect},
    style::Stylize,
    symbols::border,
    text::Line,
    widgets::{
        block::{Position, Title},
        Block, Clear, Paragraph, Widget,
    },
};
use tui_textarea::Input;

use crate::domain::centered_popup_area;
use crate::core::Card;
use crate::{ui::text_widget::TextWidget, utils::time, domain::constants::popup};

#[derive(Debug, Clone)]
pub struct CardEditor {
    widgets: Vec<TextWidget>,
    selected: usize,
    card: Card,
}

impl PartialEq for CardEditor {
    fn eq(&self, _other: &Self) -> bool {
        true
    }
}

impl Eq for CardEditor {}

impl CardEditor {
    pub fn new(card: Card) -> Self {
        let widgets = vec![
            TextWidget::new(
                "Short description",
                card.short_description(),
                Constraint::Length(3),
                true,
            ),
            TextWidget::new(
                "Long description",
                card.long_description(),
                popup::CARD_EDITOR_WIDTH,
                false,
            ),
        ];

        Self {
            widgets,
            selected: 0,
            card,
        }
    }

    pub fn input(&mut self, input: Input) {
        self.widgets[self.selected].input(input);
    }

    pub fn next_field(&mut self) {
        self.widgets[self.selected].select(false);
        self.selected = (self.selected + 1) % self.widgets.len();
        self.widgets[self.selected].select(true);
    }

    pub fn get_card(&self) -> Card {
        let mut card = self.card.clone();
        let short_description = self.widgets[0].lines().join("\n");
        let long_description = self.widgets[1].lines().join("\n");
        card.update_short_description(&short_description);

        card.update_long_description(&long_description);
        card
    }

    fn areas(&self, area: Rect) -> [Rect; 3] {
        let mut constraints: Vec<Constraint> = self.widgets.iter().map(|widget| widget.constaint()).collect();
        constraints.push(Constraint::Min(1));
        Layout::vertical(constraints).areas(area)
    }
}

const WIDGET_HEIGHT: u16 = 16;
const WIDGET_WIDTH: u16 = 64;

impl Widget for &CardEditor {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let area = centered_popup_area(
            area,
            Constraint::Length(WIDGET_WIDTH),
            Constraint::Length(WIDGET_HEIGHT),
        );
        Clear.render(area, buf);

        let block = surrounding_block();
        let inner_area = block.inner(area);
        block.render(area, buf);

        let areas = self.areas(inner_area);
        let [short_desc_area, long_desc_area, date_area] = areas;

        self.widgets[0].render(short_desc_area, buf);
        self.widgets[1].render(long_desc_area, buf);
        creation_date_widget(&self.card).render(date_area, buf);
    }
}

fn surrounding_block() -> Block<'static> {
    Block::bordered()
        .title(Title::from(" Edit card ".bold()).alignment(Alignment::Center))
        .title(
            Title::from(Line::from(vec![
                " <Ctrl-s> ".bold(),
                "Save -".into(),
                " <ESC> ".bold(),
                "Discard changes ".into(),
            ]))
            .alignment(Alignment::Center)
            .position(Position::Bottom),
        )
        .on_blue()
        .border_set(border::PLAIN)
}

fn creation_date_widget(card: &Card) -> Paragraph<'_> {
    let creation_date_text = Line::from(vec![
        " Creation date: ".bold(),
        time::format(card.creation_date()).into(),
    ]);
    Paragraph::new(creation_date_text)
}
