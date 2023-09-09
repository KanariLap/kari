use crate::create_messages;

use std::fmt::Display;

create_messages!(
    /// ParserWarning enum that represents all the warnings for the `kari-parser` crate.
    ParserWarning,
    code_mask: 0000i32,
    code_prefix: "PAR",

    /// For when a user used const on a parameter or input instead of constant.
    @formatted
    const_parameter_or_input {
        args: (),
        msg: "`constant` is preferred over `const` for function parameters to indicate a R1CS constant.",
        help: None,
    }

    /// For when a keyword is deprecated but could be used as a valid identifier.
    @formatted
    deprecated {
        args: (keyword: impl Display, help: impl Display),
        msg: format!("The keyword `{keyword}` is deprecated."),
        help: Some(help.to_string()),
    }



);