#[cfg(test)]
mod tests {
    use crate::{auth::{login, signup}};

    /// Test for Signup function and login function:
    #[tokio::test]
    async fn logup() {
        let email = "something@text.com".to_string();
        let password = "wedFF1234".to_string();

        let res = login(email, password).await;
        assert_ne!(res, None);
    }

    #[tokio::test]
    async fn signup_test() {
        let email = "something@text.com".to_string();
        let password = "wedFF1234".to_string();

        let sign_result = signup(email.clone(), password.clone()).await;
        assert_ne!(sign_result, None);
        let unwrapped_hashed_user = match sign_result{
            Some(user) => user,
            None => crate::utils::User { email: String::new(), password: String::new(), balance: 1 }
        };
        assert_eq!(unwrapped_hashed_user.email, email);
        assert_eq!(unwrapped_hashed_user.balance, 0);

    }
}
