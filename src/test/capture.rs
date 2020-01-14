use crate::test::capture_result::CaptureResult;
use crate::test::test::Test;

use std::borrow::Cow;

pub fn capture<'a, T: Into<Cow<'a, str>>>(
    input: T,
) -> CaptureResult<'a> {
    Test::new().capture(input)
}
