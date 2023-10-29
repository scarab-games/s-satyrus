use bevy::{
    ecs::query::WorldQuery,
    prelude::{Component, Query, Without},
};

/// Self and `Target` must be mutually exclusive in order to satisfy rust's borrow checker
pub trait InteractsWith<Target: Component>
where
    Self: Component + Sized,
{
    type SelfInfo: WorldQuery;
    type TargetInfo: WorldQuery;

    /// Called with all potential `Target`s and their corresponding `TargetInfo`.
    /// This should determine whether `target` can even be targeted by the interaction
    /// and then do whatever the interaction needs to do
    fn interact(
        &mut self,
        self_info: &mut <Self::SelfInfo as WorldQuery>::Item<'_>,
        target_query: &mut Query<(&mut Target, Self::TargetInfo), Without<Self>>,
    );
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
        interactor.interact(&mut interactor_info, &mut target_query);
    }
}
