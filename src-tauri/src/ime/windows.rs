use super::ImeState;

pub fn current_state() -> ImeState {
    // TODO: Implement using ImmGetContext + ImmGetOpenStatus.
    ImeState::Unknown
}
