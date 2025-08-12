use leptos::prelude::*;

use capi_primitives::checkbox::{
    CheckboxIndicator as CheckboxIndicatorPrimitive, CheckboxRoot as CheckboxRootPrimitive,
};
use tailwind_fuse::tw_merge;

use icons::IconCheck;

#[component]
pub fn Checkbox(
    #[prop(into, optional)] class: Signal<String>,
    #[prop(into, optional)] checked: RwSignal<bool>,
) -> impl IntoView {
    view! {
        <CheckboxRootPrimitive
            checked=checked
            class=Signal::derive(move || tw_merge!("peer border-input dark:bg-input/30 data-[state=checked]:bg-primary data-[state=checked]:text-primary-foreground dark:data-[state=checked]:bg-primary data-[state=checked]:border-primary focus-visible:border-ring focus-visible:ring-ring/50 aria-invalid:ring-destructive/20 dark:aria-invalid:ring-destructive/40 aria-invalid:border-destructive size-4 shrink-0 rounded-[4px] border shadow-xs transition-shadow outline-none focus-visible:ring-[3px] disabled:cursor-not-allowed disabled:opacity-50", class.get()))
        {..}
        data-slot="checkbox"
    >
      <CheckboxIndicatorPrimitive
        class="flex items-center justify-center text-current transition-none"
        {..}
        data-slot="checkbox-indicator"
      >
        <IconCheck class="size-3.5" />
      </CheckboxIndicatorPrimitive>
    </CheckboxRootPrimitive>
    }
}
