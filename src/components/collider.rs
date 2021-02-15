use crate::components::Goal;
#[derive(Debug)]
pub enum Collider {
    Paddle,
    Wall,
    Goal(Goal),
}
