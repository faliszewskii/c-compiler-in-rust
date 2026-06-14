
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TransitionRule<State, LexemeKind: 'static> {
    pub pattern: &'static str,
    pub current_state: State,
    pub next_state: State,
    pub outputs: &'static [LexemeKind],
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LexerStateMachine<State, LexemeKind: 'static> {
    pub state: State,
    pub rules: Vec<TransitionRule<State, LexemeKind>>,
}
