use std::{collections::HashMap, io};

use enum_derived::Rand;

use crate::buildings::Building;
use crate::resources::{GlobalResources, Resources};

#[derive(Debug, Clone, Copy)]
pub enum Direction {
    Right,
    Left,
    Up,
    Down,
}

#[derive(Debug, Clone)]
pub struct State {
    pub turn: u32,
    pub spiral: HashMap<(i32, i32), Building>,
    pub x_bounds: (i32, i32),
    pub y_bounds: (i32, i32),
    pub owned_resources: GlobalResources,
    pub current_position: (i32, i32),
    pub direction: Direction,
    pub delta_production: Vec<Resources>,
}

impl State {
    pub fn initialize() -> Self {
        let mut initial_spiral = HashMap::new();
        initial_spiral.insert((0, 0), Building::House);
        let initial_resources = GlobalResources::initialize();
        Self {
            turn: 0,
            spiral: initial_spiral,
            x_bounds: (0, 0),
            y_bounds: (0, 0),
            owned_resources: initial_resources,
            current_position: (0, 0),
            direction: Direction::Right,
            delta_production: vec![],
        }
    }

    pub fn spiral_to_string(&self) -> String {
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

    pub fn print(&self) {
        println!("\n===========================");
        println!("=== SpiralCity - Turn {} ===", self.turn);
        println!("===========================\n");
        if self.turn != 0 {
            println!(
                "Last turn : {}\n",
                Resources::delta_to_string(&self.delta_production)
            );
        }

        // TODO: move to resources crate
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

    pub fn get_next_position(&self) -> (i32, i32) {
        let (cx, cy) = self.current_position;
        match self.direction {
            Direction::Right => (cx + 1, cy),
            Direction::Left => (cx - 1, cy),
            Direction::Up => (cx, cy + 1),
            Direction::Down => (cx, cy - 1),
        }
    }

    pub fn choose_building(&self) -> Option<Building> {
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

    pub fn turn(&self) -> Option<Self> {
        self.print();
        let mut new_state = self.clone();
        new_state.turn += 1;
        let new_building = self.choose_building()?;

        // Pay cost
        let cost = new_building.cost();
        for resource in cost {
            match resource {
                Resources::WorkingPeople(n) => {
                    new_state.owned_resources.occupied_people += n as u32
                }
                Resources::Wood(n) => new_state.owned_resources.wood -= n as u32,
                Resources::Rock(n) => new_state.owned_resources.rock -= n as u32,
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
                    Resources::Wood(n) => new_state.owned_resources.wood += n as u32,
                    Resources::Rock(n) => new_state.owned_resources.rock += n as u32,
                    _ => {}
                }
            }
        }
        new_state.owned_resources.total_people = nb_people;

        // Update delta_production
        new_state.delta_production.clear();
        if new_building == Building::House {
            new_state.delta_production.push(Resources::WorkingPeople(1));
        }
        if new_state.owned_resources.wood != self.owned_resources.wood {
            let diff = new_state.owned_resources.wood as i32 - self.owned_resources.wood as i32;
            new_state.delta_production.push(Resources::Wood(diff));
        }
        if new_state.owned_resources.rock != self.owned_resources.rock {
            let diff = new_state.owned_resources.rock as i32 - self.owned_resources.rock as i32;
            new_state.delta_production.push(Resources::Rock(diff));
        }

        Some(new_state)
    }
}
