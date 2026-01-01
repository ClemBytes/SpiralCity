use std::collections::HashMap;

struct State {
    spiral: HashMap<(i32, i32), Self>,
    x_bounds: (i32, i32),
    y_bounds: (i32, i32),

}

enum Building {
    House,
    Forest,
    Quarry,
    Workshop,
}

impl Building {
    fn convert_to_char(&self) -> char {
        match *self {
            Building::House => '🏠',
            Building::Forest => '🌲',
            Building::Quarry => '🪨',
            Building::Workshop => '🪚',
         }
    }

    fn print_spiral() {

    }
}

fn main() {
    println!("Hello, world!");
}