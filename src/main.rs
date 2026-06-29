use excel_to_json::converter::ErrorEntry;
use excel_to_json::output::{self, Summary};

/// Binary entry point for excel-to-json.
///
/// Delegates all logic to `excel_to_json::run()` so integration tests
/// can exercise the full pipeline without subprocessing the binary.
fn main() {
    match excel_to_json::run() {
        Ok(summary) => {
            // Summary was already emitted by run() on the success path.
            if summary.status == "error" {
                std::process::exit(1);
            }
        }
        Err(e) => {
            // Global error — run() could not emit a summary.
            // Emit an error summary here (without errors/warnings if empty).
            output::emit_summary(&Summary {
                status: "error".to_string(),
                files: vec![],
                errors: vec![ErrorEntry {
                    file: None,
                    message: format!("{:#}", e),
                }],
                warnings: vec![],
            });
            eprintln!("Error: {e:#}");
            std::process::exit(1);
        }
    }
}
