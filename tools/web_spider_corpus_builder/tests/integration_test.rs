use std::fs;
use std::path::PathBuf;
use std::process::Command;

#[test]
fn test_corpus_creation_and_html_fetch_from_markdown() {
    // Define a temporary output directory for testing
    let test_output_dir = PathBuf::from("test_corpus/web_sources");
    let test_md_file = PathBuf::from("test_data/test_urls.md");

    // Clean up previous test run artifacts
    if test_output_dir.exists() {
        fs::remove_dir_all(&test_output_dir).unwrap();
    }
    if test_md_file.parent().unwrap().exists() {
        fs::remove_dir_all(test_md_file.parent().unwrap()).unwrap();
    }

    // Create dummy markdown file with a known URL
    fs::create_dir_all(test_md_file.parent().unwrap()).unwrap();
    fs::write(&test_md_file, "[Example Link](http://example.com)").unwrap();

    // Build the spider executable
    let build_output = Command::new("cargo")
        .arg("build")
        .arg("-p")
        .arg("web_spider_corpus_builder")
        .output()
        .expect("Failed to build web_spider_corpus_builder");
    assert!(build_output.status.success(), "Build failed: {:?}", build_output);

    // Run the spider executable with the dummy markdown file
    let run_output = Command::new("target/debug/web_spider_corpus_builder")
        .arg("-m")
        .arg(&test_md_file)
        .output()
        .expect("Failed to run web_spider_corpus_builder");
    assert!(run_output.status.success(), "Spider run failed: {:?}", run_output);

    // Assertions
    assert!(test_output_dir.exists());

    // Check for the fetched file (filename derived from URL)
    let expected_file_name = sanitize_filename::sanitize("example.com_index.html") + ".txt";
    let expected_file_path = test_output_dir.join(expected_file_name);
    assert!(expected_file_path.exists());

    let content = fs::read_to_string(&expected_file_path).unwrap();
    assert!(content.contains("Example Domain"));
    assert!(content.contains("illustrative examples"));

    // Clean up after the test
    fs::remove_dir_all(&test_output_dir).unwrap();
    fs::remove_dir_all(test_md_file.parent().unwrap()).unwrap();
}