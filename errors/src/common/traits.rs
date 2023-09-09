/// MessageCode trait that all Errors should implement.
pub trait KariMessageCode: Sized {
    /// Returns the error's exit code for the program.
    fn exit_code(&self) -> i32;

    /// Returns the prefixed error identifier.
    fn error_code(&self) -> String;

    /// Returns the prefixed warning identifier.
    fn warning_code(&self) -> String;

    /// Returns the messages's exit code mask, as to avoid conflicts.
    fn code_mask() -> i32;

    /// Returns the message's code type for the program.
    fn message_type() -> String;

    /// Returns if the message is an error or warning.
    fn is_error() -> bool;

    /// The SuErrorCode which has a default code identifier of 037
    /// (Kari upsidedown and backwards). This is to make the exit codes
    /// unique to Su itself.
    #[inline(always)]
    fn code_identifier() -> i8 {
        37
    }
}