use cart_create::{CartBuyerIdentityInput, CartInput};
use graphql_client::{GraphQLQuery, Response};
use reqwest::{
    header::{self, HeaderMap},
    Client,
};
use std::error::Error;

type URI = String;
type URL = String;
type DateTime = String;

const URI: &str = "https://eat-your-own-dog-food.myshopify.com/api/2022-10/graphql.json";
const URL: &str = "https://eat-your-own-dog-food.myshopify.com/api/2022-10/graphql.json";
// TODO:
const TOKEN: &str = "********************************";

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "src/graphqls/schema.graphql",
    query_path = "src/graphqls/products_query.graphql",
    response_derives = "Debug"
)]
pub struct ProductsQuery;

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "src/graphqls/schema.graphql",
    query_path = "src/graphqls/cart_create.graphql",
    response_derives = "Debug, Clone"
)]
pub struct CartCreate;

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "src/graphqls/schema.graphql",
    query_path = "src/graphqls/create_customer_access_token.graphql",
    response_derives = "Debug, Clone"
)]
pub struct CustomerAccessTokenCreate;

pub struct GraphqlClient {}

impl GraphqlClient {
    pub fn new() -> Self {
        GraphqlClient {}
    }

    pub async fn query_products(
        &self,
        first: i64,
    ) -> Result<Vec<(String, String)>, Box<dyn Error>> {
        let input: products_query::Variables = products_query::Variables { first };
        let request_body = ProductsQuery::build_query(input);
        let mut header = HeaderMap::new();
        header.insert(
            "X-Shopify-Storefront-Access-Token",
            header::HeaderValue::from_str(TOKEN).unwrap(),
        );

        let client = Client::builder().default_headers(header).build()?;

        let res = client.post(URI).json(&request_body).send().await?;
        let response_body: Response<products_query::ResponseData> = res.json().await?;
        let mut result: Vec<(String, String)> = Vec::new();
        match response_body.data {
            Some(r) => {
                for edge in r.products.edges {
                    result.push((edge.node.id, edge.node.title));
                }
            }
            _ => (),
        }

        return Ok(result);

        // return Err("query produects is failed")?;
    }

    async fn add_lines_to_cart(&self) {
        todo!()
    }

    pub async fn create_customer_access_token(
        &self,
        email: String,
        password: String,
    ) -> Result<(String, String), Box<dyn Error>> {
        let input =
            customer_access_token_create::CustomerAccessTokenCreateInput { email, password };
        let request_body =
            CustomerAccessTokenCreate::build_query(customer_access_token_create::Variables {
                input,
            });
        let mut header = HeaderMap::new();
        header.insert(
            "X-Shopify-Storefront-Access-Token",
            header::HeaderValue::from_str(TOKEN).unwrap(),
        );

        let client = Client::builder().default_headers(header).build()?;

        let res = client.post(URL).json(&request_body).send().await?;
        let response_body: Response<customer_access_token_create::ResponseData> =
            res.json().await?;

        match response_body.data {
            Some(r) => {
                let token = r
                    .clone()
                    .customer_access_token_create
                    .unwrap()
                    .customer_access_token
                    .unwrap()
                    .access_token;
                let expires_at = r
                    .customer_access_token_create
                    .unwrap()
                    .customer_access_token
                    .unwrap()
                    .expires_at;

                return Ok((token, expires_at));
            }
            _ => (),
        }

        return Err("create_customer_access_token is failed")?;
    }

    pub async fn create_cart(
        &self,
        customer_access_token: String,
    ) -> Result<(String, String), Box<dyn Error>> {
        // "c53b4857b05fb46fe88c396f4374d77d".to_string()
        let iden = CartBuyerIdentityInput {
            country_code: None,
            customer_access_token: Some(customer_access_token),
            delivery_address_preferences: None,
            email: None,
            phone: None,
        };

        let cart_input = CartInput {
            attributes: None,
            buyer_identity: Some(iden),
            discount_codes: None,
            lines: None,
            note: None,
        };

        let input = cart_create::Variables {
            input: Some(cart_input),
        };

        let request_body = CartCreate::build_query(input);
        let mut header = HeaderMap::new();
        header.insert(
            "X-Shopify-Storefront-Access-Token",
            header::HeaderValue::from_str(TOKEN).unwrap(),
        );

        let client = Client::builder().default_headers(header).build()?;

        let res = client.post(URL).json(&request_body).send().await?;
        let response_body: Response<cart_create::ResponseData> = res.json().await?;

        match response_body.data {
            Some(r) => {
                let id = r.clone().cart_create.unwrap().cart.unwrap().id;
                let url = r.cart_create.unwrap().cart.unwrap().checkout_url;
                // println!("cart id: {:#?}", &id);
                // println!("cart checkout_url: {:#?}", &url);

                return Ok((id, url));
            }
            _ => (),
        }

        return Err("create_cart failed")?;
    }
}
