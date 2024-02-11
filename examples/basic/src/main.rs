use leptips::{tooltip, Side, TooltipOpts};
use leptos::*;

fn main() {
    console_error_panic_hook::set_once();
    mount_to_body(App)
}

#[component]
fn App() -> impl IntoView {
    let count = RwSignal::new(0);
    view! {
        <hr/><hr/><hr/><hr/><hr/><hr/><hr/><hr/><hr/><hr/><hr/><hr/><hr/><hr/><hr/><hr/><hr/><hr/><hr/><hr/><hr/><hr/><hr/><hr/><hr/><hr/><hr/><hr/><hr/><hr/><hr/><hr/><hr/><hr/>
        <div class="center" style:height="20rem">
            <div>
                <button
                    use:tooltip={move || view! {"my count is " {count}}}
                    on:click=move |_| count.update(|c| *c += 1)
                >"look at me!"</button>
                <br />
                <button
                    use:tooltip={TooltipOpts {
                        padding: 5.0,
                        side: Side::Left,
                        content: (|| view! { "heelllllooo there" }).into(),
                        show_on: leptips::ShowOn::Click,
                        ..Default::default()
                    }}
                >
                    "this is a button wooo"
                    <hr/>
                    "what"
                </button>
            </div>
        </div>
        <hr/><hr/><hr/><hr/><hr/><hr/><hr/><hr/><hr/><hr/><hr/><hr/><hr/><hr/><hr/><hr/><hr/><hr/><hr/><hr/><hr/><hr/><hr/><hr/><hr/><hr/><hr/><hr/><hr/><hr/><hr/><hr/><hr/><hr/>
    }
}
