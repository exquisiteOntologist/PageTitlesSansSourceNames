use crate::entities::Title;

pub fn strip_titles_single<'a>(titles: Vec<Title<'a>>) -> Vec<Title<'a>> {
    let new_titles = titles.into_iter().map(|t| -> Title<'a> {
        let title = t.title;
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

        Title {
            id: t.id,
            title: pure_title,
        }
    });

    new_titles.collect::<Vec<Title<'a>>>()
}

static SOURCE_TITLE_SEP_CHARS: [char; 2] = ['|', '-'];

fn first_sep_pos(title: &str) -> usize {
    title.find(&SOURCE_TITLE_SEP_CHARS).unwrap_or(0)
}

fn last_sep_pos(title: &str) -> usize {
    title.rfind(&SOURCE_TITLE_SEP_CHARS).unwrap_or(0)
}
