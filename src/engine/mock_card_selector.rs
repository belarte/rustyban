use crate::core::Card;
use crate::domain::services::CardSelector;

/// Mock implementation of CardSelector for testing
#[derive(Debug)]
pub struct MockCardSelector {
    pub selection: Option<(usize, usize)>,
    pub selected_card: Option<Card>,
    pub selection_enabled: bool,
    pub navigation_calls: Vec<String>,
}

impl MockCardSelector {
    #[allow(dead_code)]
    pub fn new() -> Self {
        Self {
            selection: None,
            selected_card: None,
            selection_enabled: false,
            navigation_calls: Vec::new(),
        }
    }
    
    #[allow(dead_code)]
    pub fn with_selection(mut self, column: usize, card: usize) -> Self {
        self.selection = Some((column, card));
        self.selection_enabled = true;
        self
    }
    
    #[allow(dead_code)]
    pub fn with_selected_card(mut self, card: Card) -> Self {
        self.selected_card = Some(card);
        self
    }
    
    #[allow(dead_code)]
    pub fn get_navigation_calls(&self) -> &[String] {
        &self.navigation_calls
    }
    
    #[allow(dead_code)]
    pub fn clear_navigation_calls(&mut self) {
        self.navigation_calls.clear();
    }
    
    #[allow(dead_code)]
    pub fn has_navigation_call(&self, call: &str) -> bool {
        self.navigation_calls.iter().any(|c| c == call)
    }
}

impl CardSelector for MockCardSelector {
    fn get(&self) -> Option<(usize, usize)> {
        if self.selection_enabled {
            self.selection
        } else {
            None
        }
    }
    
    fn set(&mut self, column_index: usize, card_index: usize) {
        self.selection = Some((column_index, card_index));
        self.selection_enabled = true;
    }
    
    fn get_selected_card(&self) -> Option<Card> {
        if self.selection_enabled {
            self.selected_card.clone()
        } else {
            None
        }
    }
    
    fn select_next_column(&mut self) -> (usize, usize) {
        self.navigation_calls.push("select_next_column".to_string());
        self.selection_enabled = true;
        self.selection.unwrap_or((0, 0))
    }
    
    fn select_prev_column(&mut self) -> (usize, usize) {
        self.navigation_calls.push("select_prev_column".to_string());
        self.selection_enabled = true;
        self.selection.unwrap_or((0, 0))
    }
    
    fn select_next_card(&mut self) -> (usize, usize) {
        self.navigation_calls.push("select_next_card".to_string());
        self.selection_enabled = true;
        self.selection.unwrap_or((0, 0))
    }
    
    fn select_prev_card(&mut self) -> (usize, usize) {
        self.navigation_calls.push("select_prev_card".to_string());
        self.selection_enabled = true;
        self.selection.unwrap_or((0, 0))
    }
    
    fn disable_selection(&mut self) {
        self.selection_enabled = false;
    }
    
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}