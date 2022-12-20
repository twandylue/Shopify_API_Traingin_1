# Shopify_API_Traingin_1

Arch Trainging

## Requirement

### Required APIs

- [建立帳號(customerCreate)](https://shopify.dev/api/storefront/2022-10/mutations/customerCreate)
  - type: graphql
  - scope: `unauthenticated_write_customers`

- [列出商品清單_1(collections)](https://shopify.dev/api/storefront/2022-10/queries/collections)
  - type: graphql
  - scope: None

- [列出商品清單_2(products)](https://shopify.dev/api/storefront/2022-04/queries/products)
  - type: graphql
  - scope: None

- [取得結帳網址_1(checkoutCreate)](https://shopify.dev/api/storefront/2022-10/mutations/checkoutCreate)
  - type: graphql
  - scope: `unauthenticated_write_checkouts`

- [取得結帳網址_2(checkoutCustomerAssociateV2)](https://shopify.dev/api/storefront/2022-10/mutations/checkoutCustomerAssociateV2)
  - type: graphql
  - scope: `unauthenticated_write_checkouts`

- [加入購物車(cartLinesAdd)](https://shopify.dev/api/storefront/2022-10/mutations/cartLinesAdd)
  - type: graphql
  - scope: None

---

- [查詢購物車(before adding lines into cart)](https://shopify.dev/api/storefront/2022-10/queries/cart)
  - type: graphql
  - scope: None

- [建立購物車(before adding lines into cart)](https://shopify.dev/api/storefront/2022-10/mutations/cartCreate)
  - type: graphql
  - scope: None

## References

- [GraphQL Client In Rust](https://github.com/graphql-rust/graphql-client)
- [Http Client In Rust](https://github.com/hyperium/hyper)
