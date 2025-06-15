#[cfg(test)]
mod tests {
    use crate::{
        auth::{login, signup},
        utils::User,
    };

    #[tokio::test]
    async fn signup_test() {
        let email = "something@email.com".to_string();
        let password = "wedFF1234".to_string();

        let sign_result = signup(email.clone(), password.clone()).await;
        assert_ne!(sign_result, None);
        let unwrapped_hashed_user = match sign_result {
            Some(user) => user,
            None => crate::utils::User {
                email: String::new(),
                password: String::new(),
                balance: 1,
            },
        };
        unwrapped_hashed_user
            .new_user()
            .await
            .expect("Error while trying to add new_user");
        assert_eq!(unwrapped_hashed_user.email, email);
        assert_eq!(unwrapped_hashed_user.balance, 0);
    }

    /// Test for Signup function and login function:
    #[tokio::test]
    async fn login_test() {
        let email = "something@text.com".to_string();
        let password = "wedFF1234".to_string();

        let res = login(email, password).await;
        assert_ne!(res, None);
    }

    #[tokio::test]
    async fn delete_user_test() {
        User::delete_user("something@email.com")
            .await
            .expect("Error Deleting user: ");
    }
}
