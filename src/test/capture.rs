use std::rc::Rc;

use crate::test::capture_result::CaptureResult;
use crate::test::test::Test;

pub fn capture(input: &Rc<String>) -> CaptureResult {
    Test::new().capture(&input)
}
