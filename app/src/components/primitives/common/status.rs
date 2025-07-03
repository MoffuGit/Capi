use leptos::prelude::*;
use leptos::*;
use std::rc::Rc;
use wasm_bindgen::prelude::Closure;
use wasm_bindgen::JsCast;

#[derive(Clone, Copy)]
pub struct AnimationFrame;

impl AnimationFrame {
    pub fn create() -> Self {
        Self {}
    }

    pub fn request(f: Rc<Closure<dyn Fn()>>) -> impl Fn() + 'static {
        let handle = window()
            .request_animation_frame((*f).as_ref().unchecked_ref())
            .unwrap();
        // Return a cleanup function for manual cancellation
        move || {
            if let Some(window) = web_sys::window() {
                window.cancel_animation_frame(handle).unwrap_or_default();
            }
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum TransitionStatus {
    Starting,
    Ending,
    Idle,
    #[doc(hidden)] // Hidden to avoid exposing implementation detail
    Undefined,
}

pub fn use_transition_status(
    open: ReadSignal<bool>,
    enable_idle_state: bool,
    defer_ending_state: bool,
    open_duration: u64,
    close_duration: u64,
) -> TransitionStatusState {
    // Always start in Undefined, allow effects to transition to Idle.
    let transition_status: RwSignal<TransitionStatus> = RwSignal::new(TransitionStatus::Undefined);
    // `mounted` controls whether the dialog content (Portal) is in the DOM.
    // It should be true when the dialog is in any active transition state (Starting, Ending, Idle),
    // and false when it's Undefined (i.e., fully closed and removed from DOM).
    let mounted: RwSignal<bool> = RwSignal::new(false);

    // This effect manages the 'mounted' state based on 'transition_status' and 'open'.
    // The dialog should only be mounted when it's active in the DOM (Starting, Idle, Ending).
    // It should be unmounted only when fully closed (`!open`) AND status is `Undefined`.
    Effect::new(move |_| {
        let current_status = transition_status.get();
        let current_open = open.get();

        match current_status {
            TransitionStatus::Starting | TransitionStatus::Idle | TransitionStatus::Ending => {
                // When in any active transition state, ensure dialog is mounted.
                if !mounted.get_untracked() {
                    mounted.set(true);
                }
            }
            TransitionStatus::Undefined => {
                // When status is Undefined, it means the closing animation finished.
                // If the dialog is actually closed (`!current_open`), then unmount.
                // If `current_open` is true (e.g., initial render state of `Undefined` while dialog is meant to be open),
                // we don't unmount, because the next tick will set it to `Starting`.
                if !current_open && mounted.get_untracked() {
                    mounted.set(false);
                }
            }
        }
    });

    // Effect 3: Set `transition_status` to `Starting` when dialog opens or re-opens,
    // and then schedule transition to `Idle` if enabled.
    Effect::new(move |_| {
        let current_open = open.get();
        let current_status = transition_status.get();
        let enable_idle_captured = enable_idle_state;

        // Condition for setting to Starting:
        // If open and not already in Starting or Idle (meaning it's closed or just mounted)
        if current_open
            && (current_status == TransitionStatus::Undefined
                || current_status == TransitionStatus::Ending)
        {
            // Delay setting 'Starting' using a 0ms timeout to ensure the DOM renders
            // the initial state before applying the 'Starting' state for animation.
            #[cfg(not(feature = "ssr"))]
            {
                let transition_status_setter = transition_status; // Capture RwSignal
                let timeout_handle = set_timeout_with_handle(
                    move || {
                        transition_status_setter.set(TransitionStatus::Starting);
                    },
                    std::time::Duration::from_millis(0), // Defer to next event loop tick
                )
                .expect("Failed to set timeout for Starting transition");
                on_cleanup(move || {
                    timeout_handle.clear();
                });
            }

            // If idle state is enabled, schedule transition to Idle after animation duration
            if enable_idle_captured {
                let transition_status_setter = transition_status;
                let timeout_handle = set_timeout_with_handle(
                    move || {
                        transition_status_setter.set(TransitionStatus::Idle);
                    },
                    std::time::Duration::from_millis(open_duration),
                )
                .expect("Failed to set timeout for Idle transition");
                on_cleanup(move || {
                    timeout_handle.clear();
                });
            }
        }
    });

    // Effect 4: Set `transition_status` to `Ending` immediately on close if not deferred.
    // This initiates the closing animation for non-deferred cases.
    Effect::new(move |_| {
        let current_open = open.get();
        let current_status = transition_status.get();
        let current_mounted = mounted.get(); // Track mounted to ensure it's still active

        if !current_open
            && current_mounted
            && current_status != TransitionStatus::Ending
            && !defer_ending_state
        {
            transition_status.set(TransitionStatus::Ending);
        }
    });

    #[cfg(not(feature = "ssr"))]
    let ending = Rc::new(Closure::new(move || {
        transition_status.set(TransitionStatus::Ending);
    }));

    // Effect 5: Deferred `Ending` transition using `AnimationFrame` for closing animations.
    // This effect determines *when* the 'Ending' status is set if deferred.
    Effect::new(move |_| {
        #[cfg(not(feature = "ssr"))]
        {
            let open_val = open.get();
            let mounted_val = mounted.get();
            let status_val = transition_status.get();

            if !open_val
                && mounted_val
                && status_val != TransitionStatus::Ending
                && defer_ending_state
            {
                let cancel_frame = AnimationFrame::request(ending.clone());
                on_cleanup(move || {
                    cancel_frame();
                });
            }
        }
    });

    // Effect: From Ending to Undefined after animation completes.
    // This handles the "animation out" completion.
    Effect::new(move |_| {
        let current_open = open.get();
        let current_status = transition_status.get();

        if !current_open && current_status == TransitionStatus::Ending {
            let transition_status_setter = transition_status;
            let timeout_handle = set_timeout_with_handle(
                move || {
                    transition_status_setter.set(TransitionStatus::Undefined);
                },
                std::time::Duration::from_millis(close_duration - 10),
            )
            .expect("Failed to set timeout for Undefined transition");
            on_cleanup(move || {
                timeout_handle.clear();
            });
        }
    });

    let (mounted, set_mounted) = mounted.split();

    TransitionStatusState {
        mounted,
        set_mounted,
        transition_status,
    }
}

#[derive(Clone, Copy)]
pub struct TransitionStatusState {
    pub mounted: ReadSignal<bool>,
    pub set_mounted: WriteSignal<bool>,
    pub transition_status: RwSignal<TransitionStatus>,
}
