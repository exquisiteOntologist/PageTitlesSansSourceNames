use titles::strip::strip_titles;

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

// In this instance it is easier to just keep the repeated source
static TITLES_SOURCE_DUAL: [&str; 2] = [
    "The Paris Review  -  “Practice Tantric Exodus”: Tuning into Burning Man - The Paris Review",
    "The Paris Review  -  At the Webster Apartments: One of Manhattan's Last All-Women's Boarding Houses - The Paris Review",
];

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_strip_source_left() {
        strip_titles(&TITLES_SOURCE_LEFT);
    }

    #[test]
    fn test_strip_source_right() {
        strip_titles(&TITLES_SOURCE_RIGHT);
    }

    #[test]
    fn test_strip_source_single() {
        strip_titles(&TITLES_SOURCE_SINGLE);
    }

    #[test]
    fn test_strip_source_single_alt() {
        strip_titles(&TITLES_SOURCE_SINGLE_ALT);
    }

    #[test]
    fn test_strip_source_dual() {
        strip_titles(&TITLES_SOURCE_DUAL);
    }
}
