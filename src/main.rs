mod dict;
mod macros;
mod word_component;
use dictmacro::conjugate;
use word_component::WordComponent;
use yew::{html, Component};

enum Msg {
    Search(String),
}

struct Model {
    search_text: String,
}

impl Component for Model {
    type Message = Msg;
    type Properties = ();

    fn create(_props: Self::Properties, _linkk: yew::ComponentLink<Self>) -> Self {
        Self {
            search_text: "".to_string(),
        }
    }

    fn update(&mut self, msg: Self::Message) -> yew::ShouldRender {
        match msg {
            Msg::Search(search_text) => {
                self.search_text = search_text;
                true
            }
        }
    }

    fn change(&mut self, _props: Self::Properties) -> yew::ShouldRender {
        false
    }

    fn view(&self) -> yew::Html {
        let words = dict::WORDS
            .iter()
            .filter(|word| word.word().english_definition.contains(&self.search_text))
            .map(|word_type| {
                html! {
                    <li><WordComponent word_type={word_type}/></li>
                }
            });
        html! {
            <ul>
            {for words}
            </ul>
        }
    }
}

fn main() {
    yew::start_app::<Model>();
}
