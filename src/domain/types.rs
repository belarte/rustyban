/// Represents where a new card should be inserted in a column
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum InsertPosition {
    /// Insert at the current selected position
    Current,
    /// Insert after the current selected position
    Next,
    /// Insert at the top of the column
    Top,
    /// Insert at the bottom of the column
    Bottom,
}
