// Integration test example
use filrust;

#[test]
fn get_earnings_test_mode() {
    assert_eq!(filrust::earnings(), 6);
}

