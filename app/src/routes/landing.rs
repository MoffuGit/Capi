use leptos::prelude::*;
use leptos_router::components::A;

use crate::components::auth::{SignedIn, SignedOut};
use crate::components::ui::button::{Button, ButtonSizes};

#[component]
pub fn Landing() -> impl IntoView {
    view! {
        <div class="w-full h-screen flex flex-col items-center">
            <header class="flex w-full h-16 shrink-0 items-center gap-2 transition-[width,height] ease-linear group-has-data-[collapsible=icon]/sidebar-wrapper:h-12">
                <SignedIn>
                    <A href="/servers" {..} class="w-auto h-auto ml-auto mr-4">
                    <Button size=ButtonSizes::Sm>
                            "Go to servers"
                    </Button>
                    </A>
                </SignedIn>
                <SignedOut>
                    <A href="/auth/login" {..} class="w-auto h-auto ml-auto mr-4">
                        <Button size=ButtonSizes::Sm >
                                "Log In"
                        </Button>
                    </A>

                    <A href="/auth/signup" {..} class="w-auto h-auto ml-4 mr-4">
                        <Button size=ButtonSizes::Sm >
                                "Sign Up"
                        </Button>
                    </A>
                </SignedOut>
            </header>
            <h1 class="scroll-m-20 text-center text-4xl font-extrabold tracking-tight text-balance">
                "Work on progress"
            </h1>
        </div>
    }
}
