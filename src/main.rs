use chrono::{DateTime, Local, TimeZone, Utc};
use chrono_tz::Europe::Stockholm;
use clap::Parser;
use encoding_rs::WINDOWS_1252;
use serde::Deserialize;
use std::{fs, ops::Add, path::PathBuf};

const MAX_WIDTH: usize = 75;
const MAX_HEIGHT: usize = 27;
const MAX_HANDLE_LENGTH: usize = 26;

#[derive(Debug)]
struct Screen {
    data: [char; MAX_WIDTH * MAX_HEIGHT],
    colors: [u8; MAX_WIDTH * MAX_HEIGHT],
}

impl Screen {
    fn new() -> Screen {
        Screen {
            data: [' '; MAX_WIDTH * MAX_HEIGHT],
            colors: [0; MAX_WIDTH * MAX_HEIGHT],
        }
    }

    fn get_index(pos: &Vector2) -> usize {
        pos.x as usize + (pos.y as usize * MAX_WIDTH)
    }

    fn draw(&mut self, character: char, color: u8, pos: &Vector2) {
        if pos.x as usize >= MAX_WIDTH || pos.y as usize >= MAX_HEIGHT {
            return;
        }

        let index = Self::get_index(pos);
        self.data[index] = character;
        self.colors[index] = color;
    }

    fn print(&self) -> String {
        let mut output = String::new();

        let mut last_color = 0;

        for row in 0..MAX_HEIGHT {
            let start = MAX_WIDTH * row;
            let end = start + MAX_WIDTH;
            for i in start..end {
                let c = self.data[i];
                let color_code = match self.colors[i] == last_color {
                    true => String::from(""),
                    false => match self.colors[i] {
                        0 => String::from("\u{001b}[0m"),
                        v => format!("\u{001b}[{}m", v),
                    },
                };
                last_color = self.colors[i];
                output.push_str(&format!("{}{}", color_code, c));
            }
            output.push_str(&format!("\n"));
        }

        return output;
    }
}

#[derive(Debug, Clone, Copy)]
struct Vector2 {
    x: isize,
    y: isize,
}

enum BoxStyle {
    Simple,
    Fancy,
}

enum BoxCorner {
    TopLeft,
    TopRight,
    BottomLeft,
    BottomRight,
}

impl BoxStyle {
    fn draw_horizontal_border(&self, screen: &mut Screen, position: &Vector2, top: bool) {
        match self {
            BoxStyle::Simple => screen.draw('-', 0, position),
            BoxStyle::Fancy => {
                screen.draw('-', 0, position);
                let r: f64 = rand::random();
                let r2: f64 = rand::random();
                if r < 0.50 {
                    let offset = match top {
                        true => Vector2 { x: 0, y: -1 },
                        false => Vector2 { x: 0, y: 1 },
                    };
                    screen.draw('.', 35, &(position.to_owned() + offset))
                }
                if r2 < 0.25 {
                    let offset = match top {
                        true => Vector2 { x: 0, y: -2 },
                        false => Vector2 { x: 0, y: 2 },
                    };
                    screen.draw('.', 35, &(position.to_owned() + offset))
                }
            }
        }
    }

    fn draw_vertical_border(&self, screen: &mut Screen, position: &Vector2, left: bool) {
        match self {
            BoxStyle::Simple => screen.draw('|', 0, position),
            BoxStyle::Fancy => {
                screen.draw('|', 0, position);
                let r: f64 = rand::random();
                let r2: f64 = rand::random();
                if r < 0.50 {
                    let offset = match left {
                        true => Vector2 { x: -1, y: 0 },
                        false => Vector2 { x: 1, y: 0 },
                    };
                    screen.draw('.', 35, &(position.to_owned() + offset))
                }
                if r2 < 0.25 {
                    let offset = match left {
                        true => Vector2 { x: -2, y: 0 },
                        false => Vector2 { x: 2, y: 0 },
                    };
                    screen.draw('.', 35, &(position.to_owned() + offset))
                }
            }
        }
    }

    fn draw_corner(&self, screen: &mut Screen, position: &Vector2, _corner: BoxCorner) {
        match self {
            _ => {
                screen.draw('+', 0, position);
            }
        }
    }
}

impl Default for BoxStyle {
    fn default() -> Self {
        Self::Simple
    }
}

