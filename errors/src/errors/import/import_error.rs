use crate::create_errors;

use std::{
    error::Error as ErrorArg,
    fmt::{Debug, Display},
};

create_errors!(
    /// ImportError enum that represents all the errors for the `kari-import` crate.
    ImportError,
    exit_code_mask: 4000i32,
    error_code_prefix: "IMP",
);