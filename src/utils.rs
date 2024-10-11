//! Function utilities for printing data to the screen

/// Format the number in diffrent data units of measurement.
/// The base can only be `1000` or `1024`.
///
/// ## Errors
/// The base is not `1000` or `1024`.
pub fn fmt_human(number: usize, base: usize) -> String {
    let suffixes = match base {
        1000 => vec!["", "k", "M", "G", "T", "P", "E", "Z", "Y"],
        1024 => vec!["", "Ki", "Mi", "Gi", "Ti", "Pi", "Ei", "Zi", "Yi"],
        _ => panic!("No such base: {}", base),
    };

    let mut scaled = number;
    let mut suffix = "";
    for s in suffixes.iter() {
        scaled /= base;
        if scaled < base {
            suffix = s;
            break;
        }
    }

    format!("{scaled} {suffix}")
}

/// Get different colors for procentage levels
pub fn level_color(percentage: f64) -> &'static str {
    if percentage <= 25.0 {
        "^fg(2edb2e)"
    } else if percentage <= 75.0 {
        return "^fg(ed8302)";
    } else {
        return "^fg(e8460b)";
    }
}
