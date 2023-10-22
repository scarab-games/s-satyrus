use bevy::prelude::{Component, Query, With, Without};

pub trait Agent<'a>: Component
where
    Self: Sized,
{
    type Controller: Component;
    type AgentInfo: Component;
    type TargetInfo: Component;
    type TargetMarker: Component;

    fn act(
        &mut self,
        controller: &mut Self::Controller,
        this: &Self::AgentInfo,
        target: Option<&Self::TargetInfo>,
    );

    fn get_target(
        &self,
        self_info: &Self::AgentInfo,
        controller: &Self::Controller,
        query: &Query<&Self::TargetInfo, (Without<Self>, With<Self::TargetMarker>)>,
    ) -> Option<Self::TargetInfo>;
}

pub fn agent_system<'a, A>(
    mut agent_query: Query<(&mut A, &mut A::Controller, &A::AgentInfo), Without<A::TargetMarker>>,
    target_query: Query<&A::TargetInfo, (Without<A>, With<A::TargetMarker>)>,
) where
    A: Agent<'a>,
{
    for (mut agent, mut controller, agent_info) in agent_query.iter_mut() {
        let target = agent.get_target(agent_info, &controller, &target_query);
        agent.act(&mut controller, agent_info, target.as_ref())
    }
}
