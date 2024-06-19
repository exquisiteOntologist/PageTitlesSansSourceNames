use crate::entities::Title;

enum TitleSourcePos {
    Null,
    End,
    Start,
}

struct TitleSource {
    _name: String,
    length: usize,
    position: TitleSourcePos,
}

/// Remove the source text from the given titles.
/// This caters to when the source is included multiple times.
/// If there is no 2nd source, the 2nd pass will end quickly.
pub fn strip_titles_multiple_double_pass<'a>(titles: Vec<Title<'a>>) -> Vec<Title<'a>> {
    let pass_one = strip_titles_multiple(titles);
    let pass_two = strip_titles_multiple(pass_one);
    pass_two
}

/// Pass over all the titles, finding identical source text, and removing the source text.
pub fn strip_titles_multiple<'a>(titles: Vec<Title<'a>>) -> Vec<Title<'a>> {
    let t_s = titles_locate_matching_source(&titles);

    let new_titles = titles.into_iter().map(|t| -> Title<'a> {
        let title = t.title;
        // TODO: Refactor to use character positions instead of .len() ?
        let pure_title: &str = match t_s.position {
            TitleSourcePos::Null => &title,
            TitleSourcePos::End => &title[..(title.len() - t_s.length + 1)],
            TitleSourcePos::Start => &title[t_s.length..],
        }
        .trim();

        // println!("Stripped");
        // println!("{}", title);
        // println!("{}", pure_title);

        Title {
            id: t.id,
            title: pure_title,
        }
    });

    new_titles.collect::<Vec<Title<'a>>>()
}

/// Locate the identical source name that is in all of the page titles.
/// Note that this requires the titles to come from the same source,
/// and for all of the titles to consistently include the source name.
fn titles_locate_matching_source<'a>(titles: &'a Vec<Title<'a>>) -> TitleSource {
    let mut max: usize = 0;
    for t in titles {
        let count = t.title.chars().count();
        if count > max {
            max = count;
        }
    }

    // println!("Max chars (lowest title length) {}", max);

    let mut matching_starts: Vec<&char> = Vec::new();
    let mut matching_ends: Vec<&char> = Vec::new();

    let titles_title_chars: Vec<Vec<char>> = titles
        .into_iter()
        .map(|t| t.title.chars().collect::<Vec<char>>())
        .collect();
    'chars: for i in 0..max {
        let first_title: &Vec<char> = match titles_title_chars.get(0) {
            Some(v) => v,
            None => {
                eprintln!("Error getting first title");
                &Vec::new()
            }
        };
        if first_title.is_empty() {
            eprintln!("The first titles in the set is empty");
            break 'chars;
        }
        let mut current_char_left: &char = match first_title.get(i) {
            Some(v) => v,
            None => break 'chars,
        };
        let mut current_char_right: &char = match first_title.get(first_title.len() - i - 1) {
            Some(v) => v,
            None => break 'chars,
        };

        // println!(
        //     "c: {:1} ccl: {:2} ccr: {:3}",
        //     i, current_char_left, current_char_right
        // );

        '_titles: for (t_i, title) in titles_title_chars.iter().enumerate() {
            if title.len() < 1 || ((i + 1) > title.len()) {
                continue;
            }
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
        _name: source_name,
        length: source_name_len,
        position: name_position,
    }
}
