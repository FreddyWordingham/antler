use std::{
    collections::HashSet,
    fs::read_to_string,
    path::{Path, PathBuf},
};

use crate::errors::IncludeError;

pub fn expand_includes(input: &str, base_dir: impl AsRef<Path>) -> Result<String, IncludeError> {
    let root = base_dir.as_ref().to_path_buf();
    let mut stack = HashSet::new();

    expand_includes_inner(input, &root, &root, &mut stack)
}

fn expand_includes_inner(
    input: &str,
    current_dir: &Path,
    root: &Path,
    stack: &mut HashSet<PathBuf>,
) -> Result<String, IncludeError> {
    let mut output = String::with_capacity(input.len());
    let mut i = 0;

    while i < input.len() {
        if is_line_comment_at(input, i) {
            let end = input[i..].find('\n').map_or(input.len(), |offset| i + offset + 1);

            output.push_str(&input[i..end]);
            i = end;
            continue;
        }

        if input[i..].starts_with("@include(") {
            let include_start = i;
            let path_start = i + "@include(".len();

            let close_offset = input[path_start..]
                .find(')')
                .ok_or(IncludeError::UnterminatedInclude(include_start))?;

            let path_end = path_start + close_offset;
            let raw_path = input[path_start..path_end].trim();

            if raw_path.is_empty() {
                return Err(IncludeError::EmptyPath(include_start));
            }

            let raw_path = raw_path.trim_matches('"');
            let include_path = current_dir.join(raw_path);
            let include_path = include_path.canonicalize().map_err(|source| IncludeError::Read {
                path: include_path.clone(),
                source,
            })?;

            let root = root.canonicalize().map_err(|source| IncludeError::Read {
                path: root.to_path_buf(),
                source,
            })?;

            if !include_path.starts_with(&root) {
                return Err(IncludeError::EscapesRoot(include_path));
            }

            if !stack.insert(include_path.clone()) {
                return Err(IncludeError::Cycle(include_path));
            }

            let included = read_to_string(&include_path).map_err(|source| IncludeError::Read {
                path: include_path.clone(),
                source,
            })?;

            let included_dir = include_path.parent().unwrap_or(current_dir);

            let expanded = expand_includes_inner(&included, included_dir, &root, stack)?;
            output.push_str(&expanded);

            stack.remove(&include_path);

            i = path_end + 1;
            continue;
        }

        let ch = input[i..].chars().next().unwrap();
        output.push(ch);
        i += ch.len_utf8();
    }

    Ok(output)
}

fn is_line_comment_at(input: &str, i: usize) -> bool {
    input[i..].starts_with("//")
}
