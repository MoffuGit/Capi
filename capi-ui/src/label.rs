use leptos::prelude::*;
use tailwind_fuse::tw_merge;

#[component]
pub fn Label(children: Children, #[prop(into, optional)] class: String) -> impl IntoView {
    view! {
        <label class=tw_merge!("flex items-center gap-2 text-sm leading-none font-medium select-none group-data-[disabled=true]:pointer-events-none group-data-[disabled=true]:opacity-50 peer-disabled:cursor-not-allowed peer-disabled:opacity-50", class)>
            {children()}
        </label>
    }
}
