//! A simple wallet transaction processing system that handles deposits and withdrawals.
//! This module provides functionality to calculate wallet balances based on transaction history.

use thiserror::Error;
use regex::Regex;

/// Represents possible errors that can occur during transaction processing
#[derive(Error, Debug)]
pub enum TransactionError {
    /// Returned when the wallet address is invalid or empty
    #[error("Invalid wallet address: {0}")]
    InvalidWalletAddress(String),
    /// Returned when a transaction amount is zero
    #[error("Amount cannot be zero")]
    ZeroAmount,
    #[error("No transactions found for wallet {0}")]
    NoTransactions(String),
}

/// Represents the type of transaction
#[derive(Debug, Clone, PartialEq)]
enum TransactionType {
    /// Adds funds to the wallet
    Deposit,
    /// Removes funds from the wallet
    Withdrawal,
}

/// Represents a single transaction with its associated data
#[derive(Debug, Clone)]
struct Transaction {
    /// The type of transaction (Deposit or Withdrawal)
    transaction_type: TransactionType,
    /// The wallet address associated with the transaction
    wallet_address: String,
    /// The amount of the transaction (must be non-zero)
    amount: i64,
}

/// Validates a Solana wallet address format
/// it uses the Solana address format
///
/// # Arguments
///
/// * `address` - The address to validate
///
/// # Returns
///
/// * `bool` - True if the address is valid, false otherwise
/// 
fn is_valid_solana_address(address: &str) -> bool {
    // Basic Solana address validation (alphanumeric, 32-44 chars)
    let re = Regex::new(r"^[1-9A-HJ-NP-Za-km-z]{32,44}$").unwrap();
    re.is_match(address)
}

/// Calculates the current balance for a given wallet address based on its transaction history
/// it uses the Solana address format
///
/// # Arguments
///
/// * `wallet_address` - The address of the wallet to calculate the balance for
/// * `transactions` - A slice of transactions to process
///
/// # Returns
///
/// * `Ok(i64)` - The calculated balance if successful
/// * `Err(TransactionError)` - If there's an error processing the transactions
///
fn calculate_wallet_balance(wallet_address: &str, transactions: &[Transaction]) -> Result<i64, TransactionError> {
    // Validate wallet address
    if wallet_address.is_empty() {
        return Err(TransactionError::InvalidWalletAddress("Empty address".to_string()));
    }

    // Validate Solana address format
    if !is_valid_solana_address(wallet_address) {
        return Err(TransactionError::InvalidWalletAddress(wallet_address.to_string()));
    }

    // Check if there are any transactions list is empty
    if transactions.is_empty() {
        return Err(TransactionError::NoTransactions(wallet_address.to_string()));
    }

    // Process transactions and calculate balance
    transactions
        .iter()
        .filter(|tx| tx.wallet_address == wallet_address)
        .try_fold(0i64, |acc, tx| {
            // Validate transaction amount
            if tx.amount == 0 {
                return Err(TransactionError::ZeroAmount);
            }

            // Update balance based on transaction type
            match tx.transaction_type {
                TransactionType::Deposit => Ok(acc + tx.amount),
                TransactionType::Withdrawal => Ok(acc - tx.amount),
            }
        })
}

fn main() {
    // Example transactions
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
            wallet_address: "BOBqZUF4VYuxTu1UQvzDqbpGYYFrxH6kQxWFB8Nqp3".to_string(), 
            amount: 200,
        },
        Transaction {
            transaction_type: TransactionType::Withdrawal,
            wallet_address: "BOBqZUF4VYuxTu1UQvzDqbpGYYFrxH6kQxWFB8Nqp3".to_string(),
            amount: 75,
        },
        Transaction {
            transaction_type: TransactionType::Deposit,
            wallet_address: "ALiCEqZUF4VYuxTu1UQvzDqbpGYYFrxH6kQxWFB8Nqp3".to_string(),
            amount: 25,
        },
    ];

    // Calculate and display balance
    match calculate_wallet_balance("ALiCEqZUF4VYuxTu1UQvzDqbpGYYFrxH6kQxWFB8Nqp3", &transactions) {
        Ok(balance) => println!("Balance for ALiCEqZUF4VYuxTu1UQvzDqbpGYYFrxH6kQxWFB8Nqp3: {}", balance),
        Err(e) => eprintln!("Error calculating balance: {}", e),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Tests basic balance calculation with deposit and withdrawal
    #[test]
    fn test_calculate_wallet_balance() {
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
                wallet_address: "BOBqZUF4VYuxTu1UQvzDqbpGYYFrxH6kQxWFB8Nqp3".to_string(),
                amount: 200,
            },
            Transaction {
                transaction_type: TransactionType::Withdrawal,
                wallet_address: "BOBqZUF4VYuxTu1UQvzDqbpGYYFrxH6kQxWFB8Nqp3".to_string(),
                amount: 75,
            },
            Transaction {
                transaction_type: TransactionType::Deposit,
                wallet_address: "ALiCEqZUF4VYuxTu1UQvzDqbpGYYFrxH6kQxWFB8Nqp3".to_string(),
                amount: 25,
            },
        ];

        let result = calculate_wallet_balance("ALiCEqZUF4VYuxTu1UQvzDqbpGYYFrxH6kQxWFB8Nqp3", &transactions);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), 75);
    }

    /// Tests error handling for invalid wallet addresses
    #[test]
    fn test_invalid_wallet_address() {
        let transactions = vec![];
        assert!(matches!(
            calculate_wallet_balance("", &transactions),
            Err(TransactionError::InvalidWalletAddress(_))
        ));
    }

    /// Tests error handling for empty transaction list
    #[test]
    fn test_empty_transaction_list() {
        let transactions = vec![];
        assert!(matches!(
            calculate_wallet_balance("ALiCEqZUF4VYuxTu1UQvzDqbpGYYFrxH6kQxWFB8Nqp3", &transactions),
            Err(TransactionError::NoTransactions(_))
        ));
    }

    /// Tests error handling for zero amount transactions
    #[test]
    fn test_zero_amount() {
        let transactions = vec![Transaction {
            transaction_type: TransactionType::Deposit,
            wallet_address: "ALiCEqZUF4VYuxTu1UQvzDqbpGYYFrxH6kQxWFB8Nqp3".to_string(),
            amount: 0,
        }];
        assert!(matches!(
            calculate_wallet_balance("ALiCEqZUF4VYuxTu1UQvzDqbpGYYFrxH6kQxWFB8Nqp3", &transactions),
            Err(TransactionError::ZeroAmount)
        ));
    }
}
