/// Collapse consecutive `/` and `\` separators in a stored filesystem path.
///
/// Leading UNC/network prefixes (`//` or `\\`) are preserved. Interior doubled
/// separators such as `/org//project` become `/org/project`. URI-looking
/// strings (`://`) are left unchanged so `file:` paths are not rewritten.
pub fn collapse_redundant_path_separators(path: &str) -> String {
    if path.is_empty() || path.contains("://") {
        return path.to_string();
    }

    let bytes = path.as_bytes();
    let (prefix, rest) = if bytes.len() >= 2
        && matches!(bytes[0], b'/' | b'\\')
        && matches!(bytes[1], b'/' | b'\\')
        && bytes
            .get(2)
            .is_none_or(|byte| !matches!(byte, b'/' | b'\\'))
    {
        (&path[..2], &path[2..])
    } else {
        ("", path)
    };

    let mut collapsed = String::with_capacity(path.len());
    collapsed.push_str(prefix);
    let mut last_was_separator = !prefix.is_empty();
    for ch in rest.chars() {
        let is_separator = matches!(ch, '/' | '\\');
        if is_separator && last_was_separator {
            continue;
        }
        last_was_separator = is_separator;
        collapsed.push(ch);
    }
    collapsed
}

#[cfg(test)]
#[path = "path_separators_tests.rs"]
mod tests;
