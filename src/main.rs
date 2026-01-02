use std::collections::HashMap;

struct Ressources {
    total_people: u32,
    occupied_people: u32,
    wood: u32,
    rock: u32,
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
}

struct State {
    turn: u32,
    spiral: HashMap<(i32, i32), Building>,
    x_bounds: (i32, i32),
    y_bounds: (i32, i32),
    ressources: Ressources,
    next_position: (i32, i32),
}

impl State {
    fn print_spiral(&self) -> String {
        let (x_min, x_max) = self.x_bounds;
        let (y_min, y_max) = self.y_bounds;
        let mut spiral_string = String::new();
        for y in ((y_min - 1)..=(y_max + 1)).rev() {
            let mut new_line = String::new();
            for x in (x_min - 1)..=(x_max + 1) {
                if (x, y) == self.next_position {
                    new_line.push('🟪');
                    continue;
                }
                let building = self.spiral.get(&(x, y));
                let new_char = match building {
                    Some(b) => b.convert_to_char(),
                    None => '⬛',
                };
                new_line.push(new_char);
                // // Add space between buidings?
                // if x != x_max + 1 {
                //     new_line.push(' ');
                // }
            }
            new_line.push('\n');
            spiral_string.push_str(&new_line);
        }
        spiral_string
    }

    fn print(&self) {
        println!("\n===========================");
        println!("=== SpiralCity - Turn {} ===", self.turn);
        println!("===========================\n");
        println!("Ressources");
        println!("----------");
        println!(
            "👥 Population: {} / {} (occupied/total)",
            self.ressources.occupied_people, self.ressources.total_people
        );
        println!("🪵 Wood: {}", self.ressources.wood);
        println!("🪨 Rock: {}", self.ressources.rock);
        println!("\nCity");
        println!("----");
        println!("{}", self.print_spiral());
    }
}

#[test]
fn run_test() {
    let mut example_spiral = HashMap::new();
    example_spiral.insert((0, 0), Building::House);
    example_spiral.insert((1, 0), Building::Forest);
    example_spiral.insert((1, -1), Building::House);
    example_spiral.insert((0, -1), Building::House);
    example_spiral.insert((-1, -1), Building::Quarry);
    example_spiral.insert((-1, 0), Building::Workshop);
    example_spiral.insert((-1, 1), Building::House);

    let example_ressources = Ressources {
        total_people: 4,
        occupied_people: 3,
        wood: 8,
        rock: 4,
    };
    let example = State {
        turn: 6,
        spiral: example_spiral,
        x_bounds: (-1, 1),
        y_bounds: (-1, 1),
        ressources: example_ressources,
        next_position: (0, 1),
    };
    let res = example.print_spiral();
    assert_eq!(
        res,
        String::from("⬛⬛⬛⬛⬛\n⬛🏠🟪⬛⬛\n⬛🪚🏠🌲⬛\n⬛🪨🏠🏠⬛\n⬛⬛⬛⬛⬛\n")
    );
}

fn main() {
    println!("--- Running first test for spiral printing ---");
    let mut example_spiral = HashMap::new();
    example_spiral.insert((0, 0), Building::House);
    example_spiral.insert((1, 0), Building::Forest);
    example_spiral.insert((1, -1), Building::House);
    example_spiral.insert((0, -1), Building::House);
    example_spiral.insert((-1, -1), Building::Quarry);
    example_spiral.insert((-1, 0), Building::Workshop);
    example_spiral.insert((-1, 1), Building::House);

    let example_ressources = Ressources {
        total_people: 4,
        occupied_people: 3,
        wood: 8,
        rock: 4,
    };
    let example = State {
        turn: 6,
        spiral: example_spiral,
        x_bounds: (-1, 1),
        y_bounds: (-1, 1),
        ressources: example_ressources,
        next_position: (0, 1),
    };
    example.print();
}
