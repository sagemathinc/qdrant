
use crate::types::{PointOffsetType, PayloadKeyType, PayloadType, Filter, TheMap};
use crate::entry::entry_point::OperationResult;


/// Trait for payload data storage. Should allow filter checks
pub trait PayloadStorage {

    /// Assign same payload to each given point
    fn assign_all(&mut self, point_id: PointOffsetType, payload: TheMap<PayloadKeyType, PayloadType>) -> OperationResult<()> {
        self.drop(point_id)?;
        for (key, value) in payload {
            self.assign(point_id, &key, value)?;
        }
        Ok(())
    }

    fn assign(&mut self, point_id: PointOffsetType, key: &PayloadKeyType, payload: PayloadType) -> OperationResult<()>;

    /// Get payload for point
    fn payload(&self, point_id: PointOffsetType) -> TheMap<PayloadKeyType, PayloadType>;

    /// Delete payload by key
    fn delete(&mut self, point_id: PointOffsetType, key: &PayloadKeyType) -> OperationResult<Option<PayloadType>>;

    /// Drop all payload of the point
    fn drop(&mut self, point_id: PointOffsetType) -> OperationResult<Option<TheMap<PayloadKeyType, PayloadType>>>;

    /// Completely drop payload. Pufff!
    fn wipe(&mut self) -> OperationResult<()>;

    /// Force persistence of current storage state.
    fn flush(&self) -> OperationResult<usize>;
}


pub trait ConditionChecker {
    /// Check if point satisfies filter condition
    fn check(&self, point_id: PointOffsetType, query: &Filter) -> bool;
}
