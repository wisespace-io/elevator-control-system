# Elevator control system

Rust implementation to a basic elevator control system. It is a sample application to [Rust Meetup](https://www.meetup.com/Rust-Oslo/)

The elevator control system should be able to handle a few elevators — up to 16.

Control system provides an interface for:

* Querying the state of the elevators (what floor are they on and where they are going),
* receiving an update about the status of an elevator,
* receiving a pickup request,
* time-stepping the simulation.