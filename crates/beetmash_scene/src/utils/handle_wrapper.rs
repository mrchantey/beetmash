use bevy::prelude::*;



/// Temp workaround since handle:Component removed in 0.15
#[derive(Debug, Clone, Component, Reflect, Deref, DerefMut)]
#[reflect(Component)]
pub struct HandleWrapper<T: Asset>(pub Handle<T>);


impl<T: Asset> Into<AssetId<T>> for HandleWrapper<T> {
	fn into(self) -> AssetId<T> { self.id() }
}
impl<T: Asset> Into<AssetId<T>> for &HandleWrapper<T> {
	fn into(self) -> AssetId<T> { self.id() }
}
