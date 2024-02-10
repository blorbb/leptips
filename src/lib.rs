use floater::{
    compute_position,
    geometry::{ElemRect, ElemSize},
    modifiers::{self, arrow::ArrowData},
    PositionOpts,
};
use leptos::*;
use web_sys::wasm_bindgen::JsCast;

pub use floater::geometry::Side;

macro_rules! clone {
    ($($ident:ident)*) => {
        $(let $ident = $ident.clone();)*
    };
}

static WINDOW_SCROLL_EV: std::sync::Once = std::sync::Once::new();

pub fn tooltip(el: leptos::HtmlElement<html::AnyElement>, opts: TooltipOpts) {
    let arrow = NodeRef::new();
    let tip = view! {
        <div class="tooltip" style:position="absolute">
            <div class="tooltip-contents">
                {opts.content.run()}
            </div>
            <div
                class="tooltip-arrow-box"
                ref=arrow
                style:position="absolute"
                style:aspect-ratio=1
            >
                <div
                    class="tooltip-arrow"
                    style:display="grid"
                    style:place-content="start"
                >
                    {opts.arrow.clone().map(|view| view.run())}
                </div>
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

    WINDOW_SCROLL_EV.call_once(|| {
        window_event_listener(ev::scroll, {
            clone!(el tip container arrow opts);
            move |_| {
                if !tip.is_connected() {
                    return;
                }
                recalculate(&el, &tip, &container, &arrow.get().unwrap(), &opts)
            }
        });
    });

    // show on hover (needs to be fixed up)
    _ = el.clone().on(ev::click, {
        clone!(el tip container arrow opts);
        move |_| recalculate(&el, &tip, &container, &arrow.get().unwrap(), &opts)
    });
    // _ = el.clone().on(ev::mouseleave, move |_| tip.remove());
}

pub fn recalculate(
    el: &web_sys::HtmlElement,
    tip: &leptos::HtmlElement<html::Div>,
    container: &web_sys::HtmlElement,
    arrow: &leptos::HtmlElement<html::Div>,
    opts: &TooltipOpts,
) {
    let el = el.clone();
    let tip = tip.clone();
    let container = container.clone();
    let opts = opts.clone();

    tip.remove();
    _ = el.after_with_node_1(&tip);

    let con_rect = ElemRect::from_elem_visibility(&container);
    let ref_rect = ElemRect::from_elem_offset(&el);
    let tip_size = ElemSize::from_bounding_client_rect(&tip);
    logging::log!("{tip_size:?}");

    let mut arrow_data = ArrowData::new();
    let arr_width = arrow.get_bounding_client_rect().width();

    let data = compute_position(
        ref_rect,
        tip_size,
        con_rect,
        PositionOpts::new()
            .with_side(opts.side)
            .add_modifier(&mut modifiers::offset(opts.padding))
            .add_modifier(
                opts.arrow
                    .map(|_| Box::new(modifiers::arrow(arr_width, &mut arrow_data)))
                    .as_deref_mut(),
            ),
    );

    let (x, y) = data.rect.xy();
    _ = tip.clone().style("left", format!("{x}px"));
    _ = tip.clone().style("top", format!("{y}px"));

    let arr_css = arrow_data.generate_css_text(data.side, arr_width, "px");
    arr_css.into_iter().for_each(|(k, v)| {
        _ = arrow.clone().style(k, v);
    });
}

#[derive(Clone)]
pub struct TooltipOpts {
    pub padding: f64,
    pub side: Side,
    pub content: ViewFn,
    pub arrow: Option<ViewFn>,
}

impl Default for TooltipOpts {
    fn default() -> Self {
        Self {
            padding: 0.0,
            side: Side::default(),
            content: ViewFn::default(),
            arrow: Some(
                (|| view! {
                    <svg width="16" height="6" xmlns="http://www.w3.org/2000/svg" style:transform="rotate(180deg)">
                        <path d="M0 6s1.796-.013 4.67-3.615C5.851.9 6.93.006 8 0c1.07-.006 2.148.887 3.343 2.385C14.233 6.005 16 6 16 6H0z" />
                    </svg>
                }).into(),
            ),
        }
    }
}

impl<T: Into<ViewFn>> From<T> for TooltipOpts {
    fn from(value: T) -> Self {
        Self {
            content: value.into(),
            ..Default::default()
        }
    }
}
