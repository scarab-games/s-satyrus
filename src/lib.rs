/// Things that may change their behavior depending on the presence of a target
pub mod agent;
/// Things that may mututally mutably interact
/// i.e. if a player attacks an enemy the enemy might lose health and the player could gain xp
pub mod interactions;
