use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, PartialEq, Debug)]
pub enum TransactionType {
    Income,  
    Expense, 
}

#[derive(Serialize, Deserialize, Clone, PartialEq, Debug)]
pub struct Transaction {
    pub id: i32,                      
    pub description: String,          
    pub amount: f64,                  
    pub category: String,             
    pub transaction_type: TransactionType
}