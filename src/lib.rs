use ncurses::*;

pub struct MenuResult {
    pub index: usize,
    pub key: char,
}

pub fn create_menu(title: &str, options: Vec<&str>, keys: Vec<char>) -> MenuResult {
    let mut selected = 0;

    let mut key: char = ' ';

    let mut repead = true;

    initscr();
    noecho();

    curs_set(CURSOR_VISIBILITY::CURSOR_INVISIBLE);

    while repead == true {
        clear();
        let t: String = " ".to_string() + title + "\n " + "-".repeat(title.len()).as_str();
        addstr(&t);

        for (index, option) in options.iter().enumerate() {
            addstr("\n");
            if index == selected {
                let str = "*  ".to_string() + option;
                addstr(&str);
            } else {
                let str = "  ".to_string() + option;
                addstr(&str);
            }
        }

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
                if k == *key_blucle {
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
