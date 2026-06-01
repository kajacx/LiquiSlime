use crate::{GameInteraction, InputQuery, TimeInterval};

pub trait BehaviourAdaptor {
    fn update(
        &mut self,
        interaction: &mut GameInteraction,
        input_query: &dyn InputQuery,
        delta_time: TimeInterval,
    );
}
