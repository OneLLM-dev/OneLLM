#[cfg(test)]
mod tests {
    use crate::{
        auth::basicauth::{login, signup},
        database,
        utils::User,
    };

    #[tokio::test]
    async fn user_auth() {
        database::init_db()
            .await
            .expect("error initialising database");

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

        println!("User: {:#?}", unwrapped_hashed_user);

        unwrapped_hashed_user
            .new_user()
            .await
            .expect("Error while trying to add new_user to database");
        assert_eq!(unwrapped_hashed_user.email, email);
        assert_eq!(unwrapped_hashed_user.balance, 0);

        let res = login(email.clone(), password).await;
        println!("Res:\n{:#?}\n", res);
        assert_ne!(res, None);

        User::delete_user(&email)
            .await
            .expect("Error Deleting user: ");
    }
}
