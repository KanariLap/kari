use crate::create_messages;
use std::fmt::{Debug, Display};

create_messages!(
    /// InputError enum that represents all the errors for the inputs part of `kari-ast` crate.
    InputError,
    code_mask: 1000i32,
    code_prefix: "INP",

    /// For when declared variable type mismatches actual type.
    @formatted
    unexpected_type {
        args: (expected: impl Display, received: impl Display),
        msg: format!(
            "unexpected type, expected: '{expected}', received: '{received}'",
        ),
        help: None,
    }

    /// For when the expression is not allowed in an input file.
    @formatted
    illegal_expression {
        args: (expr: impl Display),
        msg: format!("expression '{expr}' is not allowed in inputs"),
        help: None,
    }

    /// For when section name is not an allowed one.
    @formatted
    unexpected_section {
        args: (expected: &[impl Display], received: impl Display),
        msg: format!(
            "unexpected section: expected {} -- got '{received}'",
            expected
                .iter()
                .map(|x| format!("'{x}'"))
                .collect::<Vec<_>>()
                .join(", ")
        ),
        help: None,
    }
);