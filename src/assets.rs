use bevy::{platform::collections::HashMap, prelude::*};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Texture {
	Particle,
	Deleter,
	Pusher,
}

impl Texture {
	fn each() -> [Self; 3] {
		[Self::Particle, Self::Deleter, Self::Pusher]
	}
	fn get_filename(&self) -> &'static str {
		match self {
			Self::Particle => "particle.png",
			Self::Deleter => "deleter.png",
			Self::Pusher => "pusher.png",
		}
	}
}

#[derive(Resource, Default)]
pub struct TextureMap(pub HashMap<Texture, Handle<Image>>);

pub fn load_assets(asset_server: Res<AssetServer>, mut texture_map: ResMut<TextureMap>) {
	for texture in Texture::each() {
		let handle = asset_server.load(format!("textures/{}", texture.get_filename()));
		texture_map.0.insert(texture, handle);
	}
}
