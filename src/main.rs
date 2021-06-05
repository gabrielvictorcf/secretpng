mod chunk_type;
mod chunk;
mod png;
mod secretpng;

pub type Error = Box<dyn std::error::Error>;
pub type Result<T> = std::result::Result<T, Error>;

use secretpng::secretpng;

fn main() {
    secretpng()
}