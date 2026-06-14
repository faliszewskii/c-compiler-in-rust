use std::fmt::{Debug};
use clap::ValueEnum;

#[derive(Debug, Clone, ValueEnum, PartialEq)]
pub enum CompilationStage {
    #[value(name = "tp1")]
    TranslationPhase1,
    #[value(name = "tp2")]
    TranslationPhase2,
    #[value(name = "tp3")]
    TranslationPhase3,
}