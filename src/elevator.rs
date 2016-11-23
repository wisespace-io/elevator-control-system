use std::cmp::Ordering;
use std::collections::BinaryHeap;

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum Direction {
    UP,
    DOWN
}

#[derive(Clone, Eq, PartialEq)]
pub struct Request {
    floor: i32,
    direction: i32,
}

// The priority queue depends on `Ord`.
// Explicitly implement the trait so the queue becomes a min-heap instead of a max-heap.
impl Ord for Request {
    fn cmp(&self, other: &Request) -> Ordering {
        other.direction.cmp(&self.direction)
    }
}

// `PartialOrd` needs to be implemented as well.
impl PartialOrd for Request {
    fn partial_cmp(&self, other: &Request) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

pub struct Status {
    pub elevator: usize,
    pub floor: i32,
    pub direction: Direction
}

#[derive(Clone)]
pub struct Elevator {
    pub id: usize,
    pub current_floor: i32,
    pub direction: Direction,
    pub destinations_queue: BinaryHeap<Request>
}

impl Elevator {
    pub fn new(id: usize, floor: i32, direction: Direction) -> Elevator {
        Elevator {
            id: id,
            current_floor: floor,
            direction: direction,
            destinations_queue: BinaryHeap::new()
        }
    }

    pub fn update(&mut self, floor: i32, goal_floor_number: i32) {
        self.current_floor = floor;
        self.direction = if goal_floor_number > self.current_floor { Direction::UP } else { Direction::DOWN };
        self.destinations_queue.push(Request {floor: goal_floor_number, direction: self.direction as i32});
    }

    pub fn pickup(&mut self, floor: i32, direction: Direction) {
        self.destinations_queue.push(Request {floor: floor, direction: direction as i32});
    }

    pub fn step(&mut self) {
        if self.destinations_queue.is_empty() {
            return
        }

        if self.current_floor == self.destinations_queue.peek().unwrap().floor {
            self.update_direction();
        } else {
            match self.direction {
                Direction::UP => { self.current_floor += 1 },
                Direction::DOWN => { self.current_floor -= 1 },
            }
        }
    }

    fn update_direction(&mut self) {
        self.destinations_queue.pop();
        if !self.destinations_queue.is_empty() {
            let request = self.destinations_queue.peek();
            if request.unwrap().floor > self.current_floor {
                self.direction = Direction::UP;
            } else {
                self.direction = Direction::DOWN;
            }
        }
    }
}