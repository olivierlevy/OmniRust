use omnirust::core::logging::{demo_logging, init_logger};

#[test]
fn test_logging() {
    init_logger("info");
    demo_logging(); // Verify logs in console.
}
