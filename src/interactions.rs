use bevy::{
    ecs::query::WorldQuery,
    prelude::{Component, Query, Without},
};

/// Self and `Target` must be mutually exclusive in order to satisfy rust's borrow checker
pub trait InteractsWith<Target: Component>: Component {
    type SelfInfo: WorldQuery;
    type TargetInfo: WorldQuery;
    // where
    //     &'a mut Self::SelfInfo: WorldQuery<Item<'a> = Self::SelfInfo>;

    /// Whether or not this component should even try to interact with any of its targets
    fn should_interact(&self, self_info: &<Self::SelfInfo as WorldQuery>::Item<'_>) -> bool;

    /// The number of potential targets to a single interaction.
    /// i.e. How many unique targets `do_interaction` can return true for before skipping more
    fn targets_per_interact(&self, self_info: &<Self::SelfInfo as WorldQuery>::Item<'_>) -> usize;

    /// Called with all potential `Target`s and their corresponding `TargetInfo`.
    /// This should determine whether `target` can even be targeted by the interaction
    /// and then do whatever the interaction needs to do
    ///
    /// Returns whether the interaction happened
    fn do_interaction(
        &mut self,
        self_info: &mut <Self::SelfInfo as WorldQuery>::Item<'_>,
        target: &mut Target,
        target_info: &mut <Self::TargetInfo as WorldQuery>::Item<'_>,
    ) -> bool;
}

pub fn interaction_system<'a, 'b, A, I, T, S>(
    mut interactor_query: Query<(&mut A, I), Without<T>>,
    mut target_query: Query<(&mut T, S), Without<A>>,
) where
    A: InteractsWith<T, SelfInfo = I, TargetInfo = S>,
    I: WorldQuery,
    S: WorldQuery,
    T: Component,
{
    for (mut interactor, mut interactor_info) in &mut interactor_query {
        if interactor.should_interact(&interactor_info) {
            let mut interacts_left = interactor.targets_per_interact(&interactor_info);
            for (mut target, mut target_info) in &mut target_query {
                if interacts_left == 0 {
                    break;
                }
                let sub = if interactor.do_interaction(
                    &mut interactor_info,
                    &mut target,
                    &mut target_info,
                ) {
                    1
                } else {
                    0
                };
                interacts_left -= sub;
            }
        }
    }
}
