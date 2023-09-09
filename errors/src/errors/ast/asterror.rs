use crate::create_messages;
use std::{
    error::Error as ErrorArg,
    fmt::{Debug, Display},
};

create_messages!(
    /// AstError enum that represents all the errors for the `kari-ast` crate.
    AstError,
    code_mask: 2000i32,
    code_prefix: "AST",

    /// For when the AST fails to be represented as a JSON string.
    @backtraced
    failed_to_convert_ast_to_json_string {
        args: (error: impl ErrorArg),
        msg: format!("failed to convert ast to a json string {error}"),
        help: None,
    }

    /// For when the AST fails to create the AST JSON file.
    @backtraced
    failed_to_create_ast_json_file {
        args: (path: impl Debug, error: impl ErrorArg),
        msg: format!("failed to create ast json file `{path:?}` {error}"),
        help: None,
    }

    /// For when the AST fails to write the AST JSON file.
    @backtraced
    failed_to_write_ast_to_json_file {
        args: (path: impl Debug, error: impl ErrorArg),
        msg: format!("failed to write ast to a json file `{path:?}` {error}"),
        help: None,
    }

    /// For when the a JSON string fails to be represented as an AST.
    @backtraced
    failed_to_read_json_string_to_ast {
        args: (error: impl ErrorArg),
        msg: format!("failed to convert json string to an ast {error}"),
        help: None,
    }

    /// For when the a JSON files fails to be represented as an AST.
    @backtraced
    failed_to_read_json_file {
        args: (path: impl Debug, error: impl ErrorArg),
        msg: format!("failed to convert json file `{path:?}` to an ast {error}"),
        help: None,
    }

    /// For when the AST fails to be represented as a JSON value.
    @backtraced
    failed_to_convert_ast_to_json_value {
        args: (error: impl ErrorArg),
        msg: format!("failed to convert ast to a json value {error}"),
        help: None,
    }

    /// For when a user shadows a function.
    @formatted
    shadowed_function {
        args: (func: impl Display),
        msg: format!("function `{func}` shadowed by"),
        help: None,
    }

    /// For when a user shadows a struct.
    @formatted
    shadowed_struct {
        args: (struct_: impl Display),
        msg: format!("struct `{struct_}` shadowed by"),
        help: None,
    }

    /// For when a user shadows a record.
    @formatted
    shadowed_record {
        args: (record: impl Display),
        msg: format!("record `{record}` shadowed by"),
        help: None,
    }

    /// For when a user shadows a variable.
    @formatted
    shadowed_variable {
        args: (var: impl Display),
        msg: format!("variable `{var}` shadowed by"),
        help: None,
    }
);