#[cfg(test)]
mod tests {
    use life_tracker::clean_input::sanitize_html;


    #[test]
    fn test_sanitize_html_removes_script_tags() {
        let input = "<script>alert('Hello');</script><p>Safe content</p>";
        let expected = "<p>Safe content</p>";
        assert_eq!(sanitize_html(input), expected);
    }

    #[test]
    fn test_sanitize_html_allows_safe_tags() {
        let input = "<b>Bold</b> <i>Italic</i> <u>Underline</u>";
        let expected = "<b>Bold</b> <i>Italic</i> <u>Underline</u>";
        assert_eq!(sanitize_html(input), expected);
    }

    #[test]
    fn test_sanitize_html_removes_unsafe_attributes() {
        let input = "<a href=\"javascript:alert('Hello')\">Click me</a>";
        let expected = "<a>Click me</a>";
        assert_eq!(sanitize_html(input), expected);
    }

    #[test]
    fn test_sanitize_html_removes_sql_injection_attempts() {
        let input = "<div>Robert'); DROP TABLE Students;--</div>";
        let expected = "<div>Robert'); DROP TABLE Students;--</div>";
        assert_eq!(sanitize_html(input), expected);
    }
}