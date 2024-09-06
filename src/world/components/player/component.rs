use crate::world::player::player::Player;

use super::health::PlayerHealthComponent;

pub struct PlayerComponent {}

impl PlayerComponent {
  pub fn get_health(player: &Player) -> PlayerHealthComponent {
    return PlayerHealthComponent::new(player.env, player.get_component("minecraft:health"));
  }
}