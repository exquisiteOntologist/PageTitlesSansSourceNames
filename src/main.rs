static TITLES_SOURCE_LEFT: [&str; 4] = [
    "FANGS - This is a title",
    "FANGS - This is another title - a title with a dash",
    "FANGS - Hello",
    "FANGS - - - - 123 What - 3 .",
];

static TITLES_SOURCE_RIGHT: [&str; 5] = [
    "What We Know About the Latest Gaza Cease-Fire Proposal - The New York Times",
    "Doctor-Assisted Death Is Legal in 10 States. Could New York Be No. 11? - The New York Times",
    "Boeing Starliner Launch: Video and Live Updates - The New York Times",
    "Conversations and insights about the moment. - The New York Times",
    "Conversations and insights about the moment. - The New York Times",
];

static TITLES_SOURCE_SINGLE: [&str; 1] = ["FANGS - - - - 123 What -3."];
static TITLES_SOURCE_SINGLE_ALT: [&str; 1] = ["FANGS - Some days I subconsciously cross"];

fn main() {
    println!("Processing titles");
    strip_titles(&TITLES_SOURCE_LEFT);
    strip_titles(&TITLES_SOURCE_RIGHT);
    strip_titles(&TITLES_SOURCE_SINGLE);
    strip_titles(&TITLES_SOURCE_SINGLE_ALT);
}

// There are two main strategies depending on number of input items
// 1) Many titles, split either side of the leftmost and rightmost dash.
// Compare the text from either side of the split to see if they are the same.
// This has an advantage as it makes less assumptions. There does not need to be a separator.
// 2) Single title, find common seperator characters and use one
fn strip_titles(titles: &[&str]) {
    if titles.len() > 1 {
        strip_titles_multiple(titles);
    } else {
        strip_titles_single(titles);
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

    println!("--- === ---");
}

/// Locate the identical source name that is in all of the page titles.
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

    // println!("Max chars (lowest title length) {}", max);

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
            .get(titles_title_chars.get(0).unwrap().len() - i - 1)
            .unwrap();

        // println!(
        //     "c: {:1} ccl: {:2} ccr: {:3}",
        //     i, current_char_left, current_char_right
        // );

        '_titles: for (t_i, title) in titles_title_chars.iter().enumerate() {
            let ri = title.len() - i - 1;
            // println!(
            //     "tc: {:1} t: {:2} l: {:3} r: {:4}",
            //     i,
            //     title.clone().iter().collect::<String>(),
            //     title[i],
            //     title[ri]
            // );
            let right_match = &title[ri] == current_char_right;
            let left_match = &title[i] == current_char_left;

            if !left_match && !right_match {
                // no more matches so stop looping
                break 'chars;
            }

            if t_i == (titles.len() - 1) {
                // println!("last title");
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

        // println!(
        //     "Text start {:?}",
        //     matching_starts.clone().into_iter().collect::<String>()
        // );

        // println!(
        //     "Text end {:?}",
        //     matching_ends.clone().into_iter().collect::<String>()
        // );
    }

    let prefer_end = matching_starts.len() <= matching_ends.len();
    let name_position: TitleSourcePos = if prefer_end && matching_ends.len() > 2 {
        TitleSourcePos::End
    } else if matching_starts.len() > 2 {
        TitleSourcePos::Start
    } else {
        TitleSourcePos::Null
    };

    let source_name_len: usize = match name_position {
        TitleSourcePos::End => matching_ends.len(),
        TitleSourcePos::Start => matching_starts.len(),
        TitleSourcePos::Null => 0,
    };
    let source_name: String = match name_position {
        TitleSourcePos::End => matching_ends.into_iter().rev().collect(),
        TitleSourcePos::Start => matching_starts.into_iter().collect(),
        TitleSourcePos::Null => String::new(),
    };

    // println!("Source name: {:?}", source_name);
    // let start_name: String = matching_starts.into_iter().collect();
    // println!("Source name if start: {:?}", start_name);

    TitleSource {
        name: source_name,
        length: source_name_len,
        position: name_position,
    }
}

fn strip_titles_single(titles: &[&str]) {
    for title in titles {
        let t_s_l = first_sep_pos(title);
        let t_s_r = last_sep_pos(title);
        let t_s_r_len = title.len() - t_s_r;

        // here we assume the source is shorter than the title
        let pure_title: &str = if t_s_l == 0 && t_s_r == 0 {
            title
        } else if t_s_l < t_s_r_len {
            println!("right, sl");
            &title[(t_s_l + 1)..]
        } else {
            println!("left, sr");
            &title[..(t_s_r - 1)]
        }
        .trim();

        println!("Stripped");
        println!("{}", title);
        println!("{}", pure_title);
    }
}

static SOURCE_TITLE_SEP_CHARS: [char; 2] = ['|', '-'];

fn first_sep_pos(title: &str) -> usize {
    title.find(&SOURCE_TITLE_SEP_CHARS).unwrap_or(0)
}

fn last_sep_pos(title: &str) -> usize {
    title.rfind(&SOURCE_TITLE_SEP_CHARS).unwrap_or(0)
}
