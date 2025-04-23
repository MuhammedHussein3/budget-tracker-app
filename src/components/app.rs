use yew::prelude::*;                        
use crate::models::state::{State, Filter}; 
use crate::storage::local_storage::{load_state, save_state}; 
use crate::components::transaction_form::TransactionForm; 
use crate::components::transaction_list::TransactionList; 
use crate::components::balance::Balance; 

const STORAGE_KEY: &str = "budget_tracker";

#[function_component(App)]
pub fn app() -> Html {
    let state = use_reducer(|| {
        load_state(STORAGE_KEY).unwrap_or_else(|| State {
            transactions: vec![], 
            filter: Filter::All,  
            next_id: 1,           
        })
    });

    {
        let state_for_effect = state.clone(); 
        use_effect_with(
            (*state).clone(), 
            move |_| {
                save_state(STORAGE_KEY, &*state_for_effect).unwrap_or_else(|e| {
                    gloo::console::error!(format!("Failed to save: {}", e));
                });
                || () 
            },
        );
    }

    html! {
        <div class="max-w-lg mx-auto p-4 bg-white rounded-lg shadow-md">
            <h1 class="text-2xl font-bold text-center mb-4 text-gray-800">{"Budget Tracker"}</h1>
            <TransactionForm state={state.clone()} />
            <TransactionList state={state.clone()} />
            <Balance state={state.clone()} />
            <p>{"Number of transactions: "}{(*state).transactions.len()}</p>
        </div>
    }
}