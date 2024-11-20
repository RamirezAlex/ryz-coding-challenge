enum TransactionType {
    Deposit,
    Withdrawal,
}

struct Transaction {
    transaction_type: TransactionType,
    wallet_address: String,
    amount: i64,
}

fn calculate_wallet_balance(wallet_address: &str, transactions: &[Transaction]) -> i64 {
    transactions
        .iter()
        .filter(|transaction| transaction.wallet_address == wallet_address)
        .fold(0, |balance, transaction| match transaction.transaction_type {
            TransactionType::Deposit => balance + transaction.amount,
            TransactionType::Withdrawal => balance - transaction.amount,
        })
}

fn main() {
    let transactions = vec![
        Transaction {
            transaction_type: TransactionType::Deposit,
            wallet_address: "ALiCEqZUF4VYuxTu1UQvzDqbpGYYFrxH6kQxWFB8Nqp3".to_string(),
            amount: 100,
        },
        Transaction {
            transaction_type: TransactionType::Withdrawal,
            wallet_address: "ALiCEqZUF4VYuxTu1UQvzDqbpGYYFrxH6kQxWFB8Nqp3".to_string(),
            amount: 50,
        },
        Transaction {
            transaction_type: TransactionType::Deposit,
            wallet_address: "BoBbXD8yD1oqPv3J4nwj1mQh4kCrBVbNF6W1FVPrz4sq".to_string(),
            amount: 200,
        },
        Transaction {
            transaction_type: TransactionType::Withdrawal,
            wallet_address: "BoBbXD8yD1oqPv3J4nwj1mQh4kCrBVbNF6W1FVPrz4sq".to_string(),
            amount: 75,
        },
        Transaction {
            transaction_type: TransactionType::Deposit,
            wallet_address: "ALiCEqZUF4VYuxTu1UQvzDqbpGYYFrxH6kQxWFB8Nqp3".to_string(),
            amount: 150,
        },
    ];

    let wallet_address = "ALiCEqZUF4VYuxTu1UQvzDqbpGYYFrxH6kQxWFB8Nqp3";
    let balance = calculate_wallet_balance(wallet_address, &transactions);
    println!("Balance for {}: {}", wallet_address, balance);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_calculate_wallet_balance() {
        let txs = vec![
            Transaction {
                transaction_type: TransactionType::Deposit,
                wallet_address: "ALiCEqZUF4VYuxTu1UQvzDqbpGYYFrxH6kQxWFB8Nqp3".to_string(),
                amount: 100,
            },
            Transaction {
                transaction_type: TransactionType::Withdrawal,
                wallet_address: "ALiCEqZUF4VYuxTu1UQvzDqbpGYYFrxH6kQxWFB8Nqp3".to_string(),
                amount: 50,
            },
            Transaction {
                transaction_type: TransactionType::Deposit,
                wallet_address: "BoBbXD8yD1oqPv3J4nwj1mQh4kCrBVbNF6W1FVPrz4sq".to_string(),
                amount: 200,
            },
            Transaction {
                transaction_type: TransactionType::Withdrawal,
                wallet_address: "BoBbXD8yD1oqPv3J4nwj1mQh4kCrBVbNF6W1FVPrz4sq".to_string(),
                amount: 75,
            },
            Transaction {
                transaction_type: TransactionType::Deposit,
                wallet_address: "ALiCEqZUF4VYuxTu1UQvzDqbpGYYFrxH6kQxWFB8Nqp3".to_string(),
                amount: 150,
            },
        ];

        assert_eq!(calculate_wallet_balance("ALiCEqZUF4VYuxTu1UQvzDqbpGYYFrxH6kQxWFB8Nqp3", &txs), 200);
        assert_eq!(calculate_wallet_balance("BoBbXD8yD1oqPv3J4nwj1mQh4kCrBVbNF6W1FVPrz4sq", &txs), 125);
    }
}
