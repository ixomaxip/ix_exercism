// This stub file contains items which aren't used yet; feel free to remove this module attribute
// to enable stricter warnings.
#![allow(unused)]

pub struct Player {
    pub health: u32,
    pub mana: Option<u32>,
    pub level: u32,
}

impl Player {
    pub fn revive(&self) -> Option<Player> {
        self.is_dead().and_then(|p| Some(self.update_mana(p)))
    }

    fn is_dead(&self) -> Option<Player> {
        match self {
            Player {health: 0, ..} => Some(Player {health: 100, ..*self}),
            _ => None
        }
    }

    fn update_mana(&self, player: Player) -> Player {
        match player.level {
            l if l >= 10 => Player {mana: Some(100), ..player},
            _ => player
        }
    }


    pub fn cast_spell(&mut self, mana_cost: u32) -> u32 {
        match self.mana {
            None => {self.health = if mana_cost > self.health  { 0 } else { self.health - mana_cost}; 0},
            Some(mana) if mana < mana_cost => 0,
            Some(mana) => {self.mana = Some(mana - mana_cost); 2 * mana_cost}
        }
    }
}
