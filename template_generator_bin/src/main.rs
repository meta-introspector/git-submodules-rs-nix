use std::fs;
use std::io::Write;
use std::path::Path;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let generated_contents = wikimedia_template_introspector_core::generate_wikiproject_crates_content();

    let base_path = Path::new("/data/data/com.termux.nix/files/home/pick-up-nix/source/github/meta-introspector/submodules/generated_wikiproject_crates");

    for (wikiproject_name, content) in generated_contents {
        let crate_dir = base_path.join(&wikiproject_name);
        let src_dir = crate_dir.join("src");
        let lib_rs_path = src_dir.join("lib.rs");

        fs::create_dir_all(&src_dir)?;
        fs::write(&lib_rs_path, content)?;
        println!("Generated {}/src/lib.rs", wikiproject_name);
    }

    Ok(())
}
