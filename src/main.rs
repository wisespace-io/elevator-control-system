mod elevator;
mod elevatorcontrolsystem;

pub use elevatorcontrolsystem::*;

fn main() {
    from_ground_floor_to_third();
    from_third_to_first();
}

fn from_ground_floor_to_third() {
    println!("----> ground_floor_to_third");
    let mut controller = ElevatorController::new(4);
    controller.pickup(3, Direction::UP);

    // Going UP . . .
    controller.step();
    print_status(controller.status());
    controller.step();
    print_status(controller.status());
    controller.step();
    print_status(controller.status());
}

fn from_third_to_first() {
    println!("----> third_to_first");
    let mut controller = ElevatorController::new(4);

    controller.update(3, 4, 0); // Reset state as default is ALL going UP
    controller.pickup(1, Direction::DOWN);

    // Going Down . . .
    controller.step();
    print_status(controller.status());
    controller.step();
    print_status(controller.status());
    controller.step();
    print_status(controller.status());
}

fn print_status(statuses: Vec<Status>) {
    for status in statuses.iter() {
        println!("Elevator: {},  Floor: {}, Direction: {:?}", status.elevator, status.floor, status.direction);
    }
}
