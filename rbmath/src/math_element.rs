use roxmltree::Node;
use smallvec::SmallVec;

use crate::{
    constants::{NUMBER_SIGN, PLACEHOLDER},
    letters::Classifier,
    symbol_tables,
    traits::Renderer,
};
use brl::{braille, BraillePattern::*, BrailleString};

pub(crate) type ChildrenArray<'a> = [Box<MathElement<'a>>; 8];

#[derive(Copy, Clone, PartialEq, Eq)]
pub(crate) enum ElementType {
    Root,
    Number,
    Ident,
    Operator,
    Row,
    Sqrt,
    Radical,
    Fraction,
    Subscript,
    Superscript,
    SubSuperscript,
    Underscript,
    Overscript,
    UnderOverscript,
}

#[derive(Copy, Clone, PartialEq, Eq)]
pub (crate) enum SubscriptType {
    Sub,
    Sup,
    SubSup,
    Under,
    Over,
    UnderOver,
}

pub(crate) struct MathElement<'a> {
    element: ElementType,
    children: SmallVec<ChildrenArray<'a>>,
    text: &'a str,
}

impl<'a> MathElement<'a> {
    pub fn from_xml(node: Node<'a, '_>) -> Option<MathElement<'a>> {
        if !node.is_element() {
            panic!(
                "MathElement::from_xml was called on a non-element node. This should never happen!"
            );
        }
        let elem_type = match node.tag_name().name() {
            "math" => Some(ElementType::Root),
            "mrow" => Some(ElementType::Row),
            "mn" => Some(ElementType::Number),
            "mi" => Some(ElementType::Ident),
            "mo" => Some(ElementType::Operator),
            "mroot" => Some(ElementType::Radical),
            "msqrt" => Some(ElementType::Sqrt),
            "mfrac" => Some(ElementType::Fraction),
            "msub" => Some(ElementType::Subscript),
            "msup" => Some(ElementType::Superscript),
            "msubsup" => Some(ElementType::SubSuperscript),
            "munder" => Some(ElementType::Underscript),
            "mover" => Some(ElementType::Overscript),
            "munderover" => Some(ElementType::UnderOverscript),
            _ => None,
        };
        if let Some(elem_type) = elem_type {
            let children: SmallVec<[Box<MathElement<'a>>; 8]> = node
                .children()
                .filter_map(|c| {
                    if c.is_element() {
                        if let Some(child) = MathElement::from_xml(c) {
                            return Some(Box::new(child));
                        }
                    }
                    None
                })
                .collect();
            Some(MathElement {
                element: elem_type,
                children,
                text: node.text().unwrap_or(""),
            })
        } else {
            log::warn!("Unknown MathML tag {}", node.tag_name().name());
            None
        }
    }

    fn is_integer(&self) -> bool {
        match self.element {
            ElementType::Number => self.text.chars().all(|c| c.is_ascii_digit()),
            ElementType::Row if self.children.len() == 1 => self.children[0].is_integer(),
            ElementType::Row if self.children.len() == 2 => {
                if self.children[0].element == ElementType::Operator
                    && (self.children[0].text == "-" || self.children[0].text == "+")
                {
                    self.children[1].is_integer()
                } else {
                    false
                }
            }
            _ => false,
        }
    }

    fn is_operators(&self) -> bool {
        match self.element {
            ElementType::Operator => true,
            ElementType::Row => self.children.iter().all(|c| c.is_operators()),
            _ => false
        }
    }

    fn is_single_ident(&self) -> bool {
        match self.element {
            ElementType::Ident => true,
            ElementType::Row if self.children.len() == 1 => self.children[0].is_single_ident(),
            _ => false,
        }
    }
    pub fn render(&self, r: &mut impl crate::traits::Renderer) {
        use ElementType::*;
        use SubscriptType::*;
        match self.element {
            Root | Row => self.render_children(r),
            Number => self.render_number(r, false),
            Ident => self.render_ident(r),
            Operator => self.render_operator(r),
            Sqrt => self.render_radical(r, true),
            Radical => self.render_radical(r, false),
            Fraction => self.render_fraction(r),
            Subscript => self.render_subscript(r, Sub),
            Superscript => self.render_subscript(r, Sup),
            SubSuperscript => self.render_subscript(r, SubSup),
            Underscript => self.render_subscript(r, Under),
            Overscript => self.render_subscript(r, Over),
            UnderOverscript => self.render_subscript(r, UnderOver),
        }
    }

    fn render_children(&self, r: &mut impl Renderer) {
        for c in &self.children {
            c.render(r);
        }
    }

