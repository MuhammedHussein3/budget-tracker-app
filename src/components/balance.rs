use yew::prelude::*;
use crate::models::state::State;
use crate::models::transaction::TransactionType;


#[derive(Properties, PartialEq)]
pub struct BalanceProps {
    pub state: UseReducerHandle<State>,
}

fn calculate_balance(transactions: &[crate::models::transaction::Transaction]) -> f64 {
    transactions.iter().fold(0.0, |acc, tx|{
        match tx.transaction_type {
            TransactionType::Income => acc + tx.amount,
            TransactionType::Expense => acc + tx.amount,
        }
    })
}


#[function_component(Balance)]
pub fn balance(props: &BalanceProps) -> Html {
    let total_balance = calculate_balance(&props.state.transactions);
    let balance_class = if total_balance >= 0.0 {
        "text-green-600"
    } else {
        "text-red-600"
    };
    

    html! {
        <section class="mt-4">
            <h2 class="text-xl font-bold mb-2 text-gray-800">{"Balance"}</h2>
            <p class={balance_class}>
                {format!("${:.2}", total_balance)}
            </p>
        </section>
    }
}



