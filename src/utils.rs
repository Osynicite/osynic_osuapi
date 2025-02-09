// e.g. CN -> 1f1e8-1f1f3, for /assets/images/flags/1f1e8-1f1f3.svg
pub fn country_code_to_unicode_flag(code: &str) -> Option<String> {
    if code.len() == 2 && code.chars().all(|c| c.is_ascii_alphabetic()) {
        let base = 0x1F1E6 - 'A' as u32;
        let flag = code.to_uppercase()
            .chars()
            .map(|c| format!("{:x}", base + c as u32))
            .collect::<Vec<_>>()
            .join("-");
        Some(flag)
    } else {
        None
    }
}
