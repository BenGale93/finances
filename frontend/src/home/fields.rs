use common::Tags;
use web_sys::HtmlInputElement;
use yew::prelude::*;

pub enum FieldMsg {
    Update(AttrValue),
}

#[derive(PartialEq, Properties)]
pub struct AccountPickerProps {
    pub id: AttrValue,
    pub account_list: Vec<String>,
    pub given_account: AttrValue,
    pub on_input: Callback<AttrValue>,
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
        let given_account = ctx.props().given_account.clone();
        let account_list_html: Html = ctx
            .props()
            .account_list
            .iter()
            .map(|a| {
                html! {
                    if a == given_account.as_str() {
                    <option selected=true id={a.clone()}>{a.clone()}</option>
                    } else {
                    <option id={a.clone()}>{a.clone()}</option>
                    }
                }
            })
            .collect();

        html! {
            <select
                class="form-control"
                id="account"
                form={ctx.props().id.clone()}
                value={given_account}
                onfocus={ ctx.link().callback(|e: FocusEvent| {
                    let input = e.target_unchecked_into::<HtmlInputElement>();
                    FieldMsg::Update(AttrValue::from(input.value()))
                }) }
                oninput={ ctx.link().callback(|e: InputEvent| {
                    let input = e.target_unchecked_into::<HtmlInputElement>();
                    FieldMsg::Update(AttrValue::from(input.value()))
                }) }
                >
                {account_list_html}
            </select>
        }
    }
}

#[derive(PartialEq, Properties)]
pub struct DatePickerProps {
    pub id: AttrValue,
    pub given_date: AttrValue,
    pub on_input: Callback<AttrValue>,
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
                form={ctx.props().id.clone()}
                value={ctx.props().given_date.clone()}
                oninput={ ctx.link().callback(|e: InputEvent| {
                    let input = e.target_unchecked_into::<HtmlInputElement>();
                    FieldMsg::Update(AttrValue::from(input.value()))
                }) }
            />
        }
    }
}

#[derive(PartialEq, Properties)]
pub struct DescriptionProps {
    pub id: AttrValue,
    pub given_description: AttrValue,
    pub on_input: Callback<AttrValue>,
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
                form={ctx.props().id.clone()}
                value={ctx.props().given_description.clone()}
                oninput={ ctx.link().callback(|e: InputEvent| {
                    let input = e.target_unchecked_into::<HtmlInputElement>();
                    FieldMsg::Update(AttrValue::from(input.value()))
                }) }
            />
        }
    }
}

#[derive(PartialEq, Properties)]
pub struct AmountProps {
    pub id: AttrValue,
    pub given_amount: AttrValue,
    pub on_input: Callback<AttrValue>,
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
                form={ctx.props().id.clone()}
                value={ctx.props().given_amount.clone()}
                oninput={ ctx.link().callback(|e: InputEvent| {
                    let input = e.target_unchecked_into::<HtmlInputElement>();
                    FieldMsg::Update(AttrValue::from(input.value()))
                }) }
            />
        }
    }
}

#[derive(PartialEq, Properties)]
pub struct TagPickerProps {
    pub id: AttrValue,
    pub tags: Tags,
    pub given_tags: (AttrValue, AttrValue, AttrValue),
    pub on_input: Callback<(AttrValue, AttrValue, AttrValue)>,
}

pub enum TagsMsg {
    UpdateL1Tag(AttrValue),
    UpdateL2Tag(AttrValue),
    UpdateL3Tag(AttrValue),
    Update,
}

pub struct TagPicker {
    tags: (AttrValue, AttrValue, AttrValue),
}

impl Component for TagPicker {
    type Message = TagsMsg;
    type Properties = TagPickerProps;

