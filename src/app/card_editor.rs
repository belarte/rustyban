use ratatui::{
    buffer::Buffer,
    layout::{Alignment, Constraint, Layout, Rect},
    style::Stylize,
    symbols::border,
    text::Line,
    widgets::{
        block::{Position, Title},
        Block, Clear, Widget,
    },
};
use tui_textarea::Input;

use crate::app::text_widget::TextWidget;
use crate::app::widget_utils::centered_popup_area;
use crate::board::Card;

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
                "Short description".into(),
                card.short_description().to_string(),
                Constraint::Length(3),
                true,
            ),
            TextWidget::new(
                "Long description".into(),
                card.long_description().to_string(),
                Constraint::Length(10),
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
        let card = self.card.clone();
        let short_description = self.widgets[0].lines().join("\n");
        let long_description = self.widgets[1].lines().join("\n");
        let card = Card::update_short_description(card, &short_description);

        Card::update_long_description(card, &long_description)
    }

    fn areas(&self, area: Rect) -> [Rect; 2] {
        let constraints: Vec<Constraint> = self.widgets.iter().map(|widget| widget.constaint()).collect();
        Layout::vertical(constraints).areas(area)
    }
}

const WIDGET_HEIGHT: u16 = 15;
const WIDGET_WIDTH: u16 = 64;

impl Widget for &CardEditor {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let area = centered_popup_area(
            area,
            Constraint::Length(WIDGET_WIDTH),
            Constraint::Length(WIDGET_HEIGHT),
        );
        Clear.render(area, buf);

        let block = Block::bordered()
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
            .border_set(border::PLAIN);
        let inner_area = block.inner(area);
        block.render(area, buf);

        let areas = self.areas(inner_area);

        for (widget, area) in self.widgets.iter().zip(areas.iter()) {
            widget.render(*area, buf);
        }
    }
}
