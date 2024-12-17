pub mod behaviors;
pub mod bindings;

use behaviors::Behaviors;

#[arbiter_macros::main(
  name = "Limestone",
  about = "Limestone lending and workers."
  behaviors = Behaviors
)]
pub async fn main() {}
