#[derive(Copy, Clone, Debug, PartialEq)]
pub enum TurnState {
    AwaitInput,
    PlayerTurn,
    MonsterTurn,
}
