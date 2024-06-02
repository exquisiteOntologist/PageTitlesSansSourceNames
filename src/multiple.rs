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

pub fn strip_titles_multiple(titles: &[&str]) {
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
