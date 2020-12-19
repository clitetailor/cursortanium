use crate::test::capture_result::CaptureResult;
use crate::test::test::Test;

pub fn capture(input: &str) -> CaptureResult {
    Test::new().capture(&input)
}
