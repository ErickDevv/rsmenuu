use ncurses::attron;
use ncurses::clear;
use ncurses::curs_set;
use ncurses::endwin;
use ncurses::getch;
use ncurses::init_pair;
use ncurses::initscr;
use ncurses::mv;
use ncurses::noecho;
use ncurses::start_color;
use ncurses::stdscr;
use ncurses::COLOR_PAIR;
use ncurses::CURSOR_VISIBILITY;
use ncurses::{addstr, attroff};
use ncurses::{getmaxx, getmaxy};
use ncurses::{A_BOLD, A_ITALIC, A_UNDERLINE};
use ncurses::{COLOR_BLACK, COLOR_GREEN, COLOR_MAGENTA, COLOR_WHITE};

pub struct MenuResult {
    pub selected: usize,
    pub key: char,
}

pub fn generate_menu(
    title: &str,
    description: &str,
    options: Vec<&str>,
    instructions: Vec<&str>,
    keys: Vec<char>,
) -> MenuResult {
    let mut selected = 0;

    let mut repead = true;

    initscr();
    noecho();

    start_color();

    init_pair(1, COLOR_MAGENTA, COLOR_BLACK);
    init_pair(2, COLOR_GREEN, COLOR_BLACK);
    init_pair(3, COLOR_BLACK, COLOR_WHITE);
    init_pair(4, COLOR_WHITE, COLOR_BLACK);

    curs_set(CURSOR_VISIBILITY::CURSOR_INVISIBLE);

    attron(A_BOLD());

    let max_x = getmaxx(stdscr());
    let max_y = getmaxy(stdscr());

    let title_len = title.len();
    let desciption_len = description.len();
    let instructions_len = instructions.iter().map(|x| x.len()).max().unwrap();
    let options_len = options.iter().map(|x| x.len()).max().unwrap();

    let space: usize = 6;

    let max_len =
        if title_len > desciption_len && title_len > instructions_len && title_len > options_len {
            title_len + space
        } else if desciption_len > title_len
            && desciption_len > instructions_len
            && desciption_len > options_len
        {
            desciption_len + space
        } else if instructions_len > title_len
            && instructions_len > desciption_len
            && instructions_len > options_len
        {
            instructions_len + space
        } else {
            options_len + space
        };

    let mut key: char = ' ';

    while repead == true {
        clear();

        let mut center_count: i32 = -1;

        macro_rules! center {
            () => {{
                center_count = center_count + 1;
                mv(
                    (max_y / 2) - ((options.len() as i32 / 2) + 2) + center_count,
                    (max_x / 2) - (max_len as i32 / 2) - 1,
                );
            }};
        }

        center!();
        generate_divider(max_len);

        addstr("\n");

        center!();
        generate_title(title, max_len);

        addstr("\n");
        center!();
        generate_description(description, max_len);

        addstr("\n");

        for instruction in instructions.iter() {
            center!();
            generate_instruction(instruction, max_len);
        }

        center!();
        generate_divider(max_len);

        for (index, option) in options.iter().enumerate() {
            addstr("\n");

            if index == selected {
                center!();
                generate_option(option, max_len, true);
            } else {
                center!();
                generate_option(option, max_len, false);
            }
        }

        addstr("\n");
        center!();
        generate_divider(max_len);

        key = getch() as u8 as char;

        if key == 'W' || key == 'w' && selected > 0 {
            selected -= 1;
            repead = true;
        } else if key == 'S' || key == 's' && selected < options.len() - 1 {
            selected += 1;
            repead = true;
        } else if key == 'D' || key == 'd' {
            repead = false;
        } else {
            repead = true;
        }

        for k in keys.iter() {
            if *k == key {
                repead = false;
            }
        }
    }

    endwin();

    MenuResult { selected, key }
}

pub fn generate_divider(max_len: usize) {
    attron(COLOR_PAIR(2));
    addstr("+");
    attron(COLOR_PAIR(1));

    for _ in 0..max_len {
        addstr("-");
    }

    attron(COLOR_PAIR(2));
    addstr("+");
    attron(COLOR_PAIR(1));
}

pub fn generate_title(title: &str, max_len: usize) {
    addstr("|");

    let title_len = title.len();
    let spaces = max_len - title_len;

    for _ in 0..spaces / 2 {
        addstr(" ");
    }

    attron(COLOR_PAIR(2));
    attron(A_UNDERLINE());
    attron(A_ITALIC());
    addstr(title);
    attroff(A_ITALIC());
    attroff(A_UNDERLINE());
    attron(COLOR_PAIR(1));

    for _ in 0..(spaces / 2) + (spaces % 2) {
        addstr(" ");
    }

    addstr("|");
}

pub fn generate_description(description: &str, max_len: usize) {
    let description_len = description.len();
    let spaces = max_len - description_len;

    addstr("|");

    for _ in 0..spaces / 2 {
        addstr(" ");
    }

    attron(COLOR_PAIR(4));
    addstr(description);
    attron(COLOR_PAIR(1));

    for _ in 0..(spaces / 2) + (spaces % 2) {
        addstr(" ");
    }

    addstr("|");
}

pub fn generate_instruction(instruction: &str, max_len: usize) {
    let instruction_len = instruction.len();
    let spaces = max_len - instruction_len;

    addstr("|");

    addstr(" -");
    attron(COLOR_PAIR(4));
    addstr(instruction);
    attron(COLOR_PAIR(1));

    for _ in 0..(spaces / 2) + (spaces % 2) {
        addstr(" ");
    }

    for _ in 0..(spaces - 3 - (spaces % 2)) / 2 {
        addstr(" ");
    }

    addstr("|");
}

pub fn generate_option(option: &str, max_len: usize, selected: bool) {
    addstr("|");

    let option_len = option.len();
    let spaces = max_len - option_len;

    for _ in 0..spaces / 2 {
        addstr(" ");
    }

    if selected == true {
        attron(COLOR_PAIR(3));
    }
    addstr(option);
    attron(COLOR_PAIR(1));

    for _ in 0..(spaces / 2) + (spaces % 2) {
        addstr(" ");
    }

    addstr("|");
}

#[macro_export]
macro_rules! create_menu {
    ($title:expr, $description:expr, $instructions:expr, $options:expr, $keys:expr) => {{
        use rsmenuu::generate_menu;
        use rsmenuu::MenuResult;

        let menu_result: MenuResult =
            generate_menu($title, $description, $options, $instructions, $keys);

        menu_result
    }};
}
