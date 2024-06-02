static TITLES_SOURCE_RIGHT: [&str; 4] = [
    "This is a title - FANGS",
    "This is another title - a title with a dash - FANGS",
    "Hello - FANGS",
    "123 What 3 . - - - FANGS",
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

/// Locate the identical source name in each of the titles.
/// Note that this requires the titles to come from the same source,
/// and for all of the titles to consistently include the source name.
fn titles_locate_matching_source(titles: &[&str]) -> TitleSource {
    let mut max: usize = 0;
    for title in titles {
        let count = title.chars().count();
        if count > max {
            max = count;
        }
    }

    println!("Max chars (lowest title length) {}", max);

    let mut matching_starts: Vec<&char> = Vec::new();
    let mut matching_ends: Vec<&char> = Vec::new();

    let titles_title_chars: Vec<Vec<char>> = titles
        .into_iter()
        .map(|t| t.chars().collect::<Vec<char>>())
        .collect();
    'chars: for i in 0..max {
        let mut current_char_left: &char = titles_title_chars.get(0).unwrap().get(i).unwrap();
        let mut current_char_right: &char = titles_title_chars
            .get(0)
            .unwrap()
            .get(titles_title_chars[0].len() - i - 1)
            .unwrap();

        println!(
            "c: {:1} ccl: {:2} ccr: {:3}",
            i, current_char_left, current_char_right
        );

        'titles: for (t_i, title) in titles_title_chars.iter().enumerate() {
            let ri = title.len() - i - 1;
            println!(
                "tc: {:1} t: {:2} l: {:3} r: {:4}",
                i,
                title.clone().iter().collect::<String>(),
                title[i],
                title[ri]
            );
            let right_match = &title[ri] == current_char_right;
            let left_match = &title[i] == current_char_left;

            if !left_match && !right_match {
                // no more matches so stop looping
                break 'chars;
            }

            if t_i == (titles.len() - 1) {
                println!("last title");
                if right_match {
                    // right matched last char
                    // note this puts the characters in reverse order because rev iter (end - i)
                    matching_ends.push(&title[ri]);
                }

                if left_match {
                    // left matched last char
                    matching_starts.push(&title[i]);
                }
            }

            current_char_right = &title[ri];
            current_char_left = &title[i];
        }

        println!(
            "Text start {:?}",
            matching_starts.clone().into_iter().collect::<String>()
        );

        println!(
            "Text end {:?}",
            matching_ends.clone().into_iter().collect::<String>()
        );
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
    let source_name: String = matching_ends.into_iter().rev().collect();
    println!("Source name: {:?}", source_name);
    let start_name: String = matching_starts.into_iter().collect();
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
