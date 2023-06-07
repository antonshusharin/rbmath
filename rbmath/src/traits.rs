use crate::{letters::Classifier, math_element::ElementType};
use brl::BrailleString;

pub(crate) trait Renderer {
    fn write_string(&mut self, s: BrailleString, source_type: ElementType);
    fn get_last_element_type(&self) -> Option<ElementType>;
    fn get_last_classifier(&self) -> Option<Classifier>;
    fn set_last_classifier(&mut self, cls: Option<Classifier>);
    fn finish(self) -> BrailleString;
}
