/// This module defines the `InferableValue` struct, which represents a value that can be inferred
#[non_exhaustive]
pub struct InferableValue {
    pub(crate) value: String,
    pub(crate) row_number: usize,
    pub(crate) column_index: usize,
}

impl InferableValue {
    /// Creates a new instance of `InferableValue`.
    #[inline]
    #[must_use]
    pub const fn new(value: String, row_number: usize, column_index: usize) -> Self {
        Self {
            value,
            row_number,
            column_index,
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[tokio::test]
    async fn test_batch_data() {
        let batch_data = InferableValue {
            value: "test".into(),
            row_number: 1,
            column_index: 2,
        };
        assert_eq!(batch_data.value, "test");
        assert_eq!(batch_data.row_number, 1);
        assert_eq!(batch_data.column_index, 2);
    }

    #[tokio::test]
    async fn test_batch_data_empty() {
        let batch_data = InferableValue {
            value: String::new(),
            row_number: 0,
            column_index: 0,
        };
        assert_eq!(batch_data.value, String::new());
        assert_eq!(batch_data.row_number, 0);
        assert_eq!(batch_data.column_index, 0);
    }
}
