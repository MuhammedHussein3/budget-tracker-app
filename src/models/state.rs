use serde::{Deserialize, Serialize};       
use yew::functional::Reducible;           
use crate::models::transaction::Transaction; 

#[derive(Serialize, Deserialize, Clone, PartialEq)]
pub enum Filter {
    All,             
    Category(String)
}

#[derive(Serialize, Deserialize, Clone, PartialEq)]
pub struct State {
    pub transactions: Vec<Transaction>, 
    pub filter: Filter,                
    pub next_id: i32                 
}

#[derive(Clone)]
pub enum Action {
    Add(Transaction), 
    Remove(i32)     
}

impl Reducible for State {
    type Action = Action; 

    fn reduce(self: std::rc::Rc<Self>, action: Self::Action) -> std::rc::Rc<Self> {
        let mut next_state = (*self).clone(); 
        match action {
            Action::Add(transaction) => {
                if !transaction.description.trim().is_empty() && transaction.amount > 0.0 {
                    next_state.transactions.push(transaction); 
                    next_state.next_id += 1;                  
                }
            }
            Action::Remove(id) => {
                next_state.transactions.retain(|t| t.id != id);
            }
        }
        std::rc::Rc::new(next_state) 
    }
}