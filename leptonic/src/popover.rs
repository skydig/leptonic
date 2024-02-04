use std::rc::Rc;

use leptos::*;
use leptos_use::{use_element_bounding, use_element_hover};

use crate::{Size, UseElementBoundingReturnReadOnly};

#[derive(Clone)]
struct PopoverData {
    key: uuid::Uuid,
    children: ChildrenFn,
}

#[derive(Clone)]
struct PopoverRootContext {
    popovers: RwSignal<Vec<PopoverData>>,
}

#[component]
pub(crate) fn PopoverRoot(children: Children) -> impl IntoView {
    let popovers = create_rw_signal(Vec::new());
    let ctx = PopoverRootContext { popovers };
    provide_context::<PopoverRootContext>(ctx.clone());

    let children = children();
    view! {
        { children }

        <leptonic-popover-host>
            <For
                each=move || ctx.popovers.get()
                key=|it| it.key
                children=|it| it.children
            />
        </leptonic-popover-host>
    }
}

#[derive(Debug, Clone, Copy)]
pub enum PopoverAlignX {
    Left,
    Center,
    Right,
}

#[derive(Debug, Clone, Copy)]
pub enum PopoverAlignY {
    Top,
    Center,
    Bottom,
}

#[derive(Clone)]
#[slot]
pub struct PopoverContent {
    children: ChildrenFn,
}

#[component]
pub fn Popover(
    #[prop(into, optional)] id: Option<AttributeValue>,
    #[prop(into, optional)] class: Option<AttributeValue>,
    #[prop(into, optional)] style: Option<AttributeValue>,

    #[prop(default = PopoverAlignX::Center)] align_x: PopoverAlignX,
    #[prop(default = PopoverAlignY::Top)] align_y: PopoverAlignY,

    /// Margin.
    #[prop(default = Size::Em(0.25))]
    margin: Size,

    /// Custom X position of the popover.
    #[prop(optional)]
    position_x: Option<Callback<UseElementBoundingReturnReadOnly, String>>,

    /// Custom Y position of the popover.
    #[prop(optional)]
    position_y: Option<Callback<UseElementBoundingReturnReadOnly, String>>,

    #[prop(into, optional)] show: Option<MaybeSignal<bool>>,

    popover_content: PopoverContent,

    children: Children,
) -> impl IntoView {
    let (clicked, set_clicked) = create_signal(false);

    let ctx = expect_context::<PopoverRootContext>();

    let el: NodeRef<leptos::html::Custom> = create_node_ref();
    let el_bounds = use_element_bounding(el);

    let pop_el: NodeRef<leptos::html::Custom> = create_node_ref();
    let pop_bounds = use_element_bounding(pop_el);

    let show: MaybeSignal<bool> = match show {
        Some(show) => show,
        None => {
            let is_hovered = use_element_hover(el);
            MaybeSignal::derive(move || is_hovered.get() || clicked.get())
        }
    };

    let x = move || match align_x {
        PopoverAlignX::Left => el_bounds.x.get(),
        PopoverAlignX::Center => {
            el_bounds.x.get() + (el_bounds.width.get() / 2.0) - (pop_bounds.width.get() / 2.0)
        }
        PopoverAlignX::Right => el_bounds.x.get(),
    };

    let y = move || match align_y {
        PopoverAlignY::Top => el_bounds.y.get() - pop_bounds.height.get(),
        PopoverAlignY::Center => el_bounds.y.get(),
        PopoverAlignY::Bottom => el_bounds.y.get(),
    };

    let left = move || match align_x {
        PopoverAlignX::Left => format!("calc({}px - {})", x(), margin),
        PopoverAlignX::Center => format!("{}px", x()),
        PopoverAlignX::Right => format!("calc({}px + {})", x(), margin),
    };

    let top = move || match align_y {
        PopoverAlignY::Top => format!("calc({}px - {})", y(), margin),
        PopoverAlignY::Center => format!("{}px", y()),
        PopoverAlignY::Bottom => format!("calc({}px + {})", y(), margin),
    };

    let pop_bounds_read_only: UseElementBoundingReturnReadOnly = pop_bounds.into();

    let left = move || match position_x {
        Some(pos_x) => pos_x.call(pop_bounds_read_only),
        None => left(),
    };

    let top = move || match position_y {
        Some(pos_y) => pos_y.call(pop_bounds_read_only),
        None => top(),
    };

    let pop_style = move || format!("left: {}; top: {};", left(), top());

    ctx.popovers.update(move |popovers| {
        popovers.push(PopoverData {
            key: uuid::Uuid::now_v7(),
            children: Rc::new(move || {
                view! {
                    <leptonic-popover ref=pop_el style=pop_style data-active=move || match show.get() { true => "true", false => "false" }> // id=id class=class style=style
                        { (popover_content.children)() }
                    </leptonic-popover>
                }
                .into_view()
                .into()
            }),
        })
    });

    view! {
        <leptonic-has-popover ref=el id=id class=class style=style on:click=move |_| set_clicked.set(!clicked.get_untracked())>
            { children() }
        </leptonic-has-popover>
    }
}
