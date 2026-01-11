use std::{collections::HashMap, io};

use enum_derived::Rand;

enum Resources {
    WorkingPeople(u32),
    Wood(u32),
    Rock(u32),
}

#[derive(Debug, Clone)]
struct GlobalResources {
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

    fn cost(&self) -> Vec<Resources> {
        match *self {
            Building::House => vec![Resources::Wood(1)],
            Building::Forest => vec![Resources::WorkingPeople(1)],
            Building::Quarry => vec![Resources::WorkingPeople(1)],
            Building::Workshop => vec![
                Resources::WorkingPeople(2),
                Resources::Wood(1),
                Resources::Rock(1),
            ],
        }
    }

    fn characteristics_to_string(&self) -> String {
        let mut res = String::new();
        match *self {
            Building::House => {
                res.push_str("    Cost           : -1 wood 🪵\n");
                res.push_str("    Production     : +1 people 👥 (once when built)\n");
                res.push_str("    Special effect : none\n");
            }
            Building::Forest => {
                res.push_str("    Cost           : 1 working people 👥\n");
                res.push_str("    Production     : +2 wood 🪵 / turn\n");
                res.push_str("    Special effect : -1 wood 🪵 if next to a quarry 🪨\n");
            }
            Building::Quarry => {
                res.push_str("    Cost           : 1 working people 👥\n");
                res.push_str("    Production     : +rocks 🪨 / turn\n");
                res.push_str("    Special effect : -1 rock 🪨 if next to a forest 🌲\n");
            }
            Building::Workshop => {
                res.push_str(
                    "    Cost           : 2 working people 👥 | -1 wood 🪵 | -1 rock 🪨\n",
                );
                res.push_str("    Production     : none\n");
                res.push_str("    Special effect : adjacent buildings produce +1 resource\n");
            }
        }
        res
    }

    fn can_be_built(&self, state: &State) -> bool {
        let cost = self.cost();
        for resource in cost {
            match resource {
                Resources::WorkingPeople(n) => {
                    if state.owned_resources.total_people - state.owned_resources.occupied_people
                        < n
                    {
                        return false;
                    }
                }
                Resources::Wood(n) => {
                    if state.owned_resources.wood < n {
                        return false;
                    }
                }
                Resources::Rock(n) => {
                    if state.owned_resources.rock < n {
                        return false;
                    }
                }
            }
        }
        true
    }

