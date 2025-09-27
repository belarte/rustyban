/// Layout constants for the Kanban board UI
pub mod layout {
    /// Percentage width for the left column
    pub const LEFT_COLUMN_WIDTH: u16 = 33;
    
    /// Percentage width for the center column  
    pub const CENTER_COLUMN_WIDTH: u16 = 34;
    
    /// Percentage width for the right column
    pub const RIGHT_COLUMN_WIDTH: u16 = 33;
    
    /// Maximum height for individual cards in a column
    pub const MAX_CARD_HEIGHT: u16 = 4;
    
    /// Maximum number of cards that can be displayed in a column
    pub const MAX_CARDS_PER_COLUMN: usize = 8;
}

/// Constants for popup and dialog sizing
pub mod popup {
    use ratatui::layout::Constraint;
    
    /// Default width for help popup
    pub const HELP_POPUP_WIDTH: Constraint = Constraint::Length(60);
    
    /// Default height for help popup
    pub const HELP_POPUP_HEIGHT: Constraint = Constraint::Length(20);
    
    /// Default width for card editor popup
    pub const CARD_EDITOR_WIDTH: Constraint = Constraint::Length(10);
}
