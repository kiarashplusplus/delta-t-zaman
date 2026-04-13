use leptos::*;

#[component]
pub fn Toggle(
    #[prop(into)] checked: Signal<bool>,
    on_change: impl Fn(bool) + 'static + Send + Sync,
    #[prop(default = "")] class: &'static str,
) -> impl IntoView {
    view! {
        <input 
            type="checkbox" 
            class=format!("custom-toggle {}", class)
            checked=checked
            on:change=move |ev| {
                let is_checked = event_target_checked(&ev);
                on_change(is_checked);
            }
        />
    }
}
