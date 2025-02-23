#[inline(always)]
pub fn get_string_width(text: &str) -> usize {
    text.lines().last().unwrap_or("").len()
}
