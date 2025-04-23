use yew::prelude::*; 
use gloo::console::log;                      
use web_sys::{HtmlInputElement, HtmlSelectElement, Event};
use crate::models::state::{Action, State};   
use crate::models::transaction::{Transaction, TransactionType}; 

#[derive(Properties, PartialEq)]
pub struct TransactionFormProps {
    pub state: UseReducerHandle<State>, 
}

pub fn transaction_type_selected(value: String) -> TransactionType {
    if value == "Income"  {
        return TransactionType::Income;
    }else {
        return TransactionType::Expense;
    }
}

pub fn handle_input_change(setter: UseStateHandle<String>) -> Callback<InputEvent> {
    Callback::from(move |e: InputEvent| {
        let input: HtmlInputElement = e.target_unchecked_into();
        setter.set(input.value());
    })
}


pub fn handle_transaction_type_change(setter: UseStateHandle<TransactionType>) -> Callback<Event> {
    Callback::from(move |e: Event| {
        let select: HtmlSelectElement = e.target_unchecked_into();
        let value = select.value();
        setter.set(transaction_type_selected(value));
    })
}

pub fn build_input_handlers(
    description: UseStateHandle<String>,
    amount: UseStateHandle<String>,
    category: UseStateHandle<String>,
) -> (Callback<InputEvent>, Callback<InputEvent>, Callback<InputEvent>) {
    (
        handle_input_change(description),
        handle_input_change(amount),
        handle_input_change(category),
    )
}


#[function_component(TransactionForm)]
pub fn transaction_form(props: &TransactionFormProps) -> Html {
    let description = use_state(|| String::new()); 
    let amount = use_state(|| String::new());      
    let category = use_state(|| String::new());    
    let transaction_type = use_state(|| TransactionType::Expense); 
    let error = use_state(|| String::new());

    let on_submit = {
        let state = props.state.clone();
        let description = description.clone();
        let amount = amount.clone();
        let category = category.clone();
        let transaction_type = transaction_type.clone();
        let error = error.clone();
    
        Callback::from(move |e: SubmitEvent| {
            e.prevent_default();
            log!("Submit triggered");

            let desc = (*description).trim();
            let amt_str = (*amount).trim();
            let cat = (*category).trim();
            let tx_type = (*transaction_type).clone();
    
            if desc.is_empty() {
                log!("Missing description");
                error.set("Description is required".into());
                return;
            }
    
            if amt_str.is_empty() {
                log!("Missing amount");
                error.set("Amount is required".into());
                return;
            }
    
            let Ok(amt_value) = amt_str.parse::<f64>() else {
                log!("Invalid amount format: {:?}", amt_str);
                error.set("Amount must be a valid number".into());
                return;
            };
    
            if amt_value <= 0.0 {
                log!("Non-positive amount: {}", amt_value);
                error.set("Amount must be greater than 0".into());
                return;
            }
    
            if cat.is_empty() {
                log!("Missing category");
                error.set("Category is required".into());
                return;
            }
            
            log!("Valid input, dispatching transaction...");
            state.dispatch(Action::Add(Transaction {
                id: state.next_id,
                description: desc.into(),
                amount: amt_value,
                category: cat.into(),
                transaction_type: tx_type,
            }));
            log!("Transaction added: {} - {}", amt_value, cat);

            description.set(String::new());
            amount.set(String::new());
            category.set(String::new());
            transaction_type.set(TransactionType::Expense);
            error.set(String::new());
        })
    };

    let (on_description_change, on_amount_change, on_category_change) =
    build_input_handlers(description.clone(), amount.clone(), category.clone());

    let on_type_change = handle_transaction_type_change(transaction_type.clone());
    
    html! {
        <form onsubmit={on_submit} class="mb-4">
            <div class="mb-2">
                <label class="block text-gray-700">{"Description"}</label>
                <input
                    type="text"
                    value={(*description).clone()}
                    oninput={on_description_change}
                    class="w-full p-2 border rounded"
                    placeholder="Enter description"
                />
            </div>
            <div class="mb-2">
                <label class="block text-gray-700">{"Amount"}</label>
                <input
                    type="number"
                    value={(*amount).clone()}
                    oninput={on_amount_change}
                    class="w-full p-2 border rounded"
                    placeholder="Enter amount"
                    step="0.01"
                />
            </div>
            <div class="mb-2">
                <label class="block text-gray-700">{"Category"}</label>
                <input
                    type="text"
                    value={(*category).clone()}
                    oninput={on_category_change}
                    class="w-full p-2 border rounded"
                    placeholder="Enter category"
                />
            </div>
            <div class="mb-2">
                <label class="block text-gray-700">{"Type"}</label>
                <select
                    onchange={on_type_change}
                    class="w-full p-2 border rounded"
                >
                    <option value="Expense" selected={*transaction_type == TransactionType::Expense}>{"Expense"}</option>
                    <option value="Income" selected={*transaction_type == TransactionType::Income}>{"Income"}</option>
                </select>
            </div>
            <div class="mb-2">
                { if !(*error).is_empty() {
                    html! {
                        <p class="text-red-500">{(*error).clone()}</p>
                    }
                } else {
                    html! {}
                }}
            </div>
            <button
                type="submit"
                class="w-full bg-blue-500 text-white p-2 rounded hover:bg-blue-600"
            >
                {"Add Transaction"}
            </button>
        </form>
    }
}

