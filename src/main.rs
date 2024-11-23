#[tokio::main]
async fn main() {
    let client = beancount_plaid::plaid_api::Plaid::new();
    let access_token = beancount_plaid::plaid_api::link_new_account(&client)
        .await
        .unwrap();
    println!("Access token: {:?}", access_token);
    let accounts = beancount_plaid::plaid_api::get_accounts(&client, &access_token)
        .await
        .unwrap();
    println!("Accounts: {:?}", accounts);
}
