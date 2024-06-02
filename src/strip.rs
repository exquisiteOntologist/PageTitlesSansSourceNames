use crate::{
    entities::Title, multiple::strip_titles_multiple_double_pass, single::strip_titles_single,
};

// There are two main strategies depending on number of input items
// 1) Many titles, split either side of the leftmost and rightmost dash.
// Compare the text from either side of the split to see if they are the same.
// This has an advantage as it makes less assumptions. There does not need to be a separator.
// 2) Single title, find common seperator characters and use one
pub fn strip_titles<'a>(titles: Vec<Title<'a>>) -> Vec<Title<'a>> {
    println!("-- === ---");

    if titles.len() > 1 {
        strip_titles_multiple_double_pass(titles)
    } else {
        strip_titles_single(titles)
    }
}
