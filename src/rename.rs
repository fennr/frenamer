use anyhow::{Result, bail};

pub fn generate_new_filename(filename: &str) -> Result<String> {
    if filename.is_empty() {
        bail!("Empty filename");
    }

    let (name, ext) = split_filename(filename);
    
    let numbers: String = name.chars().filter(|c| c.is_numeric()).collect();
    let text: String = name.chars()
        .filter(|c| !c.is_numeric())
        .map(|c| if c == '.' || c == '-' { '_' } else { c })
        .collect();
    
    let text = text.trim_matches('_');
    
    Ok(if !text.is_empty() && !numbers.is_empty() && !ext.is_empty() {
        format!("{}_{}.{}", text, numbers, ext)
    } else if !text.is_empty() && !ext.is_empty() {
        format!("{}.{}", text, ext)
    } else {
        filename.to_string()
    })
}

fn split_filename(filename: &str) -> (&str, &str) {
    match filename.rfind('.') {
        Some(pos) => (&filename[..pos], &filename[pos + 1..]),
        None => (filename, ""),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generate_new_filename() -> Result<()> {
        assert_eq!(
            generate_new_filename("35.search-insert-position.rs")?,
            "search_insert_position_35.rs"
        );
        assert_eq!(
            generate_new_filename("test-file.123.txt")?,
            "test_file_123.txt"
        );
        assert_eq!(
            generate_new_filename("nodigits.txt")?,
            "nodigits.txt"
        );
        assert_eq!(
            generate_new_filename("123.txt")?,
            "123.txt"
        );
        Ok(())
    }

    #[test]
    fn test_empty_filename() {
        let result = generate_new_filename("");
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("Empty filename"));
    }

    #[test]
    fn test_split_filename() {
        assert_eq!(split_filename("test.txt"), ("test", "txt"));
        assert_eq!(split_filename("test"), ("test", ""));
        assert_eq!(split_filename(".gitignore"), ("", "gitignore"));
        assert_eq!(split_filename("multiple.dots.txt"), ("multiple.dots", "txt"));
    }
}
