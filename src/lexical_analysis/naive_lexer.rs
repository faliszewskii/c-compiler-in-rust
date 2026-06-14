use crate::error::CompilerError;
use crate::error::CompilerError::UserError;
use crate::lexical_analysis::lexeme::Lexeme;
use crate::lexical_analysis::naive_state_machine::LexerStateMachine;
use anyhow::{anyhow, Context};
use regex::{Captures, Match, Regex};

pub fn lex<State: std::cmp::PartialEq + std::clone::Clone, LexemeKind: std::clone::Clone>(
    source: &str,
    state_machine: &mut LexerStateMachine<State, LexemeKind>,
) -> Result<Vec<Lexeme<LexemeKind>>, CompilerError> {
    let mut lexemes = Vec::new();
    let mut slice = source;
    while !slice.is_empty() {
        let mut matched_rules = Vec::new();
        for rule in &state_machine.rules {
            if rule.current_state == state_machine.state {
                let anchored = r"\A".to_string() + rule.pattern;
                let re =
                    Regex::new(&anchored).context("Failed to compile anchored lexer regex!")?;
                if let Some(matched) = re.captures(slice) {
                    if get_implicit_capture_group(&matched).is_empty() {
                        Err(anyhow!("Lexer transition rule matched an empty string!"))?;
                    }
                    if matched.len() - 1 != rule.outputs.len() {
                        Err(anyhow!(
                            "Lexer transition rule should have the same amount of captures as outputs!"
                        ))?;
                    }
                    matched_rules.push((matched, rule));
                }
            }
        }
        let (longest_match, selected_rule) = matched_rules
            .iter()
            .max_by_key(|(matched, _)| get_implicit_capture_group(matched).len())
            .ok_or_else(|| UserError("Unexpected character during lexing".to_owned()))?;
        state_machine.state = selected_rule.next_state.clone();
        slice = &slice[get_implicit_capture_group(longest_match).len()..];
        for (i, output) in selected_rule.outputs.iter().enumerate() {
            lexemes.push(Lexeme {
                kind: output.clone(),
                text: longest_match[i + 1].to_string(),
            });
        }
    }
    Ok(lexemes)
}

fn get_implicit_capture_group<'a>(matched: &Captures<'a>) -> Match<'a> {
    matched
        .get(0)
        .expect("If a match is found, first group is always present")
}
