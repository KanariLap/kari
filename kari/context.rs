use crate::commands::Network;
use kari_errors::{CliError, PackageError, Result};
use snarkvm::file::Manifest;

use kari_package::build::{BuildDirectory, BUILD_DIRECTORY_NAME};
use std::{
    env::current_dir,
    fs::File,
    io::Write,
    path::{Path, PathBuf},
};

/// Project context, manifest, current directory etc
/// All the info that is relevant in most of the commands
#[derive(Clone)]
pub struct Context {
    /// Path at which the command is called, None when default
    pub path: Option<PathBuf>,
}

impl Context {
    pub fn new(path: Option<PathBuf>) -> Result<Context> {
        Ok(Context { path })
    }

    /// Returns the path to the Kari package.
    pub fn dir(&self) -> Result<PathBuf> {
        match &self.path {
            Some(path) => Ok(path.clone()),
            None => Ok(current_dir().map_err(CliError::cli_io_error)?),
        }
    }

    /// Returns the package name as a String.
    /// Opens the manifest file `program.json` and creates the build directory if it doesn't exist.
    pub fn open_manifest(&self) -> Result<Manifest<Network>> {
        // Open the manifest file.
        let path = self.dir()?;
        let manifest = Manifest::<Network>::open(&path).map_err(PackageError::failed_to_open_manifest)?;

        // Lookup the program id.
        // let program_id = manifest.program_id();

        // Create the Kari build/ directory if it doesn't exist.
        let build_path = path.join(Path::new(BUILD_DIRECTORY_NAME));
        if !build_path.exists() {
            BuildDirectory::create(&build_path)?;
        }

        // Mirror the program.json file in the Kari build/ directory for Kanari SDK compilation.

        // Read the manifest file to string.
        let manifest_string =
            std::fs::read_to_string(manifest.path()).map_err(PackageError::failed_to_open_manifest)?;

        // Construct the file path.
        let build_manifest_path = build_path.join(Manifest::<Network>::file_name());

        // Write the file.
        File::create(build_manifest_path)
            .map_err(PackageError::failed_to_open_manifest)?
            .write_all(manifest_string.as_bytes())
            .map_err(PackageError::failed_to_open_manifest)?;

        // Get package name from program id.
        Ok(manifest)
    }
}