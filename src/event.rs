#[derive(Debug)]
pub enum GameEvent {
    PlayerHitObstacle,
    EntityMoved { entity_id: u32 },
    CrateMovedToSpot { is_correct_spot: bool },
}
