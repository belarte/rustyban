use ratatui::layout::{Constraint, Flex, Layout, Rect};

/// Create a centered popup area within the given bounds
pub fn centered_popup_area(area: Rect, horizontal: Constraint, vertical: Constraint) -> Rect {
    let vertical = Layout::vertical([vertical]).flex(Flex::Center);
    let horizontal = Layout::horizontal([horizontal]).flex(Flex::Center);
    let [area] = vertical.areas(area);
    let [area] = horizontal.areas(area);
    area
}

#[cfg(test)]
mod tests {
    use super::centered_popup_area;
    use ratatui::layout::{Constraint, Rect};

    #[test]
    fn popup_area() {
        let base_area = Rect::new(16, 32, 64, 128);

        let area = centered_popup_area(base_area, Constraint::Length(64), Constraint::Length(128));
        assert_eq!(area, Rect::new(16, 32, 64, 128));

        let area = centered_popup_area(base_area, Constraint::Length(16), Constraint::Length(32));
        assert_eq!(area, Rect::new(40, 80, 16, 32));

        let area = centered_popup_area(base_area, Constraint::Percentage(100), Constraint::Percentage(100));
        assert_eq!(area, Rect::new(16, 32, 64, 128));

        let area = centered_popup_area(base_area, Constraint::Percentage(50), Constraint::Percentage(50));
        assert_eq!(area, Rect::new(32, 64, 32, 64));
    }
}