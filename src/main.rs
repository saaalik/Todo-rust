#![recursion_limit = "640"]

use yew::prelude::*;

enum Msg {
    Add,
    Update(String),
    Remove(usize),
    RemoveAll,
    Nothing,
}



struct Model {
    link: ComponentLink<Self>,
    input: String,
    todos: Vec<String>,
}

impl Component for Model {
    type Message = Msg;
    type Properties = ();

    fn create(_props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {
            link,
            value: 0,
            todos: vec![],
            input: "".to_string(),
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::Add => {
                let t = self.input.clone();
                self.todos.push(t);
                self.input = "".to_string();
                println!("{:?}", self.todos);
                true
            }
            Msg::Update(s) => {
                self.input = s;
                true
            }
            Msg::Remove(i) => {
                self.todos.remove(i);
                true
            }
            Msg::RemoveAll => {
                self.todos = vec![];
                true
            }
            Msg::Nothing => {true}
        }
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        let view_todo = |(i, todo): (usize, &String)| {
            html!{
                <li>
                    { todo }
                    <button onclick = self.link.callback(move |_| Msg::Remove(i))>{"X"}</button>
                </li>
            }
        };
        html! {
            <div class="main">
                <h1>{"TO DO APP"}</h1>
                <div style="margin:0px auto">
                <input
                    placeholder="Add to-do item..."
                    oninput=self.link.callback(|e: InputData| Msg::Update(e.value))
                    onkeypress= self.link.callback(|e: KeyboardEvent| {
                        if e.key() == "Enter" {Msg::Add} else {Msg::Nothing}
                    })
                    />
                <button onclick=self.link.callback(|_| Msg::Add)>{"Enter"}</button>
                <button onclick=self.link.callback(|_| Msg::RemoveAll)>{"Clear"}</button>
                </div>
                <ul>
                {for self.todos.iter().enumerate().map(view_todo)}
                </ul>
            </div>
        }
    }
}

fn main() {
    yew::start_app::<Model>();
}