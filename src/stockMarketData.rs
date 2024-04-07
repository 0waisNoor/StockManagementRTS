
pub const ARRAY_SIZE: usize = 60;
pub type Array = [[&str;5]; ARRAY_SIZE];
pub static mut 


pub struct Transaction {
    pub user_id: String,
    pub num_shares: u32,
    pub date: String, // You might want to use a proper date/time library here
    pub time: String,
    pub price: f64,
    pub order_type: String,
}

// Define a struct to represent stock market data
pub struct StockMarketData {
    pub stock_name: String,
    pub price: f64,
    pub increase_decrease: f64,
    pub shares_traded: u32,
    pub transactions: Vec<Transaction>, // Vector of transactions
    pub sector: String,
}

impl StockMarketData {
    // Constructor method to create new StockMarketData instances
    pub fn new(
        stock_name: String,
        price: f64,
        increase_decrease: f64,
        shares_traded: u32,
        sector: String,
    ) -> Self {
        StockMarketData {
            stock_name,
            price,
            increase_decrease,
            shares_traded,
            transactions: Vec::new(), // Initialize transactions vector
            sector,
        }
    }

    // Method to add a transaction to the StockMarketData instance
    pub fn add_transaction(
        &mut self,
        user_id: String,
        num_shares: u32,
        date: String,
        time: String,
        price: f64,
        order_type: String,
    ) {
        let transaction = Transaction {
            user_id,
            num_shares,
            date,
            time,
            price,
            order_type,
        };
        self.transactions.push(transaction);
    }
}

pub fn prepareStocks(){


    println!("{}",stocks[0][1]);
}
