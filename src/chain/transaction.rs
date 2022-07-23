
pub type Balance = u8;

pub  struct Transaction {
    id: String,
    from: Address,
    to: Address,
    amount: Balance,
    time: i64,
    transaction_type: TransactionType
}

 struct Address {
    private_key: String,
    public_key: String
}

pub enum TransactionType {
    Token,
    Nft,
    Contract,
    Bridge
}