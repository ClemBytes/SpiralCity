use std::collections::HashMap;

use crate::buildings::Building;
pub mod buildings;

use crate::resources::GlobalResources;
pub mod resources;

use crate::states::State;
pub mod states;

use crate::states::Direction;

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
        delta_production: vec![],
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
        delta_production: vec![],
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
        delta_production: vec![],
    };

    example.print();
    let new_building = example.choose_building();
    println!("Chosen building: {new_building:?}");
}

fn main() {
    play();
}
