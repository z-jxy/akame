use std::path::{PathBuf, Path};

/// Find the first executable with the given name in the PATH. Used for validating dependencies.
/// clang is required to compile the LLVM IR to a binary. or we can also use gcc if it's available.
/// TODO: gcc will also require `llc` to be able to compile the LLVM IR to assembly. at this point, there should be clang anyway most likely
pub fn find_it<P>(exe_name: P) -> Option<PathBuf>
    where P: AsRef<Path>,
{
    std::env::var_os("PATH").and_then(|paths| {
        std::env::split_paths(&paths).filter_map(|dir| {
            let full_path = dir.join(&exe_name);
            if full_path.is_file() {
                Some(full_path)
            } else {
                None
            }
        }).next()
    })
}