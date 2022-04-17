#[macro_use]
extern crate version_sync;

#[test]
fn test_readme_version() {
    assert_markdown_deps_updated!("README.md");
}
