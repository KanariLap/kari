use crate::create_messages;
use std::{
    error::Error as ErrorArg,
    fmt::{Debug, Display},
};

create_messages!(
    /// CliError enum that represents all the errors for the `kari-lang` crate.
    CliError,
    code_mask: 7000i32,
    code_prefix: "CLI",

    /// For when the CLI experiences an IO error.
    @backtraced
    cli_io_error {
        args: (error: impl ErrorArg),
        msg: format!("cli io error {error}"),
        help: None,
    }

    /// For when the CLI could not fetch the versions.
    @backtraced
    could_not_fetch_versions {
        args: (error: impl ErrorArg),
        msg: format!("Could not fetch versions: {error}"),
        help: None,
    }

    /// For when the CLI fails to enable ansi support.
    @backtraced
    failed_to_enable_ansi_support {
        args: (),
        msg: "failed to enable ansi_support",
        help: None,
    }

    /// For when the CLI fails to self update.
    @backtraced
    self_update_error {
        args: (error: impl ErrorArg),
        msg: format!("self update crate Error: {error}"),
        help: None,
    }

    /// For when the CLI fails to self update.
    @backtraced
    self_update_build_error {
        args: (error: impl ErrorArg),
        msg: format!("self update crate failed to build Error: {error}"),
        help: None,
    }

    /// For when the CLI has an old release version.
    @backtraced
    old_release_version {
        args: (current: impl Display, latest: impl Display),
        msg: format!("Old release version {current} {latest}"),
        help: None,
    }

    @backtraced
    failed_to_load_instructions {
        args: (error: impl Display),
        msg: format!("Failed to load compiled Kanari instructions into Kanari file.\nSnarkVM Error: {error}"),
        help: Some("Generated Kanari instructions have been left in `main.kanari`".to_string()),
    }

    @backtraced
    needs_leo_build {
        args: (),
        msg: "You must run leo build before deploying a program.".to_string(),
        help: None,
    }

    @backtraced
    failed_to_execute_kanari_build {
        args: (error: impl Display),
        msg: format!("Failed to execute the `kanari build` command.\nSnarkVM Error: {error}"),
        help: None,
    }

    @backtraced
    failed_to_execute_kanari_new {
        args: (error: impl Display),
        msg: format!("Failed to execute the `kanari new` command.\nSnarkVM Error: {error}"),
        help: None,
    }

    @backtraced
    failed_to_execute_kanari_run {
        args: (error: impl Display),
        msg: format!("Failed to execute the `kanari run` command.\nSnarkVM Error: {error}"),
        help: None,
    }

    @backtraced
    failed_to_execute_kanari_node {
        args: (error: impl Display),
        msg: format!("Failed to execute the `kanari node` command.\nSnarkVM Error: {error}"),
        help: None,
    }

    @backtraced
    failed_to_execute_kanari_deploy {
        args: (error: impl Display),
        msg: format!("Failed to execute the `kanari deploy` command.\nSnarkVM Error: {error}"),
        help: None,
    }

    @backtraced
    failed_to_parse_kanari_new {
        args: (error: impl Display),
        msg: format!("Failed to parse the `kanari new` command.\nSnarkVM Error: {error}"),
        help: None,
    }

    @backtraced
    failed_to_parse_kanari_run {
        args: (error: impl Display),
        msg: format!("Failed to parse the `kanari run` command.\nSnarkVM Error: {error}"),
        help: None,
    }

    @backtraced
    failed_to_parse_kanari_node {
        args: (error: impl Display),
        msg: format!("Failed to parse the `kanari node` command.\nSnarkVM Error: {error}"),
        help: None,
    }

    @backtraced
    failed_to_parse_kanari_deploy {
        args: (error: impl Display),
        msg: format!("Failed to parse the `kanari deploy` command.\nSnarkVM Error: {error}"),
        help: None,
    }
);