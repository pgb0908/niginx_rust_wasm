pub fn ensure_or_generate(existing: Option<String>, prefix: &str, context_id: u32) -> String {
    match existing {
        Some(value) if !value.is_empty() => value,
        _ => format!("{}-{}", prefix, context_id),
    }
}
