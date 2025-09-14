use std::fs;
use std::io::{self, Write};
use tectonic;

fn main() -> io::Result<()> {
    let latex_input_path = "../../abstract_mathematical_idea.tex"; // Relative path from tools/latex_compiler_tool/src/
    let output_pdf_path = "../../abstract_mathematical_idea.pdf"; // Relative path from tools/latex_compiler_tool/src/

    println!("Reading LaTeX input from: {}", latex_input_path);
    let latex_input = fs::read_to_string(latex_input_path)?;

    println!("Compiling LaTeX to PDF...");
    let pdf_data = tectonic::latex_to_pdf(latex_input.as_bytes())
        .map_err(|e| io::Error::new(io::ErrorKind::Other, format!("Tectonic compilation failed: {}", e)))?;

    println!("Saving PDF to: {}", output_pdf_path);
    let mut file = fs::File::create(output_pdf_path)?;
    file.write_all(&pdf_data)?;

    println!("PDF generated successfully!");
    Ok(())
}