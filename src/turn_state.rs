#[derive(Copy, PartialEq, Clone, Debug)]
pub enum TurnState {
    AwaitingInput,
    PlayerTurn,
    EnemyTurn,
}
