use std::fs;
use std::path::{Path, PathBuf};

pub(crate) fn lib_exists(paths: &[PathBuf], name: &str) -> bool {
    let shared = format!("lib{}.so", name);
    let static_lib = format!("lib{}.a", name);

    paths.iter().any(|path| {
        let shared_path = path.join(&shared);
        let static_path = path.join(&static_lib);
        shared_path.exists() || static_path.exists() || has_versioned_shared(path, name)
    })
}

fn has_versioned_shared(path: &Path, name: &str) -> bool {
    let prefix = format!("lib{}.so.", name);
    let entries = match fs::read_dir(path) {
        Ok(entries) => entries,
        Err(_) => return false,
    };

    entries.filter_map(Result::ok).any(|entry| {
        let file_name = entry.file_name();
        file_name.to_string_lossy().starts_with(&prefix)
    })
}

pub(crate) fn find_openmp_runtime(paths: &[PathBuf]) -> Option<&'static str> {
    if lib_exists(paths, "gomp") {
        Some("gomp")
    } else if lib_exists(paths, "omp") {
        Some("omp")
    } else {
        None
    }
}
