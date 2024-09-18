use leptos::*;
use leptos_tiptap::*;

use crate::{
    components::{
        button::{Button, ButtonSize},
        icon::Icon,
    },
    OptMaybeSignal, Out,
};

#[component]
pub fn TiptapEditor(
    #[prop(into, optional)] id: Option<AttributeValue>,
    #[prop(into, optional)] class: Option<AttributeValue>,
    #[prop(into, optional)] disabled: OptMaybeSignal<bool>,
    #[prop(into)] value: Signal<String>,
    #[prop(into, optional)] set_value: Option<Out<TiptapContent>>,
) -> impl IntoView {
    let (msg, set_msg) = create_signal(TiptapInstanceMsg::Noop);

    let (selection_state, set_selection_state) = create_signal(TiptapSelectionState::default());

    let instance_id = uuid::Uuid::now_v7();

    view! {
        <leptonic-tiptap-editor id=id class=class>
            <TiptapInstance
                id=instance_id.to_string()
                msg=msg
                disabled=match disabled.0 {
                    Some(sig) => sig,
                    None => MaybeSignal::Static(false),
                }
                value=value
                set_value=move |v| {
                    if let Some(set_value) = &set_value {
                        set_value.set(v);
                    }
                }
                on_selection_change=move |state| set_selection_state.set(state)
            />
        </leptonic-tiptap-editor>
    }
}

/*

#[derive(Properties, PartialEq)]
pub struct Props {
    pub api_base_url: String,
    pub id: String,
    pub value: String,
    pub class: String,
    pub disabled: bool,
    pub onchange: Option<Callback<String>>,
}

pub struct CrudTipTapEditor {
    link: Option<Scope<TiptapInstance>>,
    choose_image: bool,
    selection_state: SelectionState,
}

choose_image: false,

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::InstanceLinked(link) => {
                self.link = link;
                false
            }
            Msg::SelectionChanged(selection) => {
                self.selection_state = selection.state;
                false
            }
            Msg::ContentChanged(content) => {
                if let Some(onchange) = &ctx.props().onchange {
                    onchange.emit(content.content);
                }
                false
            }
            Msg::ChooseImage => {
                // Enables the chooser modal!
                self.choose_image = true;
                true
            }
            Msg::ChooseImageCanceled => {
                self.choose_image = false;
                true
            }
            Msg::ImageChosen(resource) => {
                self.choose_image = false;
                self.send_tiptap_msg(TiptapInstanceMsg::SetImage(resource));
                true
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
            <div class={classes!("tiptap-editor", ctx.props().disabled.then(|| "disabled"))}>

                <div class={"tiptap-menu"}>

                    <div class={"tiptap-btn"} onclick={ctx.link().callback(|_| Msg::ChooseImage)}>
                        <CrudIcon variant={Bi::Image}/>
                        {"image"}
                    </div>

                </div>

                // This is our TipTap instance!
                <TiptapInstance
                    id={ctx.props().id.clone()}
                    class={"tiptap-instance".to_owned()}
                    content={ctx.props().value.clone()}
                    disabled={ctx.props().disabled}
                    on_link={ctx.link().callback(|link: Option<Scope<TiptapInstance>>| Msg::InstanceLinked(link))}
                    on_selection_change={ctx.link().callback(Msg::SelectionChanged)}
                    on_content_change={ctx.link().callback(Msg::ContentChanged)}
                />

                {
                    match &self.choose_image {
                        true => html! {
                            <CrudModal>
                                <CrudImageChooserModal
                                    api_base_url={ctx.props().api_base_url.clone()}
                                    on_cancel={ctx.link().callback(|_| Msg::ChooseImageCanceled)}
                                    on_choose={ctx.link().callback(|res: FileResource| Msg::ImageChosen(ImageResource {
                                        title: res.name.clone(),
                                        alt: res.name,
                                        url: res.path,
                                    }))}
                                />
                            </CrudModal>
                        },
                        false => html! {}
                    }
                }
            </div>
        }
    }
}
 */
