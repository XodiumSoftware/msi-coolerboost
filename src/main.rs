//! Unified entry point for MSI `CoolerBoost`.
//!
//! A single `msi-coolerboost` binary supports both CLI and tray modes:
//!
//! - `msi-coolerboost` or `msi-coolerboost toggle` — toggle `CoolerBoost` and print the state.
//! - `msi-coolerboost tray` — run the system tray GUI.
//! - `msi-coolerboost --help` — show usage information.

use std::env;

/// Prints CLI usage information.
fn print_usage() {
    println!("Usage: msi-coolerboost [COMMAND]");
    println!();
    println!("Commands:");
    println!("  toggle    Toggle CoolerBoost and print the state (default)");
    println!("  tray      Run the system tray application");
}

fn main() {
    match env::args().nth(1).as_deref() {
        None | Some("toggle") => msi_coolerboost::toggle_mode::run_toggle(),
        Some("tray") => msi_coolerboost::tray_mode::run_tray(),
        Some("--help" | "-h") => print_usage(),
        Some(unknown) => {
            eprintln!("Unknown command: {unknown}");
            print_usage();
            std::process::exit(1);
        }
    }
}
