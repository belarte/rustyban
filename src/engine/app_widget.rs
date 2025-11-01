use ratatui::{
    buffer::Buffer,
    layout::{Constraint, Layout, Rect},
    style::Stylize,
    text::Line,
    widgets::Widget,
};

use super::App;

impl Widget for &App {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let [title_area, board_area, logger_area, bottom_area] = Layout::vertical([
            Constraint::Length(1),
            Constraint::Min(0),
            Constraint::Length(3),
            Constraint::Length(1),
        ])
        .areas(area);

        let [status_area, instructions_area] =
            Layout::horizontal([Constraint::Percentage(50), Constraint::Percentage(50)]).areas(bottom_area);

        let title = Line::from(" Welcome ".bold()).centered();
        title.render(title_area, buf);

        let instructions = Line::from(vec![
            " Help ".into(),
            "<?> ".blue().bold(),
            "Quit ".into(),
            "<q> ".blue().bold(),
        ])
        .centered();
        instructions.render(instructions_area, buf);

        self.board().as_ref().borrow().render(board_area, buf);
        self.logger().render(logger_area, buf);
        render_status_bar(self, status_area, buf);
    }
}

fn render_status_bar(app: &App, area: Rect, buf: &mut Buffer) {
    let mut spans = Vec::new();

    if app.can_undo() {
        spans.push("Undo ".into());
        spans.push("<u> ".blue().bold());
        if let Some(desc) = app.last_undo_description() {
            spans.push(format!("({}) ", desc).into());
        }
    } else {
        spans.push("Undo ".dim());
        spans.push("<u> ".dim());
    }

    spans.push("| ".dim());

    if app.can_redo() {
        spans.push("Redo ".into());
        spans.push("<Ctrl-r> ".blue().bold());
        if let Some(desc) = app.last_redo_description() {
            spans.push(format!("({}) ", desc).into());
        }
    } else {
        spans.push("Redo ".dim());
        spans.push("<Ctrl-r> ".dim());
    }

    let status_line = Line::from(spans).centered();
    status_line.render(area, buf);
}
