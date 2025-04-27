use yew::prelude::*;

use crate::models::state::State;


pub struct FilterProps {
    pub state: UseReducerHandle<State>,
}

#[function_component(Filter)]
pub fn filter() -> Html {
    html! {
        <div>{"Filter Placeholder"}</div>
    }
}