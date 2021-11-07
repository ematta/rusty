use crate::prelude::*;

#[system]
pub fn end_turn(#[resource] turn_state: &mut TurnState) {
    let new_state = match turn_state {
        TurnState::AwaitInput => return,
        TurnState::MonsterTurn => TurnState::AwaitInput,
        TurnState::PlayerTurn => TurnState::MonsterTurn,
    };
    *turn_state = new_state;
}
