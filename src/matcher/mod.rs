use ignore::gitignore::{Gitignore, GitignoreBuilder};
use std::path::Path;

#[derive(Debug)]
pub struct Matcher {
    matcher: Gitignore,
}

impl Matcher {
    pub fn new(base_dir: &Path, ignore_file: String) -> Self {
        let mut builder = GitignoreBuilder::new(base_dir);

        let ignore_file = base_dir.join(ignore_file);
        if ignore_file.exists() {
            builder.add(ignore_file);
        }

        let matcher = builder
            .build()
            .expect("Failed to create ignore matcher builder");
        Self { matcher }
    }

    pub fn is_matched(&self, file: &Path) -> bool {
        self.matcher
            .matched_path_or_any_parents(file, file.is_dir())
            .is_ignore()
    }
}
