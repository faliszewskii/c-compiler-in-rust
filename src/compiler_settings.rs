use crate::compilation_stage::CompilationStage;

pub(crate) struct CompilerSettings {
    pub until: Option<CompilationStage>,
}