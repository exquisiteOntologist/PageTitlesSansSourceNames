static TITLES_SOURCE_RIGHT: [&str; 4] = [
    "This is a title - FANGS",
    "This is another title - a title with a dash - FANGS",
    "Hello - FANGS",
    "123 What 3 . - - - Fangs",
];

// There should be two strategies
// 1) Many titles, split either side of the leftmost and rightmost dash.
// Compare the text from either side of the split to see if they are the same
// 2) Only a single title... use the
fn main() {
    println!("Processing titles");
    strip_titles(&TITLES_SOURCE_RIGHT);
}

fn strip_titles(titles: &[&str]) {
    if titles.len() > 1 {
        strip_titles_multiple(titles);
    } else {
        // strip_titles_single(titles);
    }
}

enum TitleSourcePos {
    Null,
    End,
    Start,
}

struct TitleSource {
    name: String,
    length: usize,
    position: TitleSourcePos,
}

fn strip_titles_multiple(titles: &[&str]) {
    let t_s = titles_locate_matching_source(&titles);

    for title in titles {
        // TODO: Refactor to use character positions instead of .len() ?
        let pure_title: &str = match t_s.position {
            TitleSourcePos::Null => &title,
            TitleSourcePos::End => &title[..(title.len() - t_s.length + 1)],
            TitleSourcePos::Start => &title[t_s.length..],
        };
        println!("Stripped");
        println!("{}", title);
        println!("{}", pure_title);
    }
}

fn titles_locate_matching_source(titles: &[&str]) -> TitleSource {
    let t_a = titles[0];
    let t_b = titles[1];

    let max: usize = if t_a.chars().count() > t_b.chars().count() {
        t_b.chars().count()
    } else {
        t_a.chars().count()
    };

    let t_a_c: Vec<char> = t_a.chars().collect();
    let t_b_c: Vec<char> = t_b.chars().collect();

    let mut matching_starts: Vec<char> = Vec::new();
    let mut matching_ends: Vec<char> = Vec::new();
    let mut current_char: char = '_';
    for i in 0..(max + 0) {
        if t_a_c[t_a_c.len() - i - 1] == t_b_c[t_b_c.len() - i - 1] {
            // note this puts the characters in reverse order
            matching_ends.push(t_a_c[t_a_c.len() - i - 1]);
        }
        if t_a_c[i] == t_b_c[i] {
            matching_starts.push(t_a_c[i]);
        }
    }

    let prefer_end = matching_starts.len() <= matching_ends.len();
    let name_position: TitleSourcePos = if prefer_end && matching_ends.len() > 2 {
        TitleSourcePos::End
    } else if matching_starts.len() > 2 {
        TitleSourcePos::Start
    } else {
        TitleSourcePos::Null
    };

    let source_name_len: usize = matching_ends.len();
    let source_name: String = matching_ends.iter().rev().collect();
    println!("Source name: {:?}", source_name);
    let start_name: String = matching_starts.iter().collect();
    println!("Source name if start: {:?}", start_name);

    TitleSource {
        name: source_name,
        length: source_name_len,
        position: name_position,
    }
}

static SOURCE_TITLE_SEP_CHARS: [char; 2] = ['|', '-'];

fn first_sep_pos(title: &str) -> usize {
    title.find(&SOURCE_TITLE_SEP_CHARS).unwrap_or(title.len())
}

fn last_sep_pos(title: &str) -> usize {
    title.rfind(&SOURCE_TITLE_SEP_CHARS).unwrap_or(title.len())
}
