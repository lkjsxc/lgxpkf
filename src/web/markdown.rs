use pulldown_cmark::{html, Event, Options, Parser};

pub fn render_markdown(value: &str) -> String {
    let mut options = Options::empty();
    options.insert(Options::ENABLE_TABLES);
    options.insert(Options::ENABLE_STRIKETHROUGH);
    options.insert(Options::ENABLE_TASKLISTS);
    let parser = Parser::new_ext(value, options).map(|event| match event {
        Event::Html(html) | Event::InlineHtml(html) => Event::Text(html),
        other => other,
    });
    let mut output = String::new();
    html::push_html(&mut output, parser);
    output
}

pub fn note_excerpt(value: &str, max_len: usize) -> String {
    let mut excerpt = String::new();
    let mut count = 0;
    for ch in value.chars().filter(|ch| *ch != '\n' && *ch != '\r') {
        if count >= max_len {
            break;
        }
        excerpt.push(ch);
        count += 1;
    }
    if excerpt.is_empty() {
        return "Empty note".to_string();
    }
    if value.chars().count() > max_len {
        excerpt.push_str("...");
    }
    excerpt
}
