use regex::Regex;
use std::sync::LazyLock;
use crate::utils::errors;

static REGEX: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r"(\d+): (\d+( \d+)+)")?
});
