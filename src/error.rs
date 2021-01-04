use quick_error::quick_error;

use crate::component::position::Position;

pub type SokobanResult<T, E = SokobanError> = std::result::Result<T, E>;

quick_error! {
    #[derive(Debug)]
    pub enum SokobanError {
        OutOfBounds(position: Position) {
            display("Move would place an entity out of bounds, error {:?}", position)
        }
    }
}
