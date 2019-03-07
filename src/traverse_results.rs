use crate::operation_result::OperationResult;
use crate::page_index::PageIndex;

pub struct TraverseResults {
    pub page_index: Vec<PageIndex>,
    pub error_count: usize,
    pub errors: Vec<OperationResult>,
}

impl TraverseResults {
    pub fn new(page_index: Vec<PageIndex>, errors: Vec<OperationResult>) -> Self {
        Self {
            page_index,
            error_count: errors.len(),
            errors,
        }
    }
}
