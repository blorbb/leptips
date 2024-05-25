use floater::{
    compute_position,
    geometry::{ElemRect, ElemSize},
    modifiers::{self, arrow::ArrowData, shift::limiter},
    padding::Padding,
    PositionOpts,
};
use leptos::*;
use leptos_use::use_event_listener;
use web_sys::wasm_bindgen::JsCast;

mod opts;
pub use opts::*;

pub use floater::geometry::Side;

macro_rules! clone {
    ($($ident:ident)*) => {
        $(let $ident = $ident.clone();)*
    };
}

pub fn tooltip(el: leptos::HtmlElement<html::AnyElement>, opts: Opts) {
    let content = opts.content.clone();
    let context_opts = use_context::<Opts>().unwrap_or_else(Opts::default);
    let opts = AllOpts::default()
        .overwrite_from(context_opts)
        .overwrite_from(opts);
    let container = (opts.container.clone()).and_then(|f| f());

    // to put styles into this one
    let arrow = NodeRef::new();
    // to get dimensions from this one
    let arrow_inner = NodeRef::new();

    _ = el.clone().classes(opts.class);
    let tip = view! {
        <div
            class={format!("tooltip {}", opts.class)}
            style:position="fixed"
        >
            <div class="tooltip-contents" style:border-radius={format!("{}px", opts.border_radius)}>
                {content.run()}
            </div>
            <div
                class="tooltip-arrow-box"
                ref=arrow
                style:position="fixed"
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

    let scroll_callback = {
        clone!(el tip container arrow arrow_inner opts);
        move |_| {
            if tip.is_connected() {
                recalculate_all_opts(&el, &tip, container.as_ref(), &arrow, &arrow_inner, &opts)
            }
        }
    };
    _ = use_event_listener(container.clone(), ev::scroll, scroll_callback.clone());
    let handle = window_event_listener(ev::scroll, scroll_callback);
    on_cleanup(move || handle.remove());

    let handle = window_event_listener(ev::blur, {
        clone!(tip);
        move |_| {
            tip.remove();
        }
    });
    on_cleanup(move || handle.remove());

    let handle = window_event_listener(ev::resize, {
        clone!(el tip container arrow arrow_inner opts);
        move |_| {
            if tip.is_connected() {
                recalculate_all_opts(&el, &tip, container.as_ref(), &arrow, &arrow_inner, &opts)
            }
        }
    });
    on_cleanup(move || handle.remove());

    match opts.show_on {
        ShowOn::Hover => {
            _ = el.clone().on(ev::undelegated(ev::mouseenter), {
                clone!(el tip container arrow arrow_inner opts);
                move |_| {
                    recalculate_all_opts(&el, &tip, container.as_ref(), &arrow, &arrow_inner, &opts)
                }
            });
            _ = el.clone().on(ev::undelegated(ev::mouseleave), {
                clone!(tip);
                move |_| tip.remove()
            });
        }
        ShowOn::Click => {
            _ = el.clone().on(ev::undelegated(ev::click), {
                clone!(el tip container arrow arrow_inner opts);
                move |_| {
                    recalculate_all_opts(&el, &tip, container.as_ref(), &arrow, &arrow_inner, &opts)
                }
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
    container: Option<&web_sys::Element>,
    arrow: &NodeRef<html::Div>,
    arrow_inner: &NodeRef<html::Div>,
    opts: Opts,
) {
    let opts = AllOpts::default().overwrite_from(opts);
    recalculate_all_opts(el, tip, container, arrow, arrow_inner, &opts)
}

fn recalculate_all_opts(
    el: &web_sys::HtmlElement,
    tip: &leptos::HtmlElement<html::Div>,
    container: Option<&web_sys::Element>,
    arrow: &NodeRef<html::Div>,
    arrow_inner: &NodeRef<html::Div>,
    opts: &AllOpts,
) {
    let (arrow, arrow_inner) = (arrow.get().unwrap(), arrow_inner.get().unwrap());

    if !tip.is_connected() {
        _ = el.after_with_node_1(&tip);
    }

    let viewport_rect = {
        let html_element: web_sys::HtmlHtmlElement = document()
            .scrolling_element()
            .expect("document should have scrolling element")
            .dyn_into()
            .unwrap();
        ElemRect::new(
            0.0,
            0.0,
            f64::from(html_element.client_width()),
            f64::from(html_element.client_height()),
        )
    };

    let con_rect = if let Some(container) = container {
        ElemRect::from_bounding_client_rect(&container).intersect(&viewport_rect)
    } else {
        viewport_rect
    };
    let ref_rect = ElemRect::from_bounding_client_rect(&el);
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
