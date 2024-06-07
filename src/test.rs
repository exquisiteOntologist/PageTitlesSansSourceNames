use crate::{entities::Title, strip::strip_titles};

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

static TITLES_SOURCE_NONE: [&str; 5] = [
    "What We Know About the Latest Gaza Cease-Fire Proposal",
    "Doctor-Assisted Death Is Legal in 10 States. Could New York Be No. 11?",
    "Boeing Starliner Launch: Video and Live Updates",
    "Conversations and insights about the moment.",
    "Conversations and insights about the moment.",
];

static TITLES_SOURCE_SINGLE: [&str; 1] = ["FANGS - - - - 123 What -3."];

static TITLES_SOURCE_SINGLE_ALT: [&str; 1] = ["FANGS - Some days I subconsciously cross"];

// In this instance it is easier to just keep the repeated source
static TITLES_SOURCE_DUAL: [&str; 3] = [
    "The Paris Review  -  “Practice Tantric Exodus”: Tuning into Burning Man - The Paris Review",
    "The Paris Review  -  At the Webster Apartments: One of Manhattan's Last All-Women's Boarding Houses - The Paris Review",
    "The Paris Review  -  Dorm Room Art?: At the Biennale - The Paris Review"
];

static TITLES_SOME_EMPTY: [&str; 6] = [
    "",
    "The Paris Review  -  At the Webster Apartments: One of Manhattan's Last All-Women's Boarding Houses - The Paris Review",
    "The Paris Review  -  At the Webster Apartments: One of Manhattan's Last All-Women's Boarding Houses - The Paris Review",
    "",
    "The Paris Review  -  At the Webster Apartments: One of Manhattan's Last All-Women's Boarding Houses - The Paris Review",
    ""
];

fn titles_enumerated<'a>(titles: &'a [&str]) -> Vec<(i32, &'a str)> {
    titles
        .into_iter()
        .enumerate()
        .map(|(i, t)| (i as i32, *t))
        .collect()
}

fn map_title_array<'a>(titles: &'a Vec<(i32, &'a str)>) -> Vec<Title<'a>> {
    let titles = titles
        .into_iter()
        .map(|(i, t)| -> Title<'a> { Title { id: i, title: t } });
    let coll: Vec<Title<'a>> = titles.collect();
    coll
}

#[test]
fn test_strip_source_left() {
    let t_e = titles_enumerated(&TITLES_SOURCE_LEFT);
    let t = map_title_array(&t_e);
    let p_t = strip_titles(t);
    assert_eq!(p_t.len(), 4);
    assert_eq!(p_t.is_empty(), false);
    assert_eq!(p_t[1].id, &1);
    assert_eq!(p_t[3].id, &3);
    assert_eq!(p_t[0].title, "This is a title");
}

#[test]
fn test_strip_source_right() {
    let t_e = titles_enumerated(&TITLES_SOURCE_RIGHT);
    let t = map_title_array(&t_e);
    let p_t = strip_titles(t);
    assert_eq!(p_t.len(), 5);
    assert_eq!(p_t.is_empty(), false);
    assert_eq!(p_t[1].id, &1);
    assert_eq!(
        p_t[0].title,
        "What We Know About the Latest Gaza Cease-Fire Proposal"
    );
    assert_eq!(
        p_t[1].title,
        "Doctor-Assisted Death Is Legal in 10 States. Could New York Be No. 11?"
    );
    assert_eq!(
        p_t[2].title,
        "Boeing Starliner Launch: Video and Live Updates"
    );
}

#[test]
fn test_strip_source_none() {
    let t_e = titles_enumerated(&TITLES_SOURCE_NONE);
    let t = map_title_array(&t_e);
    let p_t = strip_titles(t);
    assert_eq!(p_t.len(), 5);
    assert_eq!(p_t.is_empty(), false);
    assert_eq!(p_t[1].id, &1);
    assert_eq!(
        p_t[0].title,
        "What We Know About the Latest Gaza Cease-Fire Proposal"
    );
    assert_eq!(
        p_t[1].title,
        "Doctor-Assisted Death Is Legal in 10 States. Could New York Be No. 11?"
    );
    assert_eq!(
        p_t[2].title,
        "Boeing Starliner Launch: Video and Live Updates"
    );
}

#[test]
fn test_strip_source_single() {
    let t_e = titles_enumerated(&TITLES_SOURCE_SINGLE);
    let t = map_title_array(&t_e);
    let p_t = strip_titles(t);
    assert_eq!(p_t.len(), 1);
    assert_eq!(p_t.is_empty(), false);
    assert_eq!(p_t[0].id, &0);
    assert_eq!(p_t[0].title, "- - - 123 What -3.");
}

#[test]
fn test_strip_source_single_alt() {
    let t_e = titles_enumerated(&TITLES_SOURCE_SINGLE_ALT);
    let t = map_title_array(&t_e);
    let p_t = strip_titles(t);
    assert_eq!(p_t.len(), 1);
    assert_eq!(p_t.is_empty(), false);
    assert_eq!(p_t[0].id, &0);
    assert_eq!(p_t[0].title, "Some days I subconsciously cross");
}

#[test]
fn test_strip_source_dual() {
    let t_e = titles_enumerated(&TITLES_SOURCE_DUAL);
    let t = map_title_array(&t_e);
    let p_t = strip_titles(t);
    assert_eq!(p_t.len(), 3);
    assert_eq!(p_t.is_empty(), false);
    assert_eq!(p_t[0].id, &0);
    assert_eq!(p_t[1].id, &1);
    assert_eq!(
        p_t[0].title,
        "“Practice Tantric Exodus”: Tuning into Burning Man"
    );
    assert_eq!(
        p_t[1].title,
        "At the Webster Apartments: One of Manhattan's Last All-Women's Boarding Houses"
    );
    assert_eq!(p_t[2].title, "Dorm Room Art?: At the Biennale");
}

#[test]
fn test_strip_some_empty() {
    let t_e = titles_enumerated(&TITLES_SOME_EMPTY);
    let t = map_title_array(&t_e);
    let p_t = strip_titles(t);
    assert_eq!(p_t.len(), 6);
    assert_eq!(p_t.is_empty(), false);
    assert_eq!(p_t[0].id, &0);
    assert_eq!(p_t[1].id, &1);
    assert_eq!(p_t[0].title, "");
    assert_eq!(
        p_t[1].title,
        "At the Webster Apartments: One of Manhattan's Last All-Women's Boarding Houses"
    );
}
