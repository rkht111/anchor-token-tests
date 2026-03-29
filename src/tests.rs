#[cfg(test)]
mod tests {
    use super::*;

    // Имитация структуры аккаунта
    struct TokenAccount {
        owner: String,
        balance: u64,
    }

    // Имитация функций (если они не импортированы из основного модуля)
    fn create_token_account(owner: &str) -> TokenAccount {
        TokenAccount {
            owner: owner.to_string(),
            balance: 0,
        }
    }

    fn mint_tokens(account: &mut TokenAccount, amount: u64) {
        account.balance += amount;
    }

    fn transfer_tokens(from: &mut TokenAccount, to: &mut TokenAccount, amount: u64) {
        if from.balance >= amount {
            from.balance -= amount;
            to.balance += amount;
        }
    }

    #[test]
    fn test_create_token_account() {
        let user = "user1";
        let token_account = create_token_account(user);
        assert_eq!(token_account.owner, user);
        assert_eq!(token_account.balance, 0);
    }

    #[test]
    fn test_mint_tokens() {
        let mut account = create_token_account("user1");
        mint_tokens(&mut account, 1000);
        assert_eq!(account.balance, 1000);
    }

    #[test]
    fn test_transfer_tokens() {
        let mut acc1 = create_token_account("user1");
        let mut acc2 = create_token_account("user2");

        mint_tokens(&mut acc1, 1000);
        transfer_tokens(&mut acc1, &mut acc2, 500);

        assert_eq!(acc1.balance, 500);
        assert_eq!(acc2.balance, 500);
    }

    #[test]
    fn test_insufficient_funds_transfer() {
        let mut acc1 = create_token_account("user1");
        let mut acc2 = create_token_account("user2");

        mint_tokens(&mut acc1, 100);
        transfer_tokens(&mut acc1, &mut acc2, 500); // Сумма больше баланса

        // Баланс не должен измениться, если логика запрещает перевод
        assert_eq!(acc1.balance, 100);
        assert_eq!(acc2.balance, 0);
    }
}
