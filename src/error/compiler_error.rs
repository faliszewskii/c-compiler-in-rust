use std::env;

#[derive(thiserror::Error, Debug)]
pub enum CompilerError {
    #[error("{0}")]
    UserError(String),
    #[error(transparent)]
    Internal(anyhow::Error),
}

impl CompilerError {
    pub fn report_and_exit(self) -> ! {
        let program_path = env::current_exe().expect("Couldn't retrieve executable name");
        let program_name = program_path.file_name().expect("No file name");
        eprintln!("{}: {self}", program_name.display());
        std::process::exit(1);
    }
}

impl From<anyhow::Error> for CompilerError {
    fn from(value: anyhow::Error) -> Self {
        CompilerError::Internal(value)
    }
}
