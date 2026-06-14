
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TransitionRule<State, LexemeKind> {
    pub pattern: &'static str,
    pub current_state: State,
    pub next_state: State,
    pub output: Option<LexemeKind>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LexerStateMachine<State, LexemeKind> {
    pub state: State,
    pub rules: Vec<TransitionRule<State, LexemeKind>>,
}