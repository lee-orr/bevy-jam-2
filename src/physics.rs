use heron::PhysicsLayer;

#[derive(PhysicsLayer)]
pub enum GameCollisionLayers {
    World,
    Player,
    Spirit
}