use crate::create_errors;

use std::{
    error::Error as ErrorArg,
    fmt::{Debug, Display},
};

create_errors!(
    /// StateError enum that represents all the errors for the `kari-state` crate.
    StateError,
    exit_code_mask: 1000i32,
    error_code_prefix: "STA",
);