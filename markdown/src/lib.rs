use leptos::prelude::*;

#[component]
pub fn Markdown(
    #[prop(into)] source: Signal<String>,
    #[prop(into, optional)] class: MaybeProp<String>,
) -> impl IntoView {
    let html = Signal::derive(move || markdown::to_html(&source.get()));
    view! {
        <div class=class inner_html=html/>
    }
}
