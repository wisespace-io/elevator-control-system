pub use elevator::*;

pub trait ElevatorControlSystem {
    fn status(&self) -> Vec<Status>;

    fn update(&mut self, elevator_id: usize, floor: i32, goal_floor_number: i32);

    fn pickup(&mut self, floor: i32, direction: Direction);

    fn step(&mut self);
}

#[derive(Clone)]
pub struct ElevatorController {
    pub elevators: Vec<Elevator>,
}

impl ElevatorController {
    pub fn new(number_of_elevators: usize) -> Self {
        let mut elevators_bulk = vec![];
        for id in 0..number_of_elevators {
            elevators_bulk.push(Elevator::new(id, 0, Direction::UP));
        }

        ElevatorController {
            elevators: elevators_bulk,
        }
    }
}

impl ElevatorControlSystem for ElevatorController {
    fn status(&self) -> Vec<Status> {
        let mut statuses = vec![];
        for elevator in self.elevators.iter() {
            statuses.push(Status { elevator: elevator.id, floor: elevator.current_floor, direction: elevator.direction});
        }
        statuses
    }

    fn update(&mut self, elevator_id: usize, floor: i32, goal_floor_number: i32) {
        self.elevators[elevator_id].update(floor, goal_floor_number);
    }

    fn pickup(&mut self, floor: i32, direction: Direction) {
        let mut candidates = vec![];
        {
            let mut elevators_same_direction = vec![];
            for elevator in self.elevators.iter() {
                if elevator.direction == direction {
                    elevators_same_direction.push(elevator);
                }
            }

            for elevator in elevators_same_direction {
                let result = floor - elevator.current_floor;
                candidates.push((elevator.id, result.abs()));
            }
        }

        let result = candidates.iter().min_by_key(|&&(_, y)| { y });
        match result {
            None => println!("No elevators available"), // TODO: Request should be processed later.
            Some(value) => self.elevators[value.0].pickup(floor, direction)
        }
    }

    fn step(&mut self) {
        for id in 0..self.elevators.len() {
            self.elevators[id].step();
        }
    }
}