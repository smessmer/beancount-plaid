use anyhow::Result;
use plaid::{
    model::LinkTokenCreateRequestUser, request::LinkTokenCreateRequired, PlaidAuth, PlaidClient,
};

const CLIENT_NAME: &str = "beancount-plaid";
const COUNTRY_CODES: &[&str] = &["US"];
const LANGUAGE: &str = "en";
const USER_ID: &str = "user-id";
const PRODUCTS: &[&str] = &["transactions"];

pub struct LinkToken {
    pub link_token: String,
}

pub struct PublicToken {
    pub public_token: String,
}

pub struct AccessToken {
    pub access_token: String,
}

pub struct Plaid {
    client: PlaidClient,
}

impl Plaid {
    pub fn new() -> Plaid {
        Plaid {
            client: PlaidClient::with_auth(PlaidAuth::from_env()),
        }
    }

    pub async fn link_token_create(&self) -> Result<LinkToken> {
        let response = self
            .client
            .link_token_create(LinkTokenCreateRequired {
                client_name: CLIENT_NAME,
                country_codes: COUNTRY_CODES,
                language: LANGUAGE,
                user: LinkTokenCreateRequestUser {
                    client_user_id: USER_ID.to_string(),
                    ..Default::default()
                },
            })
            .products(PRODUCTS)
            .await?;
        Ok(LinkToken {
            link_token: response.link_token,
        })
    }

    pub async fn exchange_public_token(&self, public_token: PublicToken) -> Result<AccessToken> {
        let response = self
            .client
            .item_public_token_exchange(&public_token.public_token)
            .await?;
        Ok(AccessToken {
            access_token: response.access_token,
        })
    }
}
