use crate::domain::services::Logger;

/// Mock implementation of Logger for testing
#[derive(Debug)]
pub struct MockLogger {
    pub messages: Vec<String>,
}

impl MockLogger {
    pub fn new() -> Self {
        Self {
            messages: Vec::new(),
        }
    }
    
    /// Get all logged messages
    pub fn get_messages(&self) -> &[String] {
        &self.messages
    }
    
    /// Clear all logged messages
    pub fn clear_messages(&mut self) {
        self.messages.clear();
    }
    
    /// Check if a specific message was logged
    pub fn has_message(&self, message: &str) -> bool {
        self.messages.iter().any(|m| m.contains(message))
    }
}

impl Logger for MockLogger {
    fn log(&mut self, message: &str) {
        self.messages.push(message.to_string());
    }
    
    fn render(&self, _area: ratatui::layout::Rect, _buf: &mut ratatui::buffer::Buffer) {
        // Mock logger doesn't render anything
    }
}