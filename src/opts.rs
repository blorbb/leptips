use floater::geometry::Side;
use leptos::*;

/// Tooltip options to be passed in to `use:tooltip`.
///
/// This struct should be constructed using the [`tip`] free function.
/// Configuration is done with the builder pattern. All options not passed
/// in explicitly will be set to a default, either through a provided
/// [`context`](leptos::provide_context) of type [`DefaultOpts`],
/// or the default configuration [`DefaultOpts::default`].
///
/// A blanket implementation is provided to convert all view functions into
/// a [`PartialOpts`] struct. If you don't want to override any options,
/// you can just provide a view function in `use:tooltip` instead of wrapping
/// it in [`tip`].
#[derive(Default, Clone)]
pub struct PartialOpts {
    pub(crate) content: ViewFn,
    pub(crate) padding: Option<f64>,
    pub(crate) border_radius: Option<f64>,
    pub(crate) class: Option<&'static str>,
    pub(crate) side: Option<Side>,
    pub(crate) show_on: Option<ShowOn>,
    /// First option is whether the arrow property is set.
    /// Second option is whether there is an arrow.
    pub(crate) arrow: Option<Option<ViewFn>>,
}

impl<T: Into<ViewFn>> From<T> for PartialOpts {
    fn from(value: T) -> Self {
        Self {
            content: value.into(),
            ..Default::default()
        }
    }
}

pub fn tip<T: Into<ViewFn>>(view: T) -> PartialOpts {
    view.into().into()
}

impl PartialOpts {
    pub fn with_padding(mut self, padding: f64) -> Self {
        self.padding = Some(padding);
        self
    }

    pub fn with_side(mut self, side: Side) -> Self {
        self.side = Some(side);
        self
    }

    pub fn with_border_radius(mut self, border_radius: f64) -> Self {
        self.border_radius = Some(border_radius);
        self
    }

    pub fn with_class(mut self, class: &'static str) -> Self {
        self.class = Some(class);
        self
    }

    pub fn with_arrow(mut self, arrow: Option<impl Into<ViewFn>>) -> Self {
        self.arrow = Some(arrow.map(Into::into));
        self
    }

    pub fn show_on(mut self, on: ShowOn) -> Self {
        self.show_on = Some(on);
        self
    }

    /// Fills the options with the provided [`DefaultOpts`].
    ///
    /// This can be used as an alternative to providing a context, e.g. for
    /// making a single tooltip have different options.
    ///
    /// This should be ran as the first method after [`tip`]. Individual settings
    /// can still be overridden with the other builder methods.
    pub fn fill_from(mut self, opts: DefaultOpts) -> Self {
        self.padding = Some(opts.padding);
        self.side = Some(opts.side);
        self.arrow = Some(opts.arrow);
        self.show_on = Some(opts.show_on);
        self.border_radius = Some(opts.border_radius);
        self.class = Some(opts.class);
        self
    }
}

/// Default options to be used by a tooltip.
///
/// This should generally be used in a [`provide_context`], where all tooltips
/// in the current (and child) components will then get these configuration options.
#[derive(Clone)]
pub struct DefaultOpts {
    pub padding: f64,
    pub side: Side,
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

impl Default for DefaultOpts {
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
        }
    }
}

impl DefaultOpts {
    pub(crate) fn overwrite_from(mut self, opts: PartialOpts) -> Self {
        if let Some(padding) = opts.padding {
            self.padding = padding;
        };
        if let Some(side) = opts.side {
            self.side = side;
        }
        if let Some(arrow) = opts.arrow {
            self.arrow = arrow;
        }
        if let Some(show_on) = opts.show_on {
            self.show_on = show_on;
        }
        if let Some(border_radius) = opts.border_radius {
            self.border_radius = border_radius;
        }
        if let Some(class) = opts.class {
            self.class = class;
        }
        self
    }
}
