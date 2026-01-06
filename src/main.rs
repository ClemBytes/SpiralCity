use std::{collections::HashMap, io};

use enum_derived::Rand;

#[derive(Debug, Clone)]
struct GlobalRessources {
    total_people: u32,
    occupied_people: u32,
    wood: u32,
    rock: u32,
}

#[derive(Debug, Clone, Copy, Rand, PartialEq)]
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

    fn get_name(&self) -> String {
        match *self {
            Building::House => "House".to_string(),
            Building::Forest => "Forest".to_string(),
            Building::Quarry => "Quarry".to_string(),
            Building::Workshop => "Workshop".to_string(),
        }
    }

    fn building_to_string(&self) -> String {
        let mut res = String::new();
        res.push(self.convert_to_char());
        res.push(' ');
        res.push_str(&self.get_name());
        res
    }
}

#[derive(Debug, Clone, Copy)]
enum Direction {
    Right,
    Left,
    Up,
    Down,
}

#[derive(Debug, Clone)]
struct State {
    turn: u32,
    spiral: HashMap<(i32, i32), Building>,
    x_bounds: (i32, i32),
    y_bounds: (i32, i32),
    ressources: GlobalRessources,
    current_position: (i32, i32),
    direction: Direction,
}

impl State {
    fn initialize() -> Self {
        let mut initial_spiral = HashMap::new();
        initial_spiral.insert((0, 0), Building::House);
        let initial_ressources = GlobalRessources {
            total_people: 1,
            occupied_people: 0,
            wood: 0,
            rock: 0,
        };
        Self {
            turn: 0,
            spiral: initial_spiral,
            x_bounds: (0, 0),
            y_bounds: (0, 0),
            ressources: initial_ressources,
            current_position: (0, 0),
            direction: Direction::Right,
        }
    }

    fn spiral_to_string(&self) -> String {
        let (x_min, x_max) = self.x_bounds;
        let (y_min, y_max) = self.y_bounds;
        let mut spiral_string = String::new();
        for y in ((y_min - 1)..=(y_max + 1)).rev() {
            let mut new_line = String::new();
            for x in (x_min - 1)..=(x_max + 1) {
                if (x, y) == self.get_next_position() {
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
        println!("{}", self.spiral_to_string());
    }

    fn get_next_position(&self) -> (i32, i32) {
        let (cx, cy) = self.current_position;
        match self.direction {
            Direction::Right => (cx + 1, cy),
            Direction::Left => (cx - 1, cy),
            Direction::Up => (cx, cy + 1),
            Direction::Down => (cx, cy - 1),
        }
    }
}

fn choose_building(turn: u32) -> Option<Building> {
    fn correct_selection(turn: u32, building1: Building, building2: Building) -> bool {
        if building1 == building2 {
            return false;
        }

        if turn == 0 && (building1 == Building::Workshop || building2 == Building::Workshop) {
            return false;
        }

        true
    }

    let mut building1 = Building::House;
    let mut building2 = Building::House;
    while !correct_selection(turn, building1, building2) {
        building1 = Building::rand();
        building2 = Building::rand();
    }

    println!("Choose building 1 or 2:");
    println!("1. {}", building1.building_to_string());
    println!("2. {}", building2.building_to_string());
    println!("Q. Quit\n");

    let mut buffer = String::new();
    loop {
        io::stdin()
            .read_line(&mut buffer)
            .expect("Expected first user input");
        match buffer.trim() {
            "1" => return Some(building1),
            "2" => return Some(building2),
            "Q" => return None,
            "q" => return None,
            _ => println!("Please enter a correct value: '1', '2' or 'Q'"),
        }
        buffer.clear();
    }
}

#[test]
fn test_spiral_print() {
    // Test spiral print
    let mut example_spiral = HashMap::new();
    example_spiral.insert((0, 0), Building::House);
    example_spiral.insert((1, 0), Building::Forest);
    example_spiral.insert((1, -1), Building::House);
    example_spiral.insert((0, -1), Building::House);
    example_spiral.insert((-1, -1), Building::Quarry);
    example_spiral.insert((-1, 0), Building::Workshop);
    example_spiral.insert((-1, 1), Building::House);

    let example_ressources = GlobalRessources {
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
        current_position: (-1, 1),
        direction: Direction::Right,
    };
    let res = example.spiral_to_string();
    assert_eq!(
        res,
        String::from("⬛⬛⬛⬛⬛\n⬛🏠🟪⬛⬛\n⬛🪚🏠🌲⬛\n⬛🪨🏠🏠⬛\n⬛⬛⬛⬛⬛\n")
    );
}

#[test]
fn test_get_next_position() {
    // Test get next position
    let mut example_coordinates = State::initialize();
    assert_eq!(example_coordinates.get_next_position(), (1, 0));
    example_coordinates.current_position = (1, 0);
    example_coordinates.direction = Direction::Down;
    assert_eq!(example_coordinates.get_next_position(), (1, -1));
    example_coordinates.current_position = (1, -1);
    example_coordinates.direction = Direction::Left;
    assert_eq!(example_coordinates.get_next_position(), (0, -1));
    example_coordinates.current_position = (0, -1);
    example_coordinates.direction = Direction::Left;
    assert_eq!(example_coordinates.get_next_position(), (-1, -1));
    example_coordinates.current_position = (-1, -1);
    example_coordinates.direction = Direction::Up;
    assert_eq!(example_coordinates.get_next_position(), (-1, 0));
    example_coordinates.current_position = (-1, 0);
    example_coordinates.direction = Direction::Up;
    assert_eq!(example_coordinates.get_next_position(), (-1, 1));
}

fn _spiral_printing_example() {
    println!("--- Trying spiral printing ---");
    let mut example_spiral = HashMap::new();
    example_spiral.insert((0, 0), Building::House);
    example_spiral.insert((1, 0), Building::Forest);
    example_spiral.insert((1, -1), Building::House);
    example_spiral.insert((0, -1), Building::House);
    example_spiral.insert((-1, -1), Building::Quarry);
    example_spiral.insert((-1, 0), Building::Workshop);
    example_spiral.insert((-1, 1), Building::House);

    let example_ressources = GlobalRessources {
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
        current_position: (-1, 1),
        direction: Direction::Right,
    };
    example.print();
}

fn _choose_buiding_example() {
    // Use turn=0 to check that we never have a workshop in this case
    let new_building = choose_building(2);
    println!("Chosen building: {new_building:?}");
}

fn main() {
    _choose_buiding_example();
}
