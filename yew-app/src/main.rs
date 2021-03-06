use yew::prelude::*;

enum Msg {
    AddOne,
    ConsumeOne,
}

struct Model {
    value: i64,
}

impl Component for Model {
    type Message = Msg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self {
            value: 0,
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::AddOne => {
                self.value += 375;
                // the value has changed so we need to
                // re-render for it to appear on the page
                true
            },
            Msg::ConsumeOne => {
                self.value -= 375;
                true
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        // This gives us a component's "`Scope`" which allows us to send messages, etc to the component.
        let link = ctx.link();
        let test_string = String::from("hello world");
        html! {
        <>
            <div>
                <button onclick={link.callback(|_| Msg::AddOne)}>{ "+1" }</button>
                <button onclick={link.callback(|_| Msg::ConsumeOne)}>{ "-1" }</button>
                <p>{self.value}</p>
                <p>{test_string.clone()}</p>
            </div>
            <div>
            <button onclick={link.callback(|_| Msg::AddOne)}>{ "+1" }</button>
            <button onclick={link.callback(|_| Msg::ConsumeOne)}>{ "-1" }</button>
            <p>{self.value}</p>
            <p>{test_string}</p>
        </div>
        </>
        }
    }
}

fn main() {
    yew::start_app::<Model>();
}
