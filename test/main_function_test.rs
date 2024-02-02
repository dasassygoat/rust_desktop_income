// Imports the function under test
use crate::main::main;

// Test that main() runs without panicking
#[test]
fn test_main() {
    main();
}

// Test that main() initializes the MainWindow
#[test]
fn test_main_initializes_main_window() {
    main();

    // Could assert on MainWindow state here
}

// Test that main() shuffles the tiles
#[test]
fn test_main_shuffles_tiles() {
    let original_tiles = // generate test tiles

        main();

    let shuffled_tiles = // get shuffled tiles

        assert_ne!(original_tiles, shuffled_tiles);
}

// Limitations:
// - Can't easily test internal logic and state
// - Would need to expose more functions and state
