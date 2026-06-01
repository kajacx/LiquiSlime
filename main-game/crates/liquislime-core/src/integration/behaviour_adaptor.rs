use crate::{GameInteraction, TimeInterval};

pub trait BehaviourAdaptor {
    fn update(&mut self, interaction: &mut GameInteraction, delta_time: TimeInterval);
}
