mod extractor;
mod geom;
mod morton;
mod source;
mod topology;

pub use extractor::{Extractor, IndexedSeparateNormals, WithIndexedSeparateNormals};
pub use source::{FiniteDifference, HermiteSource, Source};

pub struct ExtractSurfaceError;

pub struct DualContouring<S> {
    source: S,
    max_res: u32,
    epsilon: f32,
}

impl<S> DualContouring<S> {
    pub fn new(source: S, max_res: u32, epsilon: f32) -> Self {
        assert!(
            max_res.is_power_of_two(),
            "`max_res` must be a power of two"
        );

        assert!(epsilon.is_finite(), "`epsilon` must be finite");
        assert!(epsilon > 0.0, "`epsilon` must be greater than 0");

        DualContouring {
            source,
            max_res,
            epsilon,
        }
    }
}

impl<S: HermiteSource> DualContouring<S> {
    pub fn extract(&self, _extractor: impl Extractor) -> Result<(), ExtractSurfaceError> {
        todo!()
    }
}
