use leptos::{
    html::{ElementType, HtmlElement},
    prelude::*,
    tachys::html::node_ref::NodeRefContainer,
    wasm_bindgen::JsCast,
};
use leptos_node_ref::{any_node_ref, AnyNodeRef};
use leptos_typed_fallback_show::TypedFallbackShow;

#[component]
pub fn Primitive<E, C>(
    element: fn() -> HtmlElement<E, (), ()>,
    #[prop(into, optional)] as_child: MaybeProp<bool>,
    #[prop(into, optional)] node_ref: AnyNodeRef,
    children: TypedChildrenFn<C>,
) -> impl IntoView
where
    E: ElementType + 'static,
    C: IntoView + 'static,
    View<C>: RenderHtml,
    HtmlElement<E, (), ()>: ElementChild<View<C>>,
    <HtmlElement<E, (), ()> as ElementChild<View<C>>>::Output: IntoView,
    <E as ElementType>::Output: JsCast,
    AnyNodeRef: NodeRefContainer<E>,
{
    let children = StoredValue::new(children.into_inner());

    view! {
        <TypedFallbackShow
            when=move || as_child.get().unwrap_or_default()
            fallback=move || {
                element().child(children.with_value(|children| children())).add_any_attr(any_node_ref(node_ref))
            }
        >
            {children.with_value(|children| children()).add_any_attr(any_node_ref(node_ref))}
        </TypedFallbackShow>
    }
}
