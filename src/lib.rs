use ncurses::addstr;
use ncurses::attroff;
use ncurses::attron;
use ncurses::clear;
use ncurses::curs_set;
use ncurses::endwin;
use ncurses::getch;
use ncurses::init_pair;
use ncurses::initscr;
use ncurses::noecho;
use ncurses::start_color;
use ncurses::A_BOLD;
use ncurses::A_ITALIC;
use ncurses::A_UNDERLINE;
use ncurses::COLOR_BLACK;
use ncurses::COLOR_GREEN;
use ncurses::COLOR_MAGENTA;
use ncurses::COLOR_PAIR;
use ncurses::COLOR_WHITE;
use ncurses::CURSOR_VISIBILITY;

pub struct MenuResult {
    pub index: usize,
    pub key: char,
}

pub struct Key {
    pub key: char,
    pub description: String,
}

pub fn instructions_off() -> Vec<Key> {
    let vec: Vec<Key> = vec![Key {
        key: ' ',
        description: String::from(""),
    }];
    vec
}

pub fn create_menu(
    title: &str,
    options: Vec<&str>,
    keys: Vec<Key>,
    instructions: bool,
) -> MenuResult {
    let mut selected = 0;

    let mut key: char = ' ';

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

    let max_caracters = options.iter().map(|x| x.len()).max().unwrap();
    let title_len = title.len();

    let max_len = if max_caracters > title_len {
        max_caracters + 4
    } else {
        title_len + 4
    };

    while repead == true {
        clear();

        if instructions == true {
            attron(COLOR_PAIR(4));
            let instructions = "- Use W and S to move up and down, and D to select";
            addstr(instructions);
            addstr("\n");
            attroff(COLOR_PAIR(4));
        }

        for key_bucle in keys.iter() {
            let instr_off = instructions_off();

            if key_bucle.key != instr_off[0].key {
                attron(COLOR_PAIR(4));
                addstr("- ");
                addstr(&key_bucle.description);
                addstr("\n");
                attroff(COLOR_PAIR(4));
            }
        }

        create_divider(max_len);

        addstr("\n");

        create_title(title, max_len);

        addstr("\n");

        create_divider(max_len);

        for (index, option) in options.iter().enumerate() {
            addstr("\n");

            if index == selected {
                create_option(option, max_len, true);
            } else {
                create_option(option, max_len, false);
            }
        }

        addstr("\n");
        create_divider(max_len);

        let k = getch() as u8 as char;

        if k == 'A' || k == 'a' {
            repead = false;
        } else if k == 'W' || k == 'w' && selected > 0 {
            selected -= 1;
            repead = true;
        } else if k == 'S' || k == 's' && selected < options.len() - 1 {
            selected += 1;
            repead = true;
        } else if k == 'D' || k == 'd' {
            repead = false;
        } else {
            for key_blucle in keys.iter() {
                if k == key_blucle.key {
                    key = k;
                    repead = false;
                }
            }
        }
    }

    endwin();
    MenuResult {
        index: selected,
        key: key,
    }
}

fn create_divider(max_len: usize) {
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

fn create_title(title: &str, max_len: usize) {
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

fn create_option(option: &str, max_len: usize, selected: bool) {
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
