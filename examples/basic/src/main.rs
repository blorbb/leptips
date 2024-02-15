use leptips::{tip, tooltip, DefaultOpts, Side};
use leptos::*;

fn main() {
    console_error_panic_hook::set_once();
    mount_to_body(App)
}

#[component]
fn App() -> impl IntoView {
    provide_context(DefaultOpts {
        padding: 5.0,
        show_on: leptips::ShowOn::Click,
        ..Default::default()
    });

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
                <Show when={move || count.get() % 2 == 0}>
                    <button
                        use:tooltip={tip(|| view! { "heelllllooo there" })
                            .with_padding(5.0)
                            .with_side(Side::Left)
                            .show_on(leptips::ShowOn::Click)
                        }
                    >
                        "this is a button wooo"
                        <hr/>
                        "what"
                    </button>
                </Show>
                <br/>
                <button
                    use:tooltip={move || view! {"my count is " {count}}}
                    on:click=move |_| count.update(|c| *c += 1)
                >"look at me!"</button>
            </div>
        </div>
        <hr/><hr/><hr/><hr/><hr/><hr/><hr/><hr/><hr/><hr/><hr/><hr/><hr/><hr/><hr/><hr/><hr/><hr/><hr/><hr/><hr/><hr/><hr/><hr/><hr/><hr/><hr/><hr/><hr/><hr/><hr/><hr/><hr/><hr/>
    }
}
