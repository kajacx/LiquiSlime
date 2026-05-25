use super::*;

pub struct GameState {
    pub factions: Vec<Faction>,
    pub grids: SlimeGrids,
}

impl GameState {
    pub fn new(factions: Vec<Faction>, width: usize, height: usize) -> Self {
        Self {
            grids: SlimeGrids::new(factions.len(), width, height),
            factions,
        }
    }

    pub fn update(&mut self, time_passed: TimeInterval) {
        self.grids.prepare_slime_spread(time_passed);
        self.grids.spread_slime();
        self.grids.annihilate_slime();
    }
}
