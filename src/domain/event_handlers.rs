

/// Trait for application operations that event handlers need
#[allow(dead_code)]
pub trait AppOperations {
    /// Update a card
    fn update_card(&mut self, card: crate::core::Card);
    /// Write to file
    fn write_to_file(&mut self, file_name: String);
    /// Navigate to next column
    fn select_next_column(&mut self);
    /// Navigate to previous column
    fn select_prev_column(&mut self);
    /// Navigate to next card
    fn select_next_card(&mut self);
    /// Navigate to previous card
    fn select_prev_card(&mut self);
    /// Disable selection
    fn disable_selection(&mut self);
    /// Get selected card
    fn get_selected_card(&self) -> Option<crate::core::Card>;
    /// Insert card at position
    fn insert_card(&mut self, position: crate::domain::InsertPosition) -> Option<crate::core::Card>;
    /// Remove card
    fn remove_card(&mut self);
    /// Increase priority
    fn increase_priority(&mut self);
    /// Decrease priority
    fn decrease_priority(&mut self);
    /// Mark card as done
    fn mark_card_done(&mut self);
    /// Mark card as undone
    fn mark_card_undone(&mut self);
    /// Write current state
    fn write(&mut self);
}