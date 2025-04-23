use yew::prelude::*;                        
use crate::models::state::{Action, State};  
use crate::models::transaction::{Transaction, TransactionType}; 


#[derive(Properties, PartialEq)]
pub struct TransactionListProps {
    pub state: UseReducerHandle<State>,
}

#[function_component(TransactionList)]
pub fn transaction_list(props: &TransactionListProps) -> Html {
    let state = props.state.clone();

    html! {
        <div class="mt-4">
            <h2 class="text-xl font-bold mb-2 text-gray-800">{"Transactions"}</h2>
            <ul class="list-disc pl-5">
                { for state.transactions.iter().map(|t| {
                    let state = state.clone(); 
                    let id = t.id; 
                    let onclick = Callback::from(move |_| state.dispatch(Action::Remove(id)));
                    html! {
                        <li key={t.id} class="mb-1">
                            {format!(
                                "{}: ${:.2} - {} ({})",
                                t.description,
                                t.amount,
                                t.category,
                                match t.transaction_type {
                                    TransactionType::Income => "Income",
                                    TransactionType::Expense => "Expense",
                                }
                            )}
                            <button
                                onclick={onclick}
                                class="ml-2 text-red-500 hover:text-red-700"
                            >
                                {"Delete"}
                            </button>
                        </li>
                    }
                })}
            </ul>
        </div>
    }
}

