pub fn format_document(text: &str) -> String {
    let indent_size = 4;
    let mut level: usize = 0;
    text.lines()
        .map(|line| {
            let trimmed = line.trim();
            if trimmed.is_empty() {
                return String::new();
            }
            let first_word = trimmed.split_whitespace().next().unwrap_or("");
            let closes_brace = trimmed.starts_with('}')
                || trimmed.starts_with(']')
                || trimmed.starts_with(')');
            let closes_keyword = matches!(
                first_word,
                "end" | "end;" | "end," | "endif"
                    | "endfor" | "endwhile" | "endfunction"
                    | "else" | "elseif" | "elif"
                    | "elsif" | "except" | "catch"
                    | "finally" | "when" | "rescue"
            );
            if closes_brace || closes_keyword {
                level = level.saturating_sub(1);
            }
            let result = format!("{}{}", " ".repeat(level * indent_size), trimmed);
            for ch in trimmed.chars() {
                match ch {
                    '{' | '[' | '(' => level += 1,
                    '}' | ']' | ')' => level = level.saturating_sub(1),
                    _ => {}
                }
            }
            let opens_keyword = matches!(
                first_word,
                "if" | "else" | "elseif" | "elif"
                    | "elsif" | "for" | "while" | "do"
                    | "loop" | "begin" | "case" | "switch"
                    | "try" | "catch" | "except" | "finally"
                    | "def" | "class" | "module" | "when"
                    | "rescue" | "unless" | "until"
            );
            let ends_with_opener = trimmed.ends_with("then")
                || trimmed.ends_with("do")
                || trimmed.ends_with("repeat");
            let has_function = trimmed.contains("function")
                && !trimmed.starts_with("end");
            if opens_keyword || ends_with_opener || has_function {
                if !trimmed.contains("end")
                    || trimmed.contains("function")
                    || closes_keyword
                {
                    level += 1;
                }
            }
            result
        })
        .collect::<Vec<_>>()
        .join("\n")
}
