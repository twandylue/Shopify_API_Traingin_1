# Shopify_API_Trainging_1

<!-- [![Github actions Status](https://github.com/graphql-rust/graphql-client/workflows/CI/badge.svg?branch=main&event=push)](https://github.com/graphql-rust/graphql-client/actions) -->

Arch Team Trainging

## Requirements

### Required APIs

- [建立帳號(customerCreate)](https://shopify.dev/api/storefront/2022-10/mutations/customerCreate)
  - scope: `unauthenticated_write_customers`

- [取得帳號 Access Token(customerAccessTokenCreate)](https://shopify.dev/api/storefront/2022-10/mutations/customerAccessTokenCreate)
  - scope: `unauthenticated_write_customers`

- [列出商品清單_1(collections)](https://shopify.dev/api/storefront/2022-10/queries/collections)
  - scope: Product(`unauthenticated_read_product_listings`)

- [列出商品清單_2(products)](https://shopify.dev/api/storefront/2022-04/queries/products)
  - scope: Product(`unauthenticated_read_product_listings`)

- [建立購物車(cartCreate)](https://shopify.dev/api/storefront/2022-10/mutations/cartCreate)
  - scope: `unauthenticated_read_product_listings`, `unauthenticated_read_customers` and `unauthenticated_write_checkouts`

- [查詢購物車(cart)](https://shopify.dev/api/storefront/2022-10/queries/cart)
  - scope: `unauthenticated_read_product_listings`, `unauthenticated_read_customers` and `unauthenticated_write_checkouts`

- [加入購物車(cartLinesAdd)](https://shopify.dev/api/storefront/2022-10/mutations/cartLinesAdd)
  - [document](https://shopify.dev/custom-storefronts/cart/manage)
  - scope: `unauthenticated_read_product_listings`, `unauthenticated_read_customers` and `unauthenticated_write_checkouts`

- [取得結帳網址_1(checkoutCreate)](https://shopify.dev/api/storefront/2022-10/mutations/checkoutCreate)
  - [document](https://shopify.dev/custom-storefronts/checkout/create#requirements)
  - scope: `unauthenticated_write_checkouts`, `unauthenticated_read_checkouts` and `write_checkouts_payments`

- [取得結帳網址_2(checkoutCustomerAssociateV2)](https://shopify.dev/api/storefront/2022-10/mutations/checkoutCustomerAssociateV2)
  - scope: `unauthenticated_write_checkouts`, `unauthenticated_read_checkouts` and `write_checkouts_payments`

#### Access Scopes

1. unauthenticated_write_customers
2. unauthenticated_read_customers
3. unauthenticated_read_product_listings
4. unauthenticated_write_checkouts
5. unauthenticated_read_checkouts
6. write_checkouts_payments

[Document](https://shopify.dev/api/usage/access-scopes)

## Sequence Diagram

### Sign Up

``` sequence-diagrams
User->ChatBot: Sign Up(providing basic info)
ChatBot->Shopify: (API) CreateCustomer
Shopify->ChatBot: Response
Note left of User: Change State to "signin"
ChatBot->User: Sign Up result
```

### Sign In

``` sequence-diagrams
User->ChatBot: Sign In(inputing basic info)
ChatBot->Shopify: (API) GetCustomer
Shopify->ChatBot: Response
Note left of User: Change State to "signin"
ChatBot->User: Sign In result
```

### Buy a stuff

TODO:

``` sequence-diagrams
Note left of User: State: "signin"
User->ChatBot: List all Products
ChatBot->Shopify: (API) ListAllProducts
Shopify->ChatBot: Response(Products)
ChatBot->User: Product List
User->ChatBot: Choose products
```

## State Machine Diagram

TODO:

## References

- [GraphQL Client In Rust](https://github.com/graphql-rust/graphql-client)
- [Http Client In Rust](https://github.com/seanmonstar/reqwest)
- [Cli In Rust](https://rust-cli.github.io/book/index.html)
