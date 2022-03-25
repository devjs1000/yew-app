use yew::prelude::*;

enum Msg {
    AddOne,
}

struct CounterComponent {
    count: i64,
}

impl Component for CounterComponent {
    type Message = Msg;
    type Properties = ();
    fn create(_ctx: &Context<Self>) -> Self {
        Self { count: 0 }
    }
    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::AddOne => {
                self.count += 2;
                true //re renders the component
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let link = ctx.link();
        html! {
                   <div class="container" style="background:rgba(200,200,200,.6); height:30rem; padding:5rem; display:flex; justify-content:center; align-items:center; ">
                   // <p> {"you clicked it "} {self.count} {"times"} </p>
                   // <button onclick={link.callback(|_| Msg::AddOne)}>{"+1"}</button>

                   <div style="background:white; padding:5rem; border-radius:1rem; display:flex; justify-content:center; align-items:center; flex-wrap:wrap; width:20rem;"  >
                   <input placeholder="username" style="padding:1rem; margin-top:.2rem;width:100%;" type="email" />
                   <input placeholder="password" style="padding:1rem; margin-top:.2rem;width:100%;" type="password" />
                       <button onclick={link.callback(|_| Msg::AddOne)} style="padding:1rem; margin-top:.2rem;width:100%;">{"login"}</button>
                   </div>
                   </div>
               }
    }
}

fn main() {
    yew::start_app::<CounterComponent>();
}
