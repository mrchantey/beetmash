use bevy::prelude::*;



/// Temp workaround since handle:Component removed in 0.15
#[derive(Debug, Clone, Component, Reflect, Deref)]
#[reflect(Component)]
pub struct HandleWrapper<T: Asset>(pub Handle<T>);
