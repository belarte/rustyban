/// Specifies where a new card should be inserted within a column.
///
/// This enum is used by the application's card insertion logic to determine
/// the exact position where a new card should be placed relative to existing
/// cards and the current selection state.
///
/// # Examples
///
/// ```rust
/// use rustyban::InsertPosition;
///
/// // Different insertion strategies
/// let positions = vec![
///     InsertPosition::Top,      // Always insert at the beginning
///     InsertPosition::Bottom,   // Always insert at the end
///     InsertPosition::Current,  // Insert at current selection
///     InsertPosition::Next,     // Insert after current selection
/// ];
///
/// // All variants are Copy and can be compared
/// assert_eq!(InsertPosition::Top, InsertPosition::Top);
/// assert_ne!(InsertPosition::Top, InsertPosition::Bottom);
/// ```
///
/// ## Usage in Application Context
///
/// ```rust
/// use rustyban::{InsertPosition, Board, Card};
/// use chrono::Local;
/// use std::borrow::Cow;
///
/// # fn main() -> rustyban::Result<()> {
/// let mut board = Board::new();
/// let now = Local::now();
///
/// // Add some initial cards
/// board.insert_card(0, 0, Cow::Owned(Card::new("First task", now)))?;
/// board.insert_card(0, 1, Cow::Owned(Card::new("Second task", now)))?;
///
/// // InsertPosition would be used by the UI layer to determine
/// // where to insert new cards based on user interaction
/// let insert_at_top = InsertPosition::Top;
/// let insert_at_bottom = InsertPosition::Bottom;
///
/// // The position enum helps the UI make consistent insertion decisions
/// match insert_at_top {
///     InsertPosition::Top => {
///         // Insert at index 0
///         board.insert_card(0, 0, Cow::Owned(Card::new("New top task", now)))?;
///     }
///     InsertPosition::Bottom => {
///         // Insert at the end
///         let column = board.column(0).unwrap();
///         board.insert_card(0, column.size(), Cow::Owned(Card::new("New bottom task", now)))?;
///     }
///     _ => {} // Handle other positions
/// }
/// # Ok(())
/// # }
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum InsertPosition {
    /// Insert at the current selected position.
    ///
    /// When this position is used, the new card will be inserted at the same
    /// index as the currently selected card, pushing the selected card and
    /// all cards below it down by one position.
    Current,
    
    /// Insert after the current selected position.
    ///
    /// When this position is used, the new card will be inserted immediately
    /// after the currently selected card, at index `selected_index + 1`.
    Next,
    
    /// Insert at the top of the column.
    ///
    /// When this position is used, the new card will always be inserted at
    /// index 0, making it the first card in the column regardless of the
    /// current selection state.
    Top,
    
    /// Insert at the bottom of the column.
    ///
    /// When this position is used, the new card will always be inserted at
    /// the end of the column, making it the last card regardless of the
    /// current selection state.
    Bottom,
}