    fn render_number(&self, r: &mut impl Renderer, lower: bool) {
        let mut res = BrailleString::new();
        if !lower {
            res += NUMBER_SIGN;
        }
        for c in self.text.chars() {
            res += match c {
                '0' if lower => Braille356,
                '0' => Braille245,
                '1' if lower => Braille2,
                '1' => Braille1,
                '2' if lower => Braille23,
                '2' => Braille12,
                '3' if lower => Braille25,
                '3' => Braille14,
                '4' if lower => Braille256,
                '4' => Braille145,
                '5' if lower => Braille26,
                '5' => Braille15,
                '6' if lower => Braille235,
                '6' => Braille124,
                '7' if lower => Braille2356,
                '7' => Braille1245,
                '8' if lower => Braille236,
                '8' => Braille125,
                '9' if lower => Braille35,
                '9' => Braille24,
                ',' | '.' => Braille2,
                other => {
                    log::warn!(
                        "Trying to render non-numeric character {} as part of number.",
                        other
                    );
                    PLACEHOLDER
                }
            }
        }
        r.write_string(res, self.element);
    }

    fn render_ident(&self, r: &mut impl Renderer) {
        let mut res = BrailleString::new();
        if let Some(f) = symbol_tables::special_function(self.text) {
            res += &f;
            r.set_last_classifier(None);
        } else {
            for c in self.text.chars() {
                if let Some(l) = Classifier::classify(c) {
                    let dots = l.get_rendering();
                    if let Some(last_class) = r.get_last_classifier() {
                        if last_class != l.1 {
                            res += &l.1.get_prefix();
                        } else if let Some(last_type) = r.get_last_element_type() {
                            if last_type == ElementType::Number && !dots.has_lower_dots() {
                                res += &l.1.get_prefix();
                            }
                        }
                    } else {
                        res += &l.1.get_prefix();
                    }
                    res += dots;
                    r.set_last_classifier(Some(l.1));
                } else {
                    res += &symbol_tables::braille_symbol(c);
                }
            }
        }
        r.write_string(res, self.element);
    }

    fn render_operator(&self, r: &mut impl Renderer) {
        for c in self.text.chars() {
            if c == '+' || c == '-' || c == '±' || c == '∓' {
                if let Some(ElementType::Number)
                | Some(ElementType::Ident)
                | Some(ElementType::Fraction)
                | Some(ElementType::Sqrt)
                | Some(ElementType::Radical) = r.get_last_element_type()
                {
                    r.write_string(braille!(0), self.element);
                }
            }
            r.write_string(symbol_tables::braille_symbol(c), self.element);
        }
    }

    fn render_radical(&self, r: &mut impl Renderer, is_sqrt: bool) {
        r.write_string(braille!(146), self.element);
        if !is_sqrt {
            if self.children[1].is_integer() {
                self.children[1].render_number(r, true);
            } else {
                self.children[1].render(r);
            }
        }
        r.write_string(braille!(156), self.element);
        self.children[0].render(r);
        r.write_string(braille!(1456), self.element);
    }

    fn render_fraction(&self, r: &mut impl Renderer) {
        let num = &self.children[0];
        let denom = &self.children[1];
        if num.is_integer() && denom.is_integer() {
            num.render_number(r, false);
            denom.render_number(r, true);
        } else if (num.is_integer() || num.is_single_ident())
            && (denom.is_integer() || denom.is_single_ident())
        {
            num.render(r);
            r.write_string(braille!(1256), self.element);
            if denom.is_integer() {
                denom.render_number(r, true);
            } else {
                denom.render(r);
            }
        } else {
            r.write_string(braille!(23), ElementType::Operator);
            num.render(r);
            r.write_string(braille!(0, 1256), self.element);
            denom.render(r);
            r.write_string(braille!(56), self.element);
        }
    }

    fn render_in_script(
        &self,
        r: &mut impl Renderer,
        script_type: SubscriptType,
        parent: ElementType,
    ) {
        use SubscriptType::*;
        if self.is_operators() {
            self.render_embelishment(r, script_type, parent);
        }else {
        r.write_string(
            match script_type {
                Sub => braille!(16),
                Sup => braille!(34),
                SubSup => braille!(16),
                Under => braille!(46, 16),
                Over => braille!(46, 34),
                UnderOver => braille!(46, 16),
            },
            parent,
        );
        if self.is_integer() {
            self.render_number(r, true);
        } else {
            self.render(r);
            r.write_string(braille!(156), parent);
        }
    }
    }

    fn render_embelishment(&self, r: &mut impl Renderer, script_type: SubscriptType, parent: ElementType) {
        use SubscriptType::*;
        if let ElementType::Operator = self.element {
            if !symbol_tables::has_affinity(self.text.chars().next().unwrap_or('0'), script_type) {
                r.write_string(match script_type {
                    Under | UnderOver => braille!(56),
                    Sub | SubSup => braille!(456),
                    Over => braille!(45),
                    Sup => braille!(46)
                }, parent);
            }
            self.render(r);
        } else {
            for c in &self.children {
                c.render_embelishment(r, script_type, parent);
            }
        }
    }

    fn render_subscript(&self, r: &mut impl Renderer, subscript_type: SubscriptType) {
        use SubscriptType::*;
        self.children[0].render(r);
        self.children[1].render_in_script(r, subscript_type, self.element);
        if let SubSup = subscript_type {
            self.children[2].render_in_script(r, Sup, self.element);
        } else if let UnderOver = subscript_type {
            self.children[2].render_in_script(r, Over, self.element);
        }
    }
}
