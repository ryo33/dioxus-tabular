/// Manages the order and visibility of columns in a table.
#[derive(Clone, PartialEq, Debug)]
pub struct ColumnOrder {
    /// The order of visible columns (indices into the column tuple)
    order: Vec<usize>,
    /// Total number of columns available
    total_columns: usize,
}

impl ColumnOrder {
    /// Creates a new ColumnOrder with default ordering (all columns visible in natural order)
    pub fn new(total_columns: usize) -> Self {
        Self {
            order: (0..total_columns).collect(),
            total_columns,
        }
    }

    /// Returns the current column order as a slice
    pub fn get_order(&self) -> &[usize] {
        &self.order
    }

    /// Returns the total number of columns
    pub fn total_columns(&self) -> usize {
        self.total_columns
    }

    /// Swaps two columns in the display order.
    /// If either column is hidden or out of bounds, this is a no-op (saturating behavior).
    pub fn swap(&mut self, col_a: usize, col_b: usize) {
        // Saturate to valid column indices
        let col_a = col_a.min(self.total_columns.saturating_sub(1));
        let col_b = col_b.min(self.total_columns.saturating_sub(1));

        // Find positions in the order vector
        let pos_a = self.order.iter().position(|&c| c == col_a);
        let pos_b = self.order.iter().position(|&c| c == col_b);

        // Only swap if both are visible
        if let (Some(pos_a), Some(pos_b)) = (pos_a, pos_b) {
            self.order.swap(pos_a, pos_b);
        }
    }

    /// Hides a column by removing it from the display order.
    /// If the column is already hidden or out of bounds, this is a no-op.
    pub fn hide_column(&mut self, col: usize) {
        // Saturate to valid column index
        let col = col.min(self.total_columns.saturating_sub(1));

        // Remove from order if present
        self.order.retain(|&c| c != col);
    }

    /// Shows a column by inserting it into the display order.
    /// If at_index is None, appends to the end.
    /// If at_index is Some(idx), inserts at that position (saturated to valid range).
    /// If the column is already visible or out of bounds, this is a no-op.
    pub fn show_column(&mut self, col: usize, at_index: Option<usize>) {
        // Saturate to valid column index
        let col = col.min(self.total_columns.saturating_sub(1));

        // If already visible, do nothing
        if self.order.contains(&col) {
            return;
        }

        // Insert at specified position or append
        match at_index {
            None => self.order.push(col),
            Some(idx) => {
                let insert_pos = idx.min(self.order.len());
                self.order.insert(insert_pos, col);
            }
        }
    }

    /// Moves a column to a specific position in the display order (0-indexed).
    /// The position is saturated to the valid range.
    /// If the column is hidden or out of bounds, this is a no-op.
    pub fn move_to(&mut self, col: usize, new_index: usize) {
        // Saturate to valid column index
        let col = col.min(self.total_columns.saturating_sub(1));

        // Find current position
        if let Some(current_pos) = self.order.iter().position(|&c| c == col) {
            // Remove from current position
            self.order.remove(current_pos);

            // Insert at new position (saturated)
            let insert_pos = new_index.min(self.order.len());
            self.order.insert(insert_pos, col);
        }
    }

    /// Moves a column one position forward (towards index 0) in the display order.
    /// If the column is already first or hidden, this is a no-op.
    pub fn move_forward(&mut self, col: usize) {
        // Saturate to valid column index
        let col = col.min(self.total_columns.saturating_sub(1));

        if let Some(pos) = self.order.iter().position(|&c| c == col) && pos > 0 {
            self.order.swap(pos, pos - 1);
        }
    }

    /// Moves a column one position backward (towards the end) in the display order.
    /// If the column is already last or hidden, this is a no-op.
    pub fn move_backward(&mut self, col: usize) {
        // Saturate to valid column index
        let col = col.min(self.total_columns.saturating_sub(1));

        if let Some(pos) = self.order.iter().position(|&c| c == col) && pos < self.order.len() - 1 {
            self.order.swap(pos, pos + 1);
        }
    }

    /// Checks if a column is currently visible
    pub fn is_visible(&self, col: usize) -> bool {
        // Saturate to valid column index
        let col = col.min(self.total_columns.saturating_sub(1));
        self.order.contains(&col)
    }

    /// Returns the display position of a column (0-indexed), or None if hidden
    pub fn position(&self, col: usize) -> Option<usize> {
        // Saturate to valid column index
        let col = col.min(self.total_columns.saturating_sub(1));
        self.order.iter().position(|&c| c == col)
    }

    /// Resets the column order to the default state (all columns visible in natural order)
    pub fn reset(&mut self) {
        self.order = (0..self.total_columns).collect();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let order = ColumnOrder::new(3);
        assert_eq!(order.get_order(), &[0, 1, 2]);
        assert_eq!(order.total_columns(), 3);
    }

    #[test]
    fn test_swap() {
        let mut order = ColumnOrder::new(3);
        order.swap(0, 2);
        assert_eq!(order.get_order(), &[2, 1, 0]);
    }

    #[test]
    fn test_swap_saturating() {
        let mut order = ColumnOrder::new(3);
        order.swap(0, 100); // Should saturate to 2
        assert_eq!(order.get_order(), &[2, 1, 0]);
    }

    #[test]
    fn test_hide_show() {
        let mut order = ColumnOrder::new(3);

        order.hide_column(1);
        assert_eq!(order.get_order(), &[0, 2]);
        assert!(!order.is_visible(1));

        order.show_column(1, None);
        assert_eq!(order.get_order(), &[0, 2, 1]);
        assert!(order.is_visible(1));
    }

    #[test]
    fn test_show_at_index() {
        let mut order = ColumnOrder::new(3);
        order.hide_column(1);
        order.show_column(1, Some(0));
        assert_eq!(order.get_order(), &[1, 0, 2]);
    }

    #[test]
    fn test_move_to() {
        let mut order = ColumnOrder::new(3);
        order.move_to(0, 2);
        assert_eq!(order.get_order(), &[1, 2, 0]);
    }

    #[test]
    fn test_move_forward_backward() {
        let mut order = ColumnOrder::new(3);

        order.move_backward(0);
        assert_eq!(order.get_order(), &[1, 0, 2]);

        order.move_forward(0);
        assert_eq!(order.get_order(), &[0, 1, 2]);
    }

    #[test]
    fn test_move_forward_at_start() {
        let mut order = ColumnOrder::new(3);
        order.move_forward(0);
        assert_eq!(order.get_order(), &[0, 1, 2]); // No change
    }

    #[test]
    fn test_move_backward_at_end() {
        let mut order = ColumnOrder::new(3);
        order.move_backward(2);
        assert_eq!(order.get_order(), &[0, 1, 2]); // No change
    }

    #[test]
    fn test_position() {
        let mut order = ColumnOrder::new(3);
        assert_eq!(order.position(1), Some(1));

        order.hide_column(1);
        assert_eq!(order.position(1), None);
    }

    #[test]
    fn test_reset() {
        let mut order = ColumnOrder::new(3);

        // Make some changes
        order.hide_column(1);
        order.swap(0, 2);
        assert_eq!(order.get_order(), &[2, 0]);

        // Reset should restore default order
        order.reset();
        assert_eq!(order.get_order(), &[0, 1, 2]);
        assert!(order.is_visible(1));
    }
}
