use floater::geometry::Side;
use leptos::*;
use leptos_use::core::ElementMaybeSignal;
use std::rc::Rc;

/// Tooltip options to be passed in to `use:tooltip`.
///
/// This struct should be constructed using the [`tip`] free function.
/// Configuration is done with the builder pattern. All options not passed
/// in explicitly will be set to a default, either through a provided
/// [`context`](leptos::provide_context) or the default configuration.
///
/// A blanket implementation is provided to convert all view functions into
/// a [`Opts`] struct. If you don't want to override any options,
/// you can just provide a view function in `use:tooltip` instead of wrapping
/// it in [`tip`].
#[derive(Default, Clone)]
pub struct Opts {
    pub(crate) content: ViewFn,
    pub(crate) padding: Option<f64>,
    pub(crate) border_radius: Option<f64>,
    pub(crate) class: Option<&'static str>,
    pub(crate) side: Option<Side>,
    pub(crate) show_on: Option<ShowOn>,
    /// First option is whether the arrow property is set.
    /// Second option is whether there is an arrow.
    pub(crate) arrow: Option<Option<ViewFn>>,
    /// First option is whether the container property is set.
    /// Second option is whether there is a container (falls back to window if its None)
    /// Third option is for whether the element has been created. This should always
    /// be `Some` when the container needs to be used.
    #[allow(clippy::type_complexity)]
    pub(crate) container: Option<Option<Rc<dyn Fn() -> Option<web_sys::Element>>>>,
}

impl<T: Into<ViewFn>> From<T> for Opts {
    fn from(value: T) -> Self {
        Self {
            content: value.into(),
            ..Default::default()
        }
    }
}

pub fn tip<T: Into<ViewFn>>(view: T) -> Opts {
    view.into().into()
}

impl Opts {
    pub fn empty() -> Opts {
        tip(|| "")
    }

    pub fn padding(mut self, padding: f64) -> Self {
        self.padding = Some(padding);
        self
    }

    pub fn side(mut self, side: Side) -> Self {
        self.side = Some(side);
        self
    }

    pub fn border_radius(mut self, border_radius: f64) -> Self {
        self.border_radius = Some(border_radius);
        self
    }

    pub fn class(mut self, class: &'static str) -> Self {
        self.class = Some(class);
        self
    }

    pub fn arrow(mut self, arrow: Option<impl Into<ViewFn>>) -> Self {
        self.arrow = Some(arrow.map(Into::into));
        self
    }

    pub fn show_on(mut self, on: ShowOn) -> Self {
        self.show_on = Some(on);
        self
    }

    /// Sets the container of the tooltip and reference.
    ///
    /// The input should usually be a [`NodeRef`], but can also be a
    /// specific element. The element must be already mounted to the
    /// page, otherwise functions like [`recalculate`] and [`tooltip`]
    /// may panic.
    ///
    /// [`recalculate`]: crate::recalculate
    /// [`tooltip`]: crate::tooltip
    pub fn container<El, T>(mut self, container: El) -> Self
    where
        El: Into<ElementMaybeSignal<T, web_sys::Element>>,
        T: Into<web_sys::Element> + Clone + 'static,
    {
        let e: ElementMaybeSignal<_, _> = container.into();
        let el: Rc<dyn Fn() -> Option<web_sys::Element>> = {
            match e {
                ElementMaybeSignal::Static(st) => Rc::new(move || st.clone().map(Into::into)),
                ElementMaybeSignal::Dynamic(dy) => Rc::new(move || dy.get().map(Into::into)),
                ElementMaybeSignal::_Phantom(_) => unreachable!(),
            }
        };
        self.container = Some(Some(el));
        self
    }

    /// Sets the container to be the entire window.
    pub fn window_container(mut self) -> Self {
        self.container = Some(None);
        self
    }
}

#[derive(Clone)]
pub(crate) struct AllOpts {
    pub padding: f64,
    pub side: Side,
    pub arrow: Option<ViewFn>,
    pub show_on: ShowOn,
    pub border_radius: f64,
    pub class: &'static str,
    /// Defaults to the whole window if this is `None`.
    pub container: Option<Rc<dyn Fn() -> Option<web_sys::Element>>>,
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub enum ShowOn {
    #[default]
    Hover,
    Click,
}

impl Default for AllOpts {
    fn default() -> Self {
        Self {
            padding: 0.0,
            side: Side::default(),
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
            container: None,
        }
    }
}

impl AllOpts {
    pub(crate) fn overwrite_with(mut self, opts: Opts) -> Self {
        let Opts {
            content: _,
            padding,
            border_radius,
            class,
            side,
            show_on,
            arrow,
            container,
        } = opts;

        self.padding = padding.unwrap_or(self.padding);
        self.border_radius = border_radius.unwrap_or(self.border_radius);
        self.class = class.unwrap_or(self.class);
        self.side = side.unwrap_or(self.side);
        self.show_on = show_on.unwrap_or(self.show_on);
        self.arrow = arrow.unwrap_or(self.arrow);
        self.container = container.unwrap_or(self.container);

        self
    }
}
