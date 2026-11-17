// Integration test for SigmaOS-multifunction-printer-support
// Verifies the crate builds and links as a dependency.
#![allow(unused_imports)]

use multifunction_printer_support as _;

#[test]
fn crate_links_and_runs() {
    assert_eq!(2 + 2, 4);
}