    fn create(ctx: &Context<Self>) -> Self {
        Self {
            tags: ctx.props().given_tags.clone(),
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            TagsMsg::Update => ctx.props().on_input.emit(self.tags.clone()),
            TagsMsg::UpdateL1Tag(t) => {
                self.tags.0 = t;
                ctx.link().send_message(TagsMsg::Update);
            }
            TagsMsg::UpdateL2Tag(t) => {
                self.tags.1 = t;
                ctx.link().send_message(TagsMsg::Update);
            }
            TagsMsg::UpdateL3Tag(t) => {
                self.tags.2 = t;
                ctx.link().send_message(TagsMsg::Update);
            }
        };
        true
    }

    fn changed(&mut self, ctx: &Context<Self>, _old_props: &Self::Properties) -> bool {
        if ctx.props().given_tags != self.tags {
            self.tags = ctx.props().given_tags.clone();
        }
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
                    if k == self.tags.0.as_str() {
                    <option selected=true id={k.clone()}>{k.clone()}</option>
                    } else {
                    <option id={k.clone()}>{k.clone()}</option>
                    }
                }
            })
            .collect();

        let l2_tag_list = ctx.props().tags.0.get(&self.tags.0.to_string());
        let l2_tag_html = match l2_tag_list {
            Some(level_2) => level_2
                .keys()
                .map(|k| {
                    html! {
                    if k == self.tags.1.as_str() {
                    <option selected=true id={k.clone()}>{k.clone()}</option>
                    } else {
                    <option id={k.clone()}>{k.clone()}</option>
                    }
                    }
                })
                .collect(),
            None => html! {<></>},
        };

        let l3_tag_list = match l2_tag_list {
            Some(t) => t.get(&self.tags.1.to_string()),
            None => None,
        };
        let l3_tag_html = match l3_tag_list {
            Some(level_3) => level_3
                .iter()
                .map(|k| {
                    html! {
                    if k == self.tags.2.as_str() {
                    <option selected=true id={k.clone()}>{k.clone()}</option>
                    } else {
                    <option id={k.clone()}>{k.clone()}</option>
                    }
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
                value={self.tags.0.clone()}
                form={ctx.props().id.clone()}
                onfocus={ ctx.link().callback(|e: FocusEvent| {
                    let input = e.target_unchecked_into::<HtmlInputElement>();
                    TagsMsg::UpdateL1Tag(AttrValue::from(input.value()))
                }) }
                oninput={ ctx.link().callback(|e: InputEvent| {
                    let input = e.target_unchecked_into::<HtmlInputElement>();
                    TagsMsg::UpdateL1Tag(AttrValue::from(input.value()))
                }) }
                >
                {l1_tag_html}
            </select>
            </td>
            <td>
            <select
                class="form-control"
                id="l2_tag"
                value={self.tags.1.clone()}
                form={ctx.props().id.clone()}
                onfocus={ ctx.link().callback(|e: FocusEvent| {
                    let input = e.target_unchecked_into::<HtmlInputElement>();
                    TagsMsg::UpdateL2Tag(AttrValue::from(input.value()))
                }) }
                oninput={ ctx.link().callback(|e: InputEvent| {
                    let input = e.target_unchecked_into::<HtmlInputElement>();
                    TagsMsg::UpdateL2Tag(AttrValue::from(input.value()))
                }) }
                >
                {l2_tag_html}
            </select>
            </td>
            <td>
            <select
                class="form-control"
                id="l3_tag"
                value={self.tags.2.clone()}
                form={ctx.props().id.clone()}
                onfocus={ ctx.link().callback(|e: FocusEvent| {
                    let input = e.target_unchecked_into::<HtmlInputElement>();
                    TagsMsg::UpdateL3Tag(AttrValue::from(input.value()))
                }) }
                oninput={ ctx.link().callback(|e: InputEvent| {
                    let input = e.target_unchecked_into::<HtmlInputElement>();
                    TagsMsg::UpdateL3Tag(AttrValue::from(input.value()))
                }) }
                >
                {l3_tag_html}
            </select>
            </td>
            </>
        }
    }
}
