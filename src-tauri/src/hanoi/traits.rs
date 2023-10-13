use anyhow::Result;
use opencv::prelude::*;

pub trait Renderable {
    fn render(&self, mat: &mut Mat) -> Result<()>;
}