use graphql_client::{GraphQLQuery, Response};
use reqwest::{
    header::{self, HeaderMap},
    Client,
};
use std::error::Error;

type URI = String;
type URL = String;

const URI: &str = "https://eat-your-own-dog-food.myshopify.com/api/2022-10/graphql.json";
const URL: &str = "https://eat-your-own-dog-food.myshopify.com/api/2022-10/graphql.json";
const TOKEN: &str = "********************************";

pub struct GraphqlClient {}

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
    query_path = "src/graphqls/cartCreate.graphql",
    response_derives = "Debug"
)]
pub struct CartCreate;

impl GraphqlClient {
    async fn query_products(first: i64) -> Result<(), Box<dyn Error>> {
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
        match response_body.data {
            Some(r) => {
                for i in r.products.edges {
                    println!("{:#?}", i.node.id);
                    println!("---------------");
                }
            }
            _ => (),
        }

        Ok(())
    }

    async fn add_lines_to_cart() {
        todo!()
    }

    async fn create_customer_access_token() {
        todo!()
    }

    async fn create_cart(token: String) -> Result<(), Box<dyn Error>> {
        todo!()
    }
}
