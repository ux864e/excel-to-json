/// Binary entry point for excel-to-json.
///
/// Delegates all logic to `excel_to_json::run()` so integration tests
/// can exercise the full pipeline without subprocessing the binary.
fn main() {
    if let Err(e) = excel_to_json::run() {
        eprintln!("Error: {e:#}");
        std::process::exit(1);
    }
}
