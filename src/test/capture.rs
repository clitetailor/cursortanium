use std::borrow::Cow;

use crate::test::capture_result::CaptureResult;
use crate::test::test::Test;

pub fn capture<'a, T: Into<Cow<'a, str>>>(
    input: T,
) -> CaptureResult<'a> {
    Test::new().capture(input)
}
