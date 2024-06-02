pub fn strip_titles_single(titles: &[&str]) {
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
