use super::collapse_redundant_path_separators;
use pretty_assertions::assert_eq;

#[test]
fn collapses_interior_posix_and_windows_separators() {
    assert_eq!(
        collapse_redundant_path_separators("/Users/me/org//some-project"),
        "/Users/me/org/some-project"
    );
    assert_eq!(
        collapse_redundant_path_separators(r"C:\Users\me\org\\some-project"),
        r"C:\Users\me\org\some-project"
    );
    assert_eq!(
        collapse_redundant_path_separators("/Users/me/org///some-project"),
        "/Users/me/org/some-project"
    );
    assert_eq!(
        collapse_redundant_path_separators(r"/Users/me/org\/some-project"),
        "/Users/me/org/some-project"
    );
}

#[test]
fn preserves_unc_prefix_and_collapses_the_remainder() {
    assert_eq!(
        collapse_redundant_path_separators(r"\\server\share\\folder"),
        r"\\server\share\folder"
    );
    assert_eq!(
        collapse_redundant_path_separators("//server/share//folder"),
        "//server/share/folder"
    );
}

#[test]
fn leaves_already_normalized_and_uri_paths_unchanged() {
    assert_eq!(
        collapse_redundant_path_separators("/Users/me/org/some-project"),
        "/Users/me/org/some-project"
    );
    assert_eq!(
        collapse_redundant_path_separators("file:///Users/me/org//some-project"),
        "file:///Users/me/org//some-project"
    );
    assert_eq!(collapse_redundant_path_separators(""), "");
}
