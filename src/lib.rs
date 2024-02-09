use floater::{
    compute_position,
    geometry::{ElemRect, ElemSize},
    modifiers, PositionOpts,
};
use leptos::*;
use web_sys::wasm_bindgen::JsCast;

pub use floater::geometry::Side;

pub fn tooltip(el: leptos::HtmlElement<html::AnyElement>, opts: TooltipOpts) {
    let tooltip_el = view! {
        <div class="tooltip">
            <div class="tooltip-contents">
                {opts.content.run()}
            </div>
            <div class="tooltip-arrow-box">
                <div class="tooltip-arrow" />
            </div>
        </div>
    };

    let container = el
        .offset_parent()
        .unwrap_or_else(|| {
            document()
                .document_element()
                .expect("no document element found")
        })
        .dyn_into::<web_sys::HtmlElement>()
        .expect("reference element's offset parent should be an HTML element");

    window_event_listener(ev::scroll, {
        let tooltip_el = tooltip_el.clone();
        let el = el.clone();
        let container = container.clone();
        let opts = opts.clone();
        move |_| {
            if !tooltip_el.is_connected() {
                return;
            }
            recalculate(&el, &tooltip_el, &container, &opts)
        }
    });

    _ = el.clone().on(ev::mouseenter, {
        let (el, tooltip_el, container) = (el.clone(), tooltip_el.clone(), container.clone());
        move |_| recalculate(&el, &tooltip_el, &container, &opts)
    });
    _ = el.clone().on(ev::mouseleave, move |_| tooltip_el.remove());
}

pub fn recalculate(
    el: &web_sys::HtmlElement,
    tip: &leptos::HtmlElement<html::Div>,
    container: &web_sys::HtmlElement,
    opts: &TooltipOpts,
) {
    let el = el.clone();
    let tip = tip.clone();
    let container = container.clone();

    tip.remove();
    _ = el.after_with_node_1(&tip);

    let con_rect = ElemRect::from_elem_visibility(&container);
    let ref_rect = ElemRect::from_elem_offset(&el);
    let tip_size = ElemSize::from_bounding_client_rect(&tip);
    logging::log!("{tip_size:?}");

    let data = compute_position(
        ref_rect,
        tip_size,
        con_rect,
        PositionOpts::new()
            .with_side(opts.side)
            .add_modifier(&mut modifiers::offset(opts.padding)),
    );
    let (x, y) = data.rect.xy();

    let tooltip_styles = (*tip).style();
    tooltip_styles
        .set_property("left", &format!("{x}px"))
        .unwrap();
    tooltip_styles
        .set_property("top", &format!("{y}px"))
        .unwrap();
}

#[derive(Clone, Default)]
pub struct TooltipOpts {
    pub padding: f64,
    pub side: Side,
    pub content: ViewFn,
}

impl<T: Into<ViewFn>> From<T> for TooltipOpts {
    fn from(value: T) -> Self {
        Self {
            content: value.into(),
            ..Default::default()
        }
    }
}
