use floater::{
    compute_position,
    geometry::{ElemRect, ElemSize},
    modifiers::{self, arrow::ArrowData, shift::limiter},
    padding::Padding,
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

pub fn tooltip(el: leptos::HtmlElement<html::AnyElement>, opts: TooltipOpts) {
    // to put styles into this one
    let arrow = NodeRef::new();
    // to get dimensions from this one
    let arrow_inner = NodeRef::new();

    _ = el.clone().classes(opts.class);
    let tip = view! {
        <div
            class={format!("tooltip {}", opts.class)}
            style:position="absolute"
        >
            <div class="tooltip-contents" style:border-radius={format!("{}px", opts.border_radius)}>
                {opts.content.run()}
            </div>
            <div
                class="tooltip-arrow-box"
                ref=arrow
                style:position="absolute"
                style:aspect-ratio=1
                style:pointer-events="none"
            >
                <div
                    ref=arrow_inner
                    class="tooltip-arrow"
                    style:display="grid"
                    style:place-content="start"
                >
                    {opts.arrow.clone().map(|view| view.run())}
                </div>
            </div>
        </div>
    };

    // TODO: find actual scrolling element
    let container = document()
        .scrolling_element()
        .unwrap()
        .dyn_into::<web_sys::HtmlElement>()
        .expect("reference element's offset parent should be an HTML element");

    let handler = window_event_listener(ev::scroll, {
        clone!(el tip container arrow arrow_inner opts);
        move |_| {
            logging::log!("scrolled");
            if tip.is_connected() {
                recalculate(&el, &tip, &container, &arrow, &arrow_inner, &opts)
            }
        }
    });
    on_cleanup(|| handler.remove());

    let handler = window_event_listener(ev::blur, {
        clone!(tip);
        move |_| {
            tip.remove();
        }
    });
    on_cleanup(|| handler.remove());

    let handler = window_event_listener(ev::resize, {
        clone!(el tip container arrow arrow_inner opts);
        move |_| {
            if tip.is_connected() {
                recalculate(&el, &tip, &container, &arrow, &arrow_inner, &opts)
            }
        }
    });
    on_cleanup(|| handler.remove());

    match opts.show_on {
        ShowOn::Hover => {
            _ = el.clone().on(ev::mouseenter, {
                clone!(el tip container arrow arrow_inner opts);
                move |_| recalculate(&el, &tip, &container, &arrow, &arrow_inner, &opts)
            });
            _ = el.clone().on(ev::mouseleave, {
                clone!(tip);
                move |_| tip.remove()
            });
        }
        ShowOn::Click => {
            _ = el.clone().on(ev::click, {
                clone!(el tip container arrow arrow_inner opts);
                move |_| recalculate(&el, &tip, &container, &arrow, &arrow_inner, &opts)
            });
            // remove on click outside
            let handler = window_event_listener(ev::click, {
                clone!(el tip);
                move |ev| {
                    let target = ev
                        .target()
                        .and_then(|target| target.dyn_into::<web_sys::Node>().ok());

                    if !(el.contains(target.as_ref()) || tip.contains(target.as_ref())) {
                        tip.remove();
                    }
                }
            });
            on_cleanup(|| handler.remove());
        }
    }
}

pub fn recalculate(
    el: &web_sys::HtmlElement,
    tip: &leptos::HtmlElement<html::Div>,
    container: &web_sys::HtmlElement,
    arrow: &NodeRef<html::Div>,
    arrow_inner: &NodeRef<html::Div>,
    opts: &TooltipOpts,
) {
    let (arrow, arrow_inner) = (arrow.get().unwrap(), arrow_inner.get().unwrap());

    if !tip.is_connected() {
        _ = el.after_with_node_1(&tip);
    }

    let con_rect = ElemRect::from_elem_visibility(&container);
    let ref_rect = ElemRect::from_elem_offset(&el);
    let tip_size = ElemSize::from_bounding_client_rect(&tip);
    // don't use client rect so that it's consistent even if rotated
    let arr_size = ElemSize::new(
        f64::from(arrow_inner.offset_width()),
        f64::from(arrow_inner.offset_height()),
    );

    let mut arrow_data = ArrowData::new();
    let arr_width = arr_size.width();
    let arr_height = arr_size.height();

    let mod_padding = Padding {
        outward: opts.padding * 2.0 + arr_height,
        inward: opts.padding,
        cross: opts.padding,
    };

    let data = compute_position(
        ref_rect,
        tip_size,
        con_rect,
        PositionOpts::new()
            .with_side(opts.side)
            .add_modifier(&mut modifiers::flip().padding(mod_padding))
            .add_modifier(
                &mut modifiers::shift()
                    .padding(mod_padding)
                    .limiter(limiter::attached(arr_width / 2.0 + opts.border_radius)),
            )
            .add_modifier(&mut modifiers::offset(opts.padding + arr_height))
            .add_modifier(
                opts.arrow
                    .as_ref()
                    .map(|_| {
                        Box::new(
                            modifiers::arrow(arr_width, &mut arrow_data)
                                .padding(opts.border_radius),
                        )
                    })
                    .as_deref_mut(),
            ),
    );

    let (x, y) = data.rect.xy();
    _ = tip.clone().style("left", format!("{x}px"));
    _ = tip.clone().style("top", format!("{y}px"));

    let arr_css = arrow_data.generate_css_props(data.side, arr_width, "px");
    // clear any other positioning attributes, e.g. when it flips
    let arr_stylesheet = (*arrow).style();
    for side in ["left", "top", "right", "bottom"] {
        _ = arr_stylesheet.remove_property(side);
    }
    // set the actual styles
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
    pub show_on: ShowOn,
    pub border_radius: f64,
    pub class: &'static str,
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub enum ShowOn {
    #[default]
    Hover,
    Click,
}

impl Default for TooltipOpts {
    fn default() -> Self {
        Self {
            padding: 0.0,
            side: Side::default(),
            content: ViewFn::default(),
            show_on: ShowOn::default(),
            arrow: Some(
                (|| view! {
                    <svg width="16" height="6" xmlns="http://www.w3.org/2000/svg" style:transform="rotate(180deg)">
                        <path d="M0 6s1.796-.013 4.67-3.615C5.851.9 6.93.006 8 0c1.07-.006 2.148.887 3.343 2.385C14.233 6.005 16 6 16 6H0z" />
                    </svg>
                }).into(),
            ),
            border_radius: 5.0,
            class: "",
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
