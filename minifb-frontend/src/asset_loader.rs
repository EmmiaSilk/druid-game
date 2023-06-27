use async_trait::async_trait;
use druid_game::render::Bitmap;
use druid_game::service::asset_loader::LoadError;
use druid_game::service::asset_loader::AssetLoader;
use image::io::Reader;
use vfc::Rgb;


pub struct LocalAssetLoader {}

impl LocalAssetLoader {
    pub fn create() -> LocalAssetLoader {
        LocalAssetLoader {}
    }
}

#[async_trait(?Send)]
impl AssetLoader for LocalAssetLoader {
    async fn load_bitmap(&mut self, path: &str) -> Result<Bitmap, LoadError> {
        //TODO Returns too happily? Does this need to be or rely on a future? 
        
        let reader = match Reader::open(path) {
            Err(error) => return Err(LoadError::OtherError(error.to_string())),
            Ok(reader) => reader,
        };

        let image = match reader.decode() {
            Err(error) => return Err(LoadError::OtherError(error.to_string())),
            Ok(image) => image,
        };

        let image = image.into_rgba8();
        let (width, height) = image.dimensions();

        let mut colors = Vec::with_capacity((width*height) as usize);
        for (_x, _y, pixel) in image.enumerate_pixels() {
            let [r, g, b, _a] = pixel.0;
            let color = Rgb::new(r, g, b);

            colors.push(color);
        }

        Ok(Bitmap::new(width as usize, height as usize, colors))
    }
}