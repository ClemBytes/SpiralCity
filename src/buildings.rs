use enum_derived::Rand;

use crate::resources::Resources;

#[derive(Debug, Clone, Copy, Rand, PartialEq)]
pub enum Building {
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
                        < n as u32
                    {
                        return false;
                    }
                }
                Resources::Wood(n) => {
                    if state.owned_resources.wood < n as u32 {
                        return false;
                    }
                }
                Resources::Rock(n) => {
                    if state.owned_resources.rock < n as u32 {
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
