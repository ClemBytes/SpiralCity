#[derive(Debug, Clone)]
pub struct GlobalResources {
    pub total_people: u32,
    pub occupied_people: u32,
    pub wood: u32,
    pub rock: u32,
}

impl GlobalResources {
    pub fn initialize() -> Self {
        Self {
            total_people: 1,
            occupied_people: 0,
            wood: 0,
            rock: 0,
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub enum Resources {
    WorkingPeople(i32),
    Wood(i32),
    Rock(i32),
}

impl Resources {
    fn resource_to_symbol(&self) -> char {
        match *self {
            Resources::WorkingPeople(_) => '👥',
            Resources::Wood(_) => '🪵',
            Resources::Rock(_) => '🪨',
        }
    }

    pub fn delta_to_string(delta_production: &[Self]) -> String {
        let mut res = String::new();
        if delta_production.is_empty() {
            return String::from("no changes");
        }

        for (i, resource) in delta_production.iter().enumerate() {
            let diff = match *resource {
                Resources::WorkingPeople(n) => n,
                Resources::Wood(n) => n,
                Resources::Rock(n) => n,
            };
            if diff == 0 {
                unreachable!("diff of resource {resource:?} should not be 0!");
            }
            res.push(resource.resource_to_symbol());
            res.push(' ');

            if diff > 0 {
                res.push('+');
            }
            res.push_str(format!("{diff}").as_str());
            res.push(' ');

            if diff > 0 {
                res.push_str("⬆️");
            } else {
                res.push('🔽');
            }

            if i != delta_production.len() - 1 {
                res.push_str(" | ");
            }
        }
        res
    }
}
