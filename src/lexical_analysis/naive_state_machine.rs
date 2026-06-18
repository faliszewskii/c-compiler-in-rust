#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TransitionRule<LexemeKind: 'static> {
    pub pattern: &'static str,
    pub outputs: &'static [LexemeKind],
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LexerStateMachine<LexemeKind: 'static> {
    pub rules: Vec<TransitionRule<LexemeKind>>,
}
