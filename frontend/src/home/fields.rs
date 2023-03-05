use common::Tags;
use web_sys::HtmlInputElement;
use yew::prelude::*;

pub enum FieldMsg {
    Update(String),
}

#[derive(PartialEq, Properties)]
pub struct AccountPickerProps {
    pub account_list: Vec<String>,
    pub given_account: String,
    pub on_input: Callback<String>,
}

pub struct AccountPicker;

impl Component for AccountPicker {
    type Message = FieldMsg;
    type Properties = AccountPickerProps;

    fn create(_ctx: &Context<Self>) -> Self {
        Self {}
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            FieldMsg::Update(a) => ctx.props().on_input.emit(a),
        };
        true
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let account_list_html: Html = ctx
            .props()
            .account_list
            .iter()
            .map(|a| {
                html! {
                    <option id={a.clone()}>{a.clone()}</option>
                }
            })
            .collect();

        html! {
            <select
                class="form-control"
                id="account"
                value={ctx.props().given_account.clone()}
                oninput={ ctx.link().callback(|e: InputEvent| {
                    let input = e.target_unchecked_into::<HtmlInputElement>();
                    FieldMsg::Update(input.value())
                }) }
                >
                <option value="default" selected=true>{"Select an account"}</option>
                {account_list_html}
            </select>
        }
    }
}

#[derive(PartialEq, Properties)]
pub struct DatePickerProps {
    pub given_date: String,
    pub on_input: Callback<String>,
}

pub struct DatePicker;

impl Component for DatePicker {
    type Message = FieldMsg;
    type Properties = DatePickerProps;

    fn create(_ctx: &Context<Self>) -> Self {
        Self {}
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            FieldMsg::Update(d) => ctx.props().on_input.emit(d),
        };
        true
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
            <input
                type="date"
                class="form-control"
                id="date"
                value={ctx.props().given_date.clone()}
                oninput={ ctx.link().callback(|e: InputEvent| {
                    let input = e.target_unchecked_into::<HtmlInputElement>();
                    FieldMsg::Update(input.value())
                }) }
            />
        }
    }
}

#[derive(PartialEq, Properties)]
pub struct DescriptionProps {
    pub given_description: String,
    pub on_input: Callback<String>,
}

pub struct DescriptionField;

impl Component for DescriptionField {
    type Message = FieldMsg;
    type Properties = DescriptionProps;

    fn create(_ctx: &Context<Self>) -> Self {
        Self {}
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            FieldMsg::Update(d) => ctx.props().on_input.emit(d),
        };
        true
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
            <input
                class="form-control"
                id="description"
                required=true
                value={ctx.props().given_description.clone()}
                oninput={ ctx.link().callback(|e: InputEvent| {
                    let input = e.target_unchecked_into::<HtmlInputElement>();
                    FieldMsg::Update(input.value())
                }) }
            />
        }
    }
}

#[derive(PartialEq, Properties)]
pub struct AmountProps {
    pub given_amount: String,
    pub on_input: Callback<String>,
}

pub struct AmountField;

impl Component for AmountField {
    type Message = FieldMsg;
    type Properties = AmountProps;

    fn create(_ctx: &Context<Self>) -> Self {
        Self {}
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            FieldMsg::Update(a) => ctx.props().on_input.emit(a),
        };
        true
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
            <input
                class="form-control"
                id="amount"
                required=true
                value={ctx.props().given_amount.clone()}
                oninput={ ctx.link().callback(|e: InputEvent| {
                    let input = e.target_unchecked_into::<HtmlInputElement>();
                    FieldMsg::Update(input.value())
                }) }
            />
        }
    }
}

#[derive(PartialEq, Properties)]
pub struct TagPickerProps {
    pub tags: Tags,
    pub given_tags: (String, String, String),
    pub on_input: Callback<(String, String, String)>,
}

pub enum TagsMsg {
    UpdateL1Tag(String),
    UpdateL2Tag(String),
    UpdateL3Tag(String),
    Update,
}

pub struct TagPicker {
    l1_tag: String,
    l2_tag: String,
    l3_tag: String,
}

impl Component for TagPicker {
    type Message = TagsMsg;
    type Properties = TagPickerProps;

    fn create(ctx: &Context<Self>) -> Self {
        let (l1_tag, l2_tag, l3_tag) = ctx.props().given_tags.clone();
        Self {
            l1_tag,
            l2_tag,
            l3_tag,
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            TagsMsg::Update => ctx.props().on_input.emit((
                self.l1_tag.clone(),
                self.l2_tag.clone(),
                self.l3_tag.clone(),
            )),
            TagsMsg::UpdateL1Tag(t) => {
                self.l1_tag = t;
                ctx.link().send_message(TagsMsg::Update);
            }
            TagsMsg::UpdateL2Tag(t) => {
                self.l2_tag = t;
                ctx.link().send_message(TagsMsg::Update);
            }
            TagsMsg::UpdateL3Tag(t) => {
                self.l3_tag = t;
                ctx.link().send_message(TagsMsg::Update);
            }
        };
        true
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let l1_tag_html: Html = ctx
            .props()
            .tags
            .0
            .keys()
            .map(|k| {
                html! {
                    <option id={k.clone()}>{k.clone()}</option>
                }
            })
            .collect();

        let l2_tag_list = ctx.props().tags.0.get(&self.l1_tag);
        let l2_tag_html = match l2_tag_list {
            Some(level_2) => level_2
                .keys()
                .map(|k| {
                    html! {
                        <option id={k.clone()}>{k.clone()}</option>
                    }
                })
                .collect(),
            None => html! {<></>},
        };

        let l3_tag_list = match l2_tag_list {
            Some(t) => t.get(&self.l2_tag),
            None => None,
        };
        let l3_tag_html = match l3_tag_list {
            Some(level_3) => level_3
                .iter()
                .map(|t| {
                    html! {
                        <option id={t.clone()}>{t.clone()}</option>
                    }
                })
                .collect(),
            None => html! {<></>},
        };

        html! {
            <>
            <td>
            <select
                class="form-control"
                id="l1_tag"
                value={self.l1_tag.clone()}
                oninput={ ctx.link().callback(|e: InputEvent| {
                    let input = e.target_unchecked_into::<HtmlInputElement>();
                    TagsMsg::UpdateL1Tag(input.value())
                }) }
                >
                {l1_tag_html}
            </select>
            </td>
            <td>
            <select
                class="form-control"
                id="l2_tag"
                value={self.l2_tag.clone()}
                oninput={ ctx.link().callback(|e: InputEvent| {
                    let input = e.target_unchecked_into::<HtmlInputElement>();
                    TagsMsg::UpdateL2Tag(input.value())
                }) }
                >
                {l2_tag_html}
            </select>
            </td>
            <td>
            <select
                class="form-control"
                id="l3_tag"
                value={self.l3_tag.clone()}
                oninput={ ctx.link().callback(|e: InputEvent| {
                    let input = e.target_unchecked_into::<HtmlInputElement>();
                    TagsMsg::UpdateL3Tag(input.value())
                }) }
                >
                {l3_tag_html}
            </select>
            </td>
            </>
        }
    }
}
