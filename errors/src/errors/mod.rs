/// Contains the ASG error definitions.
use crate::KariMessageCode;

/// Contains the AST error definitions.
pub mod ast;
pub use self::ast::*;

/// Contains the AST error definitions.
pub mod cli;
pub use self::cli::*;

/// Contains the AST error definitions.
pub mod compiler;
pub use self::compiler::*;

/// Contains the Flattener error definitions.
pub mod flattener;
pub use self::flattener::*;

/// Contains the Input error definitions.
pub mod input;
pub use self::input::*;

/// Contains the Package error definitions.
pub mod package;
pub use self::package::*;

/// Contains the Parser error definitions.
pub mod parser;
pub use self::parser::*;

/// Contains the Type Checker error definitions.
pub mod type_checker;
pub use self::type_checker::*;

/// The KariError type that contains all sub error types.
/// This allows a unified error type throughout the Kari crates.
#[derive(Debug, Error)]
pub enum KariError {
    /// Represents an AST Error in a Kari Error.
    #[error(transparent)]
    AstError(#[from] AstError),
    /// Represents an CLI Error in a Kari Error.
    #[error(transparent)]
    CliError(#[from] CliError),
    /// Represents an Compiler Error in a Kari Error.
    #[error(transparent)]
    CompilerError(#[from] CompilerError),
    /// Represents an Input Error in a Kari Error.
    #[error(transparent)]
    InputError(#[from] InputError),
    /// Represents an Package Error in a Kari Error.
    #[error(transparent)]
    PackageError(#[from] PackageError),
    /// Represents an Parser Error in a Kari Error.
    #[error(transparent)]
    ParserError(#[from] ParserError),
    /// Represents a Type Checker Error in a Kari Error.
    #[error(transparent)]
    TypeCheckerError(#[from] TypeCheckerError),
    /// Represents a Flatten Error in a Kari Error.
    #[error(transparent)]
    FlattenError(#[from] FlattenError),
    /// Purely for just exiting with the correct status code and
    /// not re-displaying an error.
    #[error("")]
    LastErrorCode(i32),
    /// Anyhow errors.
    #[error(transparent)]
    Anyhow(#[from] anyhow::Error),
}

impl KariError {
    /// Implement error code for each type of Error.
    pub fn error_code(&self) -> String {
        use KariError::*;

        match self {
            AstError(error) => error.error_code(),
            CompilerError(error) => error.error_code(),
            CliError(error) => error.error_code(),
            InputError(error) => error.error_code(),
            ParserError(error) => error.error_code(),
            PackageError(error) => error.error_code(),
            TypeCheckerError(error) => error.error_code(),
            FlattenError(error) => error.error_code(),
            LastErrorCode(_) => unreachable!(),
            Anyhow(_) => unimplemented!(), // todo: implement error codes for snarkvm errors.
        }
    }

    /// Implement exit code for each type of Error.
    pub fn exit_code(&self) -> i32 {
        use KariError::*;

        match self {
            AstError(error) => error.exit_code(),
            CompilerError(error) => error.exit_code(),
            CliError(error) => error.exit_code(),
            InputError(error) => error.exit_code(),
            ParserError(error) => error.exit_code(),
            PackageError(error) => error.exit_code(),
            TypeCheckerError(error) => error.exit_code(),
            FlattenError(error) => error.exit_code(),
            LastErrorCode(code) => *code,
            Anyhow(_) => unimplemented!(), // todo: implement exit codes for snarkvm errors.
        }
    }
}

/// The KariWarning type that contains all sub error types.
/// This allows a unified error type throughout the Kari crates.
#[derive(Debug, Error)]
pub enum KariWarning {
    /// Represents an Parser Error in a Kari Error.
    #[error(transparent)]
    ParserWarning(#[from] ParserWarning),
}

impl KariWarning {
    /// Implement warning code for each type of Warning.
    pub fn error_code(&self) -> String {
        use KariWarning::*;

        match self {
            ParserWarning(warning) => warning.warning_code(),
        }
    }
}

/// A global result type for all Kari crates, that defaults the errors to be a KariError.
pub type Result<T, E = KariError> = core::result::Result<T, E>;