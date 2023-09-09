use crate::create_messages;
use std::fmt::{Debug, Display};

/// Generates the type name of a value.
pub fn type_name<T>(_: &T) -> &'static str {
    std::any::type_name::<T>()
}

create_messages!(
    /// CliError enum that represents all the errors for the `kari-lang` crate.
    FlattenError,
    code_mask: 3000i32,
    code_prefix: "FLA",

    /// For when a constant operation would cause an overflow.
    @formatted
    binary_overflow {
        args: (left: impl Display, op: impl Display, right: impl Display, right_type: impl Display),
        msg: format!("The const operation `{left}{} {op} {right}{right_type}` causes an overflow.", type_name(&left)),
        help: None,
    }

    /// For when a constant operation would cause an overflow.
    @formatted
    unary_overflow {
        args: (left: impl Display, op: impl Display),
        msg: format!("The const operation `{left}{} {op}` causes an overflow.", type_name(&left)),
        help: None,
    }

    /// For when a loop uses a negative value.
    @formatted
    loop_has_neg_value {
        args: (value: impl Display),
        msg: format!(
            "The loop has a negative loop bound `{value}`.",
        ),
        help: None,
    }
);