impl Add for Vector2 {
    type Output = Vector2;
    fn add(self, other: Vector2) -> Vector2 {
        Vector2 {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

fn draw_string(screen: &mut Screen, position: &Vector2, string: &str, color: u8) {
    let mut row = position.y;
    let mut col = position.x;

    for c in string.chars() {
        match c {
            '\n' => {
                row = row + 1;
                col = position.x;
            }
            c => {
                col = col + 1;
                screen.draw(c, color, &Vector2 { x: col, y: row })
            }
        }
    }
}

fn draw_box(screen: &mut Screen, position: &Vector2, size: &Vector2, style: BoxStyle) {
    let position = position.clone();

    // Draw borders
    for x in position.x + 1..(position.x + size.x) {
        style.draw_horizontal_border(screen, &Vector2 { x, y: position.y }, true);
        style.draw_horizontal_border(
            screen,
            &Vector2 {
                x,
                y: position.y + size.y,
            },
            false,
        );
    }
    for y in position.y..(position.y + size.y) {
        style.draw_vertical_border(screen, &Vector2 { x: position.x, y }, true);
        style.draw_vertical_border(
            screen,
            &Vector2 {
                x: position.x + size.x,
                y,
            },
            false,
        );
    }

    // Draw corners
    style.draw_corner(screen, &position, BoxCorner::TopLeft);
    style.draw_corner(
        screen,
        &(position + Vector2 { x: size.x, y: 0 }),
        BoxCorner::TopRight,
    );
    style.draw_corner(
        screen,
        &(position + Vector2 { x: 0, y: size.y }),
        BoxCorner::BottomLeft,
    );
    style.draw_corner(
        screen,
        &(position
            + Vector2 {
                x: size.x,
                y: size.y,
            }),
        BoxCorner::BottomRight,
    );
}

fn generate_bar(width: usize, value: usize, max: usize) -> String {
    let pct = value as f64 / max as f64;
    let length = width as f64 * pct;
    let full_blocks = length.floor() as usize;
    let remainder = length % 1.0;

    let mut bar = String::from("");

    //for _i in 0..full_blocks {
    //    bar.push('█');
    //}

    //let last_block = match remainder {
    //    0.875..=1.0 => '▉',
    //    0.75..=0.875 => '▉',
    //    0.625..=0.75 => '▊',
    //    0.5..=0.625 => '▋',
    //    0.375..=0.5 => '▌',
    //    0.25..=0.375 => '▎',
    //    0.05..=0.25 => '▏',
    //    _ => ' ',
    //};

    for _i in 0..full_blocks {
        bar.push('|');
    }

    //let last_block = match remainder {
    //    0.5..=1.0 => ':',
    //    0.01..=0.5 => '-',
    //    _ => ' ',
    //};

    //bar.push(last_block);

    format!("{}", bar)
}

#[derive(Deserialize, Clone)]
struct Player {
    handle: String,
    name: String,
    level: usize,
    experience: f64,
    immortal: usize,
    start_immortal: Option<usize>,
}

impl Player {
    fn total_level(&self) -> usize {
        let immortal = match self.start_immortal {
            Some(start) => self.immortal - (start),
            None => self.immortal,
        };
        self.level + (immortal * 100)
    }
}

/// Generates a plain text leaderboard for HnS
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// RFC3339 date for when the competition starts (e.g 2024-03-01T00:00:00+01:00)
    #[arg(long)]
    start: String,

    /// RFC3339 date for when the competition end (e.g 2024-04-01T00:00:00+01:00)
    #[arg(long)]
    end: String,

    /// Greeting text file path
    #[arg(long)]
    greeting_path: PathBuf,

    /// Logo text file path
    #[arg(long)]
    logo_path: PathBuf,

    /// Path to initial User JSON data, Used to calculate total level
    #[arg(long)]
    start_data: PathBuf,

    /// Path to current User JSON data
    #[arg(long)]
    data: PathBuf,

    /// The output path (e.g ./output.txt)
    #[arg(short, long)]
    output_path: PathBuf,
}

fn main() {
    let args = Args::parse();

    let curr_date_time = Utc::now().with_timezone(&Stockholm);
    let start_date = DateTime::parse_from_rfc3339(&args.start).unwrap();
    let end_date = DateTime::parse_from_rfc3339(&args.end).unwrap();

    let current_day = curr_date_time.signed_duration_since(start_date).num_days();
    let last_day = end_date.signed_duration_since(start_date).num_days();

    let mut screen = Screen::new();
    draw_box(
        &mut screen,
        &Vector2 { x: 3, y: 3 },
        &Vector2 {
            x: (MAX_WIDTH - 6) as isize,
            y: (MAX_HEIGHT - 6) as isize,
        },
        BoxStyle::Fancy,
    );

    // Draw greeting
    //draw_box(
    //    &mut screen,
    //    &Vector2 { x: 5, y: 6 },
    //    &Vector2 {
    //        x: (MAX_WIDTH - 10) as isize,
    //        y: 4,
    //    },
    //    BoxStyle::Simple,
    //);
    let greeting =
        fs::read_to_string(args.greeting_path).expect("Greeting file could not be read!");
    draw_string(&mut screen, &Vector2 { x: 5, y: 7 }, &greeting, 0);

    // Draw logo
    let logo = fs::read_to_string(args.logo_path).expect("Logo file could not be read!");
    draw_string(&mut screen, &Vector2 { x: 4, y: 1 }, &logo, 32);

    // Draw standings
    let standing_y = 14;
    let standing_x = 3;
    let standing_width = (MAX_WIDTH - (standing_x * 2) as usize) as isize;

    draw_box(
        &mut screen,
        &Vector2 {
            x: standing_x,
            y: standing_y,
        },
        &Vector2 {
            x: standing_width,
            y: 10,
        },
        BoxStyle::Simple,
    );

    draw_box(
        &mut screen,
        &Vector2 {
            x: standing_x,
            y: standing_y - 2,
        },
        &Vector2 {
            x: standing_width,
            y: 2,
        },
        BoxStyle::Simple,
    );

    draw_string(
        &mut screen,
        &Vector2 {
            x: 4,
            y: standing_y - 3,
        },
        &format!("[Topplistan - Dag {} av {}]", current_day, last_day),
        32,
    );

    draw_string(
        &mut screen,
        &Vector2 {
            x: 31,
            y: standing_y - 3,
        },
        &format!(
            "            Uppdaterad {}",
            curr_date_time.format("%Y-%m-%d %H:%M")
        ),
        35,
    );

    draw_string(
        &mut screen,
        &Vector2 {
            x: 31,
            y: (MAX_HEIGHT as isize) - 2,
        },
        "Besök \n               för en komplett topplista",
        32,
    );
    draw_string(
        &mut screen,
        &Vector2 {
            x: 37,
            y: (MAX_HEIGHT as isize) - 2,
        },
        "http://hacknslash.thisoldcabin.net",
        0,
    );

    draw_string(
        &mut screen,
        &Vector2 {
            x: standing_x + 1,
            y: standing_y - 1,
        },
        &format!(
            "{:name_width$} | {:>3}",
            "SPELARE",
            "LEVEL",
            name_width = MAX_HANDLE_LENGTH + 2
        ),
        0,
    );

    let start_data =
        fs::read_to_string(&args.start_data).expect("Start data file could not be read!");
    let start_players: Vec<Player> = serde_json::from_str(&start_data).unwrap();

    let player_data = fs::read_to_string(&args.data).expect("Data file could not be read!");
    let players: Vec<Player> = serde_json::from_str(&player_data).unwrap();
    let mut players: Vec<Player> = players
        .iter()
        .map(|p| Player {
            handle: p.clone().handle,
            name: p.clone().name,
            level: p.level,
            experience: p.experience,
            immortal: p.immortal,
            start_immortal: match start_players.iter().find(|p2| p2.name == p.name) {
                Some(start) => Some(start.immortal),
                None => None,
            },
        })
        .collect();
    players.sort_by(|a, b| match a.total_level() != b.total_level() {
        true => b.total_level().cmp(&a.total_level()),
        false => b.experience.total_cmp(&a.experience),
    });

    let max_level = players.iter().map(|p| p.total_level()).max().unwrap();

    for i in 0..players.len().min(9) {
        draw_string(
            &mut screen,
            &Vector2 {
                x: standing_x + 1,
                y: standing_y + 1 + i as isize,
            },
            &format!(
                "{}) {:name_width$} | {:>4} {}",
                i + 1,
                players[i].handle,
                players[i].total_level(),
                generate_bar(30, players[i].total_level() as usize, max_level),
                name_width = MAX_HANDLE_LENGTH - 1
            ),
            match i {
                0 => 33,
                1 => 36,
                2 => 31,
                _ => 0,
            },
        );
    }
    // TODO: Handle end of competition

    // Competition progress
    let bar_width = 12;
    let bar_y = 4;
    let bar_x = 52;
    draw_string(
        &mut screen,
        &Vector2 { x: bar_x, y: bar_y },
        &format!("     {}", String::from("-").repeat(bar_width)),
        35,
    );
    draw_string(
        &mut screen,
        &Vector2 { x: bar_x, y: bar_y },
        &format!(
            " {}% {}",
            ((current_day as f64 / last_day as f64) * 100.0).floor(),
            generate_bar(bar_width, current_day as usize, last_day as usize)
        ),
        0,
    );

    // Print it!
    let mut output = screen.print();
    println!("{}", output);

    let encoded = WINDOWS_1252.encode(&output);
    fs::write(&args.output_path, encoded.0).unwrap();
}
