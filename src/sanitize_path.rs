#![warn(clippy::all)]

use lazy_static::lazy_static;
use std::path::Path;

const LOGGING: bool = false;

const DEFAULT_REPLACEMENT: char = '!';

const MAX_FILENAME_LENGTH: usize = 248;

const UNIX_BANNED_ASCII_CHARS: [char; 1] = ['/'];
const WINDOWS_BANNED_ASCII_CHARS: [char; 9] = ['<', '>', ':', '"', '/', '\\', '*', '|', '?'];

lazy_static! {
    static ref UNIX_BANNED_CTRL_CHARS: Vec<u8> = (0..1).collect();
    static ref WINDOWS_BANNED_CTRL_CHARS: Vec<u8> = (0..32).collect();
}

lazy_static! {
    static ref WINDOWS_RESERVED_FILE_NAMES: Vec<&'static str> = vec![
        "CON", "PRN", "AUX", "NUL", "COM1", "COM2", "COM3", "COM4", "COM5", "COM6", "COM7", "COM8",
        "COM9", "LPT1", "LPT2", "LPT3", "LPT4", "LPT5", "LPT6", "LPT7", "LPT8", "LPT9",
    ];
}

#[allow(dead_code)]
pub enum OsTarget {
    Windows,
    Unix,
    All,
}

// TODO: Vedere come riuscire a ritornare &Path
fn windows(path: &str, replacement: Option<char>) -> String {
    let replacement = replacement.unwrap_or(DEFAULT_REPLACEMENT);

    let filename = {
        let path_str = if path.ends_with("..") {
            &path[..path.len() - 2]
        } else if path.ends_with("/") {
            &path[..path.len() - 1]
        } else {
            path
        };

        let path = Path::new(&path_str);
        path.file_name().unwrap().to_string_lossy().to_string()
    };

    let mut new_path = path.to_string();

    if WINDOWS_RESERVED_FILE_NAMES.contains(&filename.trim()) {
        if LOGGING {
            println!("Windows reserved file name: `{}`", filename);
        }
        new_path = new_path.replace(&filename, (filename.clone() + "!").as_str());
    }

    for ch in &WINDOWS_BANNED_ASCII_CHARS {
        if new_path.contains(&ch.to_string()) {
            if LOGGING {
                println!("Windows banned ASCII char: `{}`", ch);
            }
            new_path = new_path.replace(&ch.to_string(), &replacement.to_string());
        }
    }

    for ctrl in WINDOWS_BANNED_CTRL_CHARS.iter() {
        if new_path.as_bytes().contains(ctrl) {
            if LOGGING {
                println!("Windows banned CTRL char: `{}`", ctrl);
            }
            new_path = new_path.replace(&ctrl.to_string(), &replacement.to_string());
        }
    }

    new_path
}

// TODO: Vedere come riuscire a ritornare &Path
fn unix(path: &str, replacement: Option<char>) -> String {
    let mut new_path = path.to_string();
    let replacement = replacement.unwrap_or(DEFAULT_REPLACEMENT);

    for ch in &UNIX_BANNED_ASCII_CHARS {
        if new_path.contains(&ch.to_string()) {
            if LOGGING {
                println!("UNIX banned ASCII char: `{}`", ch);
            }
            new_path = new_path.replace(&ch.to_string(), &replacement.to_string());
        }
    }

    for ctrl in UNIX_BANNED_CTRL_CHARS.iter() {
        if new_path.as_bytes().contains(ctrl) {
            if LOGGING {
                println!("UNIX banned CTRL char: `{}`", ctrl);
            }
            new_path = new_path.replace(&ctrl.to_string(), &replacement.to_string());
        }
    }

    new_path
}

pub fn sanitize(path: &str, replacement: Option<char>, target: Option<OsTarget>) -> String {
    let path = if path.len() > MAX_FILENAME_LENGTH {
        &path[..=MAX_FILENAME_LENGTH]
    } else {
        path
    };

    let target = target.unwrap_or(OsTarget::All);
    let replacement = replacement.unwrap_or(DEFAULT_REPLACEMENT);

    match target {
        OsTarget::All => unix(&windows(path, Some(replacement)), Some(replacement)),
        OsTarget::Windows => windows(path, Some(replacement)),
        OsTarget::Unix => unix(path, Some(replacement)),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let paths = vec![
            format!("    {}   .mp4", WINDOWS_RESERVED_FILE_NAMES[0]),
            {
                let mut s = String::new();
                for ch in &WINDOWS_BANNED_ASCII_CHARS {
                    s.push_str(&format!("{},", ch));
                }
                s
            },
            std::str::from_utf8(&[
                WINDOWS_BANNED_CTRL_CHARS[7],
                UNIX_BANNED_CTRL_CHARS[0],
                WINDOWS_BANNED_CTRL_CHARS[5],
            ])
            .unwrap()
            .to_string(),
            std::str::from_utf8(&[50_u8; MAX_FILENAME_LENGTH + 100])
                .unwrap()
                .to_string(),
        ];

        for path in paths {
            let _ = sanitize(&path, None, None);
        }
    }
}
