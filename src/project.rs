use std::path::{Path, PathBuf};

/// Describes a discovered cleanable project
#[derive(Debug)]
pub struct Project {
	/// The root directory of the project
	root : PathBuf,

	/// Directories containing dependencies
	dependency_dirs : Vec<PathBuf>,

	/// Timestamp indicating when the project was last modified
	last_modified : u64,
}

impl Project {
	/// Initialises a new project
	///
	/// # Arguments
	/// `root` - The root directory of the project
	pub fn new<P : Into<PathBuf>>(root : P) -> Project {
		Project {
			root: root.into(),
			dependency_dirs: Vec::new(),
			last_modified: 0,
		}
	}

	pub fn root(&self) -> &Path { &self.root }

	/// Marks a subdirectory of this project's root directory as a dependency,
	/// if that directory exists. If the subdirectory doesn't exist, nothing
	/// happens.
	///
	/// # Arguments
	/// `subdir` - Name of the subdirectory inside the project root directory
	pub fn try_add_dependency_dir<P : Into<PathBuf>>(&mut self, subdir : P) {
		let mut path = self.root.clone();
		path.push(subdir.into());

		if path.exists() && path.is_dir() && !self.dependency_dirs.contains(&path) {
			self.dependency_dirs.push(path);
		}
	}

	/// Checks if the given path is listed as a dependency directory of this
	/// project
	pub fn is_dependency_dir<P : Into<PathBuf>>(&self, path : P) -> bool {
		return self.dependency_dirs.contains(&path.into());
	}

	/// Consumes the project and returns the dependency directories
	pub fn into_dependency_dirs(self) -> Vec<PathBuf> {
		self.dependency_dirs
	}
}
