use regex::Regex;

/// Parsing symbol values from text
///
/// Format:
/// ```
/// NAME = 0x1234_5678;
/// ```
/// Output:
/// ```
/// 0x1234_5678
/// ```
///
pub fn parse_value(content: &str, name: &str) -> usize {
    let pattern = format!(r#"{}\s*=\s*(0x[0-9a-fA-F_]+)\s*;"#, name);
    let re = Regex::new(&pattern).unwrap();

    let caps = re
        .captures(content)
        .unwrap_or_else(|| panic!("Cannot find '{}'", name));
    let hex_str = caps.get(1).unwrap().as_str().replace('_', "");
    usize::from_str_radix(&hex_str[2..], 16).unwrap()
}
