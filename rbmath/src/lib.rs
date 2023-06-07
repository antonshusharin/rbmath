pub(crate) mod constants;
pub(crate) mod letters;
pub(crate) mod math_element;
pub(crate) mod renderers;
pub(crate) mod symbol_tables;
pub(crate) mod traits;

use brl::BrailleString;
use math_element::MathElement;
use renderers::BasicRenderer;
use traits::Renderer;

pub fn render(ml: &str) -> BrailleString {
    let ml: String = ml.chars().filter_map(symbol_tables::normalize).collect();
    let document = roxmltree::Document::parse(&ml).unwrap(); // TODO: Error type.
    let mut r = BasicRenderer::new();
    if let Some(first) = MathElement::from_xml(document.root_element()) {
        first.render(&mut r);
    }
    r.finish()
}
