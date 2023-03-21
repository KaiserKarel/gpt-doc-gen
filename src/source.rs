use core::fmt::Debug;
use std::{
    collections::HashMap,
    fs::{self, DirEntry, File},
    io::Read,
    path::{Path, PathBuf},
};

/// A map of source files to their contents.
pub struct SourceMap {
    /// The map of source files to their contents.
    pub sources: HashMap<PathBuf, Source>,
}

pub struct Source {
    pub contents: String,
    pub prompt: &'static str,
}

impl Source {
    pub fn from_file(path: impl AsRef<Path>) -> color_eyre::Result<Self> {
        let path = path.as_ref();
        let mut file = File::open(path)?;
        let mut contents = String::new();
        file.read_to_string(&mut contents)?;

        let prompt = match path.extension().map(|ext| ext.to_str()) {
            Some(Some("rs")) => crate::promts::RS_PROMPT,
            _ => unimplemented!("non-rs files are not supported yet"),
        };
        Ok(Source { contents, prompt })
    }
}

impl Debug for SourceMap {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("SourceMap")
            .field("sources", &self.sources.keys().collect::<Vec<_>>())
            .finish()
    }
}

impl SourceMap {
    /// Creates a new [`SourceMap`] from the given root path.
    ///
    /// # Examples
    ///
    /// ```
    /// use std::path::Path;
    ///
    /// let map = SourceMap::from_root(Path::new("tests/fixtures")).unwrap();
    /// ```
    pub fn from_root(path: impl AsRef<Path>) -> color_eyre::Result<Self> {
        let mut map = SourceMap {
            sources: HashMap::new(),
        };

        let path = path.as_ref();

        visit_dirs(path, &mut |path, _dir| {
            if let Some(ext) = path.extension() {
                if ext == "rs" {
                    map.add_source(path)?;
                }
            }
            Ok(())
        })?;
        Ok(map)
    }

    /// Adds a source file to the map.
    ///
    /// # Errors
    ///
    /// This function will return an error if the file could not be read.
    fn add_source(&mut self, path: &Path) -> color_eyre::Result<()> {
        let source = Source::from_file(path)?;
        self.sources.insert(path.to_path_buf(), source);
        Ok(())
    }
}

/// Visits all directories in the given directory, calling the callback for each file.
///
/// # Errors
///
/// This function will return an error if the directory could not be read.
fn visit_dirs<F: FnMut(&Path, &DirEntry) -> color_eyre::Result<()>>(
    dir: &Path,
    cb: &mut F,
) -> color_eyre::Result<()> {
    if dir.is_dir() {
        for entry in fs::read_dir(dir)? {
            let entry = entry?;
            let path = entry.path();
            if path.is_dir() {
                visit_dirs(path.as_path(), cb)?;
            } else {
                cb(&path, &entry)?;
            }
        }
    }
    Ok(())
}
