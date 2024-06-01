static SOURCE_TITLE_SEP_CHARS: [char; 2] = ['|', '-'];

static TITLES_SOURCE_RIGHT: [&str; 4] = [
    "This is a title - FANGS",
    "This is another title - a title with a dash - FANGS",
    "Hello - FANGS",
    "123 What 3 . - - - Fangs",
];

// There should be two strategies
// 1) Many titles, split either side of the leftmost and rightmost dash.
// Compare the text from either side of the split to see if they are the same
fn main() {
    println!("Processing titles");
    strip_titles(&TITLES_SOURCE_RIGHT);
}

fn strip_titles(titles: &[&str]) {
    // let mut left_sep: &str = "";
    // let mut right_sep: &str = "";
    // let mut prior_title: &str = "";

    // for title in titles {
    //     println!("Title:");
    //     println!("{}", title);

    //     title.split(SOURCE_TITLE_SEP_CHARS);

    //     prior_title = title;
    // }
    //
    if titles.len() > 1 {
        strip_titles_multiple(titles);
    } else {
        // strip_titles_single(titles);
    }
}

fn strip_titles_multiple(titles: &[&str]) {
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
    for i in 1..(max + 1) {
        if t_a_c[t_a_c.len() - i] == t_b_c[t_b_c.len() - i] {
            // note this puts the characters in reverse order
            matching_ends.push(t_a_c[t_a_c.len() - i]);
        }
        // if t_a_c[X - i] == t_b_c[X - i] {
        //     matching_starts.push(t_a_c[X - i]);
        // }
    }

    let use_end = matching_starts.len() <= matching_ends.len();

    if matching_ends.len() != 0 {
        // let mut has_sep: &bool = false;
    }

    let source_name_len: usize = matching_ends.len();
    let source_name: String = matching_ends.iter().rev().collect();
    println!("Source name: {}", source_name);

    for title in titles {
        // TODO: Refactor to use character positions
        let pure_title: &str = &title[0..(title.len() - source_name_len + 1)];
        println!("Stripped");
        println!("{}", title);
        println!("{}", pure_title);
    }
}

// fn strip_titles_multiple(titles: &[&str]) {
//     let title_a = titles[0];
//     let title_b = titles[1];

//     let title_a_split: Vec<&str> = title_a.split(SOURCE_TITLE_SEP_CHARS).collect();
//     let title_b_split: Vec<&str> = title_b.split(SOURCE_TITLE_SEP_CHARS).collect();

//     let end_match = title_a_split.last() == title_b_split.last();

//     if end_match {
//         let max: usize = if title_a_split.len() > title_b_split.len() {
//             title_b_split.len()
//         } else {
//             title_a_split.len()
//         };
//         // most of the time source names don't themselves have dashes, but this caters to those too.
//         let mut source_name: Vec<&str> = Vec::new();
//         for i in 0..max {
//             if title_a_split[title_a_split.len() - i] != title_b_split[title_b_split.len() - i] {
//                 // both titles have different lengths, so counting this way
//                 break;
//             }
//             source_name.push(title_a_split[title_a_split.len() - i]);
//         }
//     }

//     let start_match = title_a_split.first() == title_b_split.first();

//     if start_match {
//         let max: usize = if title_a_split.len() > title_b_split.len() {
//             title_b_split.len()
//         } else {
//             title_a_split.len()
//         };
//         let mut source_name: Vec<&str> = Vec::new();
//         for i in 0..max {
//             if title_a_split[i] != title_b_split[i] {
//                 // both titles have different lengths, so counting this way
//                 break;
//             }
//             source_name.push(title_a_split[i]);
//         }
//     }
// }

// fn strip_titles_single<'a>(titles: &'a [&'a str]) -> &'a [&'a str] {
//     titles
//         .into_iter()
//         .map(|t| t.replace_range(last_sep_pos(t).., ""))
// }

fn first_sep_pos(title: &str) -> usize {
    title.find(&SOURCE_TITLE_SEP_CHARS).unwrap_or(title.len())
}

fn last_sep_pos(title: &str) -> usize {
    title.rfind(&SOURCE_TITLE_SEP_CHARS).unwrap_or(title.len())
}
