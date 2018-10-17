/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

use cssparser::RGBA;
use dom::bindings::codegen::Bindings::HTMLTableCellElementBinding;
use dom::bindings::codegen::Bindings::HTMLTableCellElementBinding::HTMLTableCellElementMethods;
use dom::bindings::codegen::Bindings::NodeBinding::NodeMethods;
use dom::bindings::inheritance::Castable;
use dom::bindings::root::DomRoot;
use dom::bindings::root::LayoutDom;
use dom::bindings::str::DOMString;
use dom::document::Document;
use dom::element::{Element, RawLayoutElementHelpers};
use dom::htmlelement::HTMLElement;
use dom::htmltablerowelement::HTMLTableRowElement;
use dom::node::Node;
use dom::virtualmethods::VirtualMethods;
use dom_struct::dom_struct;
use html5ever::{LocalName, Prefix};
use style::attr::{AttrValue, LengthOrPercentageOrAuto};
use style::context::QuirksMode;

const DEFAULT_COLSPAN: u32 = 1;
const DEFAULT_ROWSPAN: u32 = 1;

#[dom_struct]
pub struct HTMLTableCellElement {
    htmlelement: HTMLElement,
}

impl HTMLTableCellElement {
    fn new_inherited(
        local_name: LocalName,
        prefix: Option<Prefix>,
        document: &Document,
    ) -> HTMLTableCellElement {
        HTMLTableCellElement {
            htmlelement: HTMLElement::new_inherited(local_name, prefix, document),
        }
    }

    #[allow(unrooted_must_root)]
    pub fn new(
        local_name: LocalName,
        prefix: Option<Prefix>,
        document: &Document,
    ) -> DomRoot<HTMLTableCellElement> {
        Node::reflect_node(
            Box::new(HTMLTableCellElement::new_inherited(
                local_name, prefix, document,
            )),
            document,
            HTMLTableCellElementBinding::Wrap,
        )
    }
}

impl HTMLTableCellElementMethods for HTMLTableCellElement {
    // https://html.spec.whatwg.org/multipage/#dom-tdth-colspan
    make_uint_getter!(ColSpan, "colspan", DEFAULT_COLSPAN);

    // https://html.spec.whatwg.org/multipage/#dom-tdth-colspan
    make_uint_setter!(SetColSpan, "colspan", DEFAULT_COLSPAN);

    // https://html.spec.whatwg.org/multipage/#dom-tdth-rowspan
    make_uint_getter!(RowSpan, "rowspan", DEFAULT_ROWSPAN);

    // https://html.spec.whatwg.org/multipage/#dom-tdth-rowspan
    make_uint_setter!(SetRowSpan, "rowspan", DEFAULT_ROWSPAN);

    // https://html.spec.whatwg.org/multipage/#dom-tdth-bgcolor
    make_getter!(BgColor, "bgcolor");

    // https://html.spec.whatwg.org/multipage/#dom-tdth-bgcolor
    make_legacy_color_setter!(SetBgColor, "bgcolor");

    // https://html.spec.whatwg.org/multipage/#dom-tdth-width
    make_getter!(Width, "width");

    // https://html.spec.whatwg.org/multipage/#dom-tdth-width
    make_nonzero_dimension_setter!(SetWidth, "width");

    // https://html.spec.whatwg.org/multipage/#dom-tdth-cellindex
    fn CellIndex(&self) -> i32 {
        let self_node = self.upcast::<Node>();

        let parent_children = match self_node.GetParentNode() {
            Some(ref parent_node) if parent_node.is::<HTMLTableRowElement>() => {
                parent_node.children()
            },
            _ => return -1,
        };

        parent_children
            .filter(|c| c.is::<HTMLTableCellElement>())
            .position(|c| &*c == self_node)
            .map_or(-1, |p| p as i32)
    }
}

pub trait HTMLTableCellElementLayoutHelpers {
    fn get_background_color(&self) -> Option<RGBA>;
    fn get_colspan(&self) -> Option<u32>;
    fn get_rowspan(&self) -> Option<u32>;
    fn get_width(&self) -> LengthOrPercentageOrAuto;
}

#[allow(unsafe_code)]
impl HTMLTableCellElementLayoutHelpers for LayoutDom<HTMLTableCellElement> {
    fn get_background_color(&self) -> Option<RGBA> {
        unsafe {
            (&*self.upcast::<Element>().unsafe_get())
                .get_attr_for_layout(&ns!(), &local_name!("bgcolor"))
                .and_then(AttrValue::as_color)
                .cloned()
        }
    }

    fn get_colspan(&self) -> Option<u32> {
        unsafe {
            (&*self.upcast::<Element>().unsafe_get())
                .get_attr_for_layout(&ns!(), &local_name!("colspan"))
                .map(AttrValue::as_uint)
        }
    }

    fn get_rowspan(&self) -> Option<u32> {
        unsafe {
            (&*self.upcast::<Element>().unsafe_get())
                .get_attr_for_layout(&ns!(), &local_name!("rowspan"))
                .map(AttrValue::as_uint)
        }
    }

    fn get_width(&self) -> LengthOrPercentageOrAuto {
        unsafe {
            (&*self.upcast::<Element>().unsafe_get())
                .get_attr_for_layout(&ns!(), &local_name!("width"))
                .map(AttrValue::as_dimension)
                .cloned()
                .unwrap_or(LengthOrPercentageOrAuto::Auto)
        }
    }
}

impl VirtualMethods for HTMLTableCellElement {
    fn super_type(&self) -> Option<&VirtualMethods> {
        Some(self.upcast::<HTMLElement>() as &VirtualMethods)
    }

    fn parse_plain_attribute(&self, local_name: &LocalName, value: DOMString) -> AttrValue {
        match *local_name {
            local_name!("colspan") => {
                let mut attr = AttrValue::from_u32(value.into(), DEFAULT_COLSPAN);
                if let AttrValue::UInt(ref mut s, ref mut val) = attr {
                    if *val == 0 {
                        *val = 1;
                        *s = "1".into();
                    }
                }
                attr
            },
            local_name!("rowspan") => {
                let mut attr = AttrValue::from_u32(value.into(), DEFAULT_ROWSPAN);
                if let AttrValue::UInt(ref mut s, ref mut val) = attr {
                    if *val == 0 {
                        let node = self.upcast::<Node>();
                        let doc = node.owner_doc();
                        // rowspan = 0 is not supported in quirks mode
                        if doc.quirks_mode() != QuirksMode::NoQuirks {
                            *val = 1;
                            *s = "1".into();
                        }
                    }
                }
                attr
            },
            local_name!("bgcolor") => AttrValue::from_legacy_color(value.into()),
            local_name!("width") => AttrValue::from_nonzero_dimension(value.into()),
            _ => self
                .super_type()
                .unwrap()
                .parse_plain_attribute(local_name, value),
        }
    }
}
