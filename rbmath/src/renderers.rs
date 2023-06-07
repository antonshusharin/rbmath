use crate::{letters::Classifier, math_element::ElementType, traits::Renderer};
use brl::BrailleString;

pub(crate) struct BasicRenderer {
    last_elem_type: Option<ElementType>,
    last_classifier: Option<Classifier>,
    res: BrailleString,
}

impl BasicRenderer {
    pub fn new() -> BasicRenderer {
        BasicRenderer {
            last_elem_type: None,
            last_classifier: None,
            res: BrailleString::new(),
        }
    }
}

impl Renderer for BasicRenderer {
    fn write_string(&mut self, s: BrailleString, source_type: ElementType) {
        self.res += &s;
        self.last_elem_type = Some(source_type);
    }

    fn finish(self) -> BrailleString {
        self.res
    }

    fn get_last_classifier(&self) -> Option<crate::letters::Classifier> {
        self.last_classifier
    }

    fn set_last_classifier(&mut self, cls: Option<crate::letters::Classifier>) {
        self.last_classifier = cls;
    }

    fn get_last_element_type(&self) -> Option<ElementType> {
        self.last_elem_type
    }
}
