use crate::create_messages;

use std::{
    error::Error as ErrorArg,
    fmt::{Debug, Display},
};

create_messages!(
    /// CompilerError enum that represents all the errors for the `kari-compiler` crate.
    CompilerError,
    code_mask: 6000i32,
    code_prefix: "CMP",

    /// For when the compiler can't read a file from the provided path.
    @backtraced
    file_read_error {
        args: (path: impl Debug, error: impl ErrorArg),
        msg: format!("Cannot read from the provided file path '{path:?}': {error}"),
        help: None,
    }

    /// For when a user tries to assign to a struct static member.
    @formatted
    illegal_static_member_assignment {
        args: (member: impl Display),
        msg: format!("Tried to assign to static member `{member}`"),
        help: None,
    }

    @formatted
    import_not_found {
        args: (file_path: impl Display),
        msg: format!("Attempted to import a file that does not exist `{file_path}`."),
        help: None,
    }

    @formatted
    cannot_open_cwd {
        args: (err: impl ErrorArg),
        msg: format!("Failed to open current working directory. Error: {err}"),
        help: None,
    }

    @formatted
    program_name_should_match_file_name {
        args: (program_name: impl Display, file_name: impl Display),
        msg: format!("Program name `{program_name}` should match file name `{file_name}`"),
        help: None,
    }

    @formatted
    program_scope_name_does_not_match {
        args: (program_scope_name: impl Display, file_name: impl Display),
        msg: format!("The program scope name `{program_scope_name}` must match `{file_name}`."),
        help: None,
    }
);