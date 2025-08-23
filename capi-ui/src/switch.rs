use leptos::prelude::*;
use tailwind_fuse::tw_merge;

use capi_primitives::switch::{
    SwitchRoot as SwitchRootPrimitive, SwitchTumb as SwitchTumbPrimitive,
};

#[component]
pub fn Switch(
    #[prop(into)] checked: RwSignal<bool>,
    #[prop(optional, into)] class: Signal<String>,
    #[prop(optional, into, default = Signal::from(false))] disabled: Signal<bool>,
) -> impl IntoView {
    view! {
    <SwitchRootPrimitive
        checked=checked
        disabled=disabled
        class=Signal::derive(move || {tw_merge!(
            "peer data-[state=checked]:bg-primary data-[state=unchecked]:bg-input focus-visible:border-ring focus-visible:ring-ring/50 dark:data-[state=unchecked]:bg-input/80 inline-flex h-[1.15rem] w-8 shrink-0 items-center rounded-full border border-transparent shadow-xs transition-all outline-none focus-visible:ring-[3px] disabled:cursor-not-allowed disabled:opacity-50",
            class.get()
        )})
        {..}
        data-slot="switch"
    >
      <SwitchTumbPrimitive
        class=tw_merge!(
          "bg-background dark:data-[state=unchecked]:bg-foreground dark:data-[state=checked]:bg-primary-foreground pointer-events-none block size-4 rounded-full ring-0 transition-transform data-[state=checked]:translate-x-[calc(100%-2px)] data-[state=unchecked]:translate-x-0"
        )
        {..}
        data-slot="switch-thumb"
      />
    </SwitchRootPrimitive>
    }
}
