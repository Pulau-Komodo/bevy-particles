use bevy::{platform::collections::HashMap, prelude::*};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Texture {
	Deleter,
	Arrow,
}

impl Texture {
	fn get_filename(&self) -> &'static str {
		match self {
			Self::Deleter => "deleter.png",
			Self::Arrow => "arrow.png",
		}
	}
}

#[derive(Resource, Default)]
pub struct TextureMap(pub HashMap<Texture, Handle<Image>>);

pub fn load_assets(asset_server: Res<AssetServer>, mut texture_map: ResMut<TextureMap>) {
	for texture in [Texture::Deleter, Texture::Arrow] {
		let handle = asset_server.load(format!("textures/{}", texture.get_filename()));
		texture_map.0.insert(texture, handle);
	}
}