    fn production(&self, coordinates: (i32, i32), state: &State) -> Option<Resources> {
        let mut neighbours = vec![];
        let (x, y) = coordinates;
        for (nx, ny) in [(x - 1, y), (x + 1, y), (x, y - 1), (x, y + 1)] {
            if state.spiral.contains_key(&(nx, ny)) {
                neighbours.push(*state.spiral.get(&(nx, ny)).unwrap());
            }
        }
        match *self {
            Building::Forest => {
                let mut nb_wood = 2;
                for neighbour in neighbours {
                    match neighbour {
                        Building::Quarry => nb_wood -= 1,
                        Building::Workshop => nb_wood += 1,
                        _ => {}
                    }
                }
                Some(Resources::Wood(nb_wood))
            }
            Building::Quarry => {
                let mut nb_rock = 2;
                for neighbour in neighbours {
                    match neighbour {
                        Building::Forest => nb_rock -= 1,
                        Building::Workshop => nb_rock += 1,
                        _ => {}
                    }
                }
                Some(Resources::Rock(nb_rock))
            }
            _ => None,
        }
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
    owned_resources: GlobalResources,
    current_position: (i32, i32),
    direction: Direction,
}

impl State {
    fn initialize() -> Self {
        let mut initial_spiral = HashMap::new();
        initial_spiral.insert((0, 0), Building::House);
        let initial_resources = GlobalResources {
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
            owned_resources: initial_resources,
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
                    if x < self.x_bounds.0 {
                        // ↑o
                        // x←
                        new_line.push('⮤');
                    } else if x > self.x_bounds.1 {
                        // →x
                        // o↓
                        new_line.push('⮧');
                    } else if y < self.y_bounds.0 {
                        // o↓
                        // ←x
                        new_line.push('⮠');
                    } else if y > self.y_bounds.1 {
                        // x→
                        // ↑o
                        new_line.push('⮣');
                    } else {
                        match self.direction {
                            Direction::Down => new_line.push('🡻'),
                            Direction::Up => new_line.push('🡹'),
                            Direction::Left => new_line.push('🡸'),
                            Direction::Right => new_line.push('🡺'),
                        }
                    }
                    new_line.push(' ');
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
        println!("Resources");
        println!("----------");
        println!(
            "👥 Population : {} / {} (occupied/total)",
            self.owned_resources.occupied_people, self.owned_resources.total_people
        );
        println!("🪵 Wood       : {}", self.owned_resources.wood);
        println!("🪨 Rock       : {}", self.owned_resources.rock);
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

    fn choose_building(&self) -> Option<Building> {
        fn correct_proposition(turn: u32, building1: Building, building2: Building) -> bool {
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
        while !correct_proposition(self.turn, building1, building2) {
            building1 = Building::rand();
            building2 = Building::rand();
        }

        println!("[1] {}", building1.building_to_string());
        println!("{}", building1.characteristics_to_string());
        println!("[2] {}", building2.building_to_string());
        println!("{}", building2.characteristics_to_string());

        if !building1.can_be_built(self) && !building2.can_be_built(self) {
            println!("You cannot build any of the buildings, you loose!");
            return None;
        }

        println!("> Choose building 1 or 2 (Q to quit):");

        let mut buffer = String::new();
        loop {
            io::stdin()
                .read_line(&mut buffer)
                .expect("Expected first user input");
            match buffer.trim() {
                "1" => {
                    if !building1.can_be_built(self) {
                        println!(
                            "You cannot build {}, choose another building!",
                            building1.building_to_string()
                        )
                    } else {
                        return Some(building1);
                    }
                }
                "2" => {
                    if !building2.can_be_built(self) {
                        println!(
                            "You cannot build {}, choose another building!",
                            building2.building_to_string()
                        )
                    } else {
                        return Some(building2);
                    }
                }
                "Q" => return None,
                "q" => return None,
                _ => println!("Please enter a correct value: '1', '2' or 'Q'"),
            }
            buffer.clear();
        }
    }

    fn turn(&self) -> Option<Self> {
        self.print();
        let mut new_state = self.clone();
        new_state.turn += 1;
        let new_building = self.choose_building()?;

        // Pay cost
        let cost = new_building.cost();
        for resource in cost {
            match resource {
                Resources::WorkingPeople(n) => new_state.owned_resources.occupied_people += n,
                Resources::Wood(n) => new_state.owned_resources.wood -= n,
                Resources::Rock(n) => new_state.owned_resources.rock -= n,
            }
        }

        // Update map (coordinates, direction, bounds)
        let new_coordinates = self.get_next_position();
        new_state.spiral.insert(new_coordinates, new_building);
        let (nx, ny) = self.get_next_position();
        if nx < self.x_bounds.0 {
            // ↑o
            // x←
            new_state.x_bounds.0 = nx;
            new_state.direction = Direction::Up;
        } else if nx > self.x_bounds.1 {
            // →x
            // o↓
            new_state.x_bounds.1 = nx;
            new_state.direction = Direction::Down;
        } else if ny < self.y_bounds.0 {
            // o↓
            // ←x
            new_state.y_bounds.0 = ny;
            new_state.direction = Direction::Left;
        } else if ny > self.y_bounds.1 {
            // x→
            // ↑o
            new_state.y_bounds.1 = ny;
            new_state.direction = Direction::Right;
        }
        new_state.current_position = (nx, ny);

        // Apply effects and update resources
        let mut nb_people = 0;
        for (&coordinates, &building) in &new_state.spiral {
            // Each house adds a person
            if building == Building::House {
                nb_people += 1;
                continue;
            }

            // Update other resources
            let option_produced_resources = building.production(coordinates, &new_state);
            if let Some(produced_resources) = option_produced_resources {
                match produced_resources {
                    Resources::Wood(n) => new_state.owned_resources.wood += n,
                    Resources::Rock(n) => new_state.owned_resources.rock += n,
                    _ => {}
                }
            }
        }
        new_state.owned_resources.total_people = nb_people;

        Some(new_state)
    }
}

fn play() {
    print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
    println!("---------------------------");
    println!("Welcome to 🌀 SpiralCity 🌀");
    println!("---------------------------");
    println!(
        "Your goal is to go as far as possible in the spiral, by choosing the good next building."
    );
    println!("You loose if you cannot build any of the 2 proposed buildings.");
    println!("Have fun!");
    let mut state = State::initialize();
    loop {
        let option_state = state.turn();
        if let Some(st) = option_state {
            state = st;
            print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
        } else {
            println!("Thanks for playing!");
            break;
        }
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

    let example_resources = GlobalResources {
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
        owned_resources: example_resources,
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

    let example_resources = GlobalResources {
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
        owned_resources: example_resources,
        current_position: (-1, 1),
        direction: Direction::Right,
    };
    example.print();
}

fn _choose_buiding_example() {
    // Use turn=0 to check that we never have a workshop in this case
    let mut example_spiral = HashMap::new();
    example_spiral.insert((0, 0), Building::House);
    example_spiral.insert((1, 0), Building::Forest);
    example_spiral.insert((1, -1), Building::House);
    example_spiral.insert((0, -1), Building::House);
    example_spiral.insert((-1, -1), Building::Quarry);
    example_spiral.insert((-1, 0), Building::Workshop);
    example_spiral.insert((-1, 1), Building::House);

    let example_resources = GlobalResources {
        total_people: 4,
        occupied_people: 4,
        wood: 8,
        rock: 4,
    };
    let example = State {
        turn: 6,
        spiral: example_spiral,
        x_bounds: (-1, 1),
        y_bounds: (-1, 1),
        owned_resources: example_resources,
        current_position: (-1, 1),
        direction: Direction::Right,
    };

    example.print();
    let new_building = example.choose_building();
    println!("Chosen building: {new_building:?}");
}

fn main() {
    play();
}
