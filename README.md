# Shopify_API_Traingin_1

Arch Trainging

## Requirements

### Required APIs

- [建立帳號(customerCreate)](https://shopify.dev/api/storefront/2022-10/mutations/customerCreate)
  - scope: `unauthenticated_write_customers`

- [列出商品清單_1(collections)](https://shopify.dev/api/storefront/2022-10/queries/collections)
  - scope: Product(`unauthenticated_read_product_listings`)

- [列出商品清單_2(products)](https://shopify.dev/api/storefront/2022-04/queries/products)
  - scope: Product(`unauthenticated_read_product_listings`)

- [取得結帳網址_1(checkoutCreate)](https://shopify.dev/api/storefront/2022-10/mutations/checkoutCreate)
  - [document](https://shopify.dev/custom-storefronts/checkout/create#requirements)
  - scope: `unauthenticated_write_checkouts`, `unauthenticated_read_checkouts` and `write_checkouts_payments`

- [取得結帳網址_2(checkoutCustomerAssociateV2)](https://shopify.dev/api/storefront/2022-10/mutations/checkoutCustomerAssociateV2)
  - scope: `unauthenticated_write_checkouts`, `unauthenticated_read_checkouts` and `write_checkouts_payments`

- [加入購物車(cartLinesAdd)](https://shopify.dev/api/storefront/2022-10/mutations/cartLinesAdd)
  - [document](https://shopify.dev/custom-storefronts/cart/manage)
  - scope: `unauthenticated_read_product_listings`, `unauthenticated_read_customers` and `unauthenticated_write_checkouts`

---

- [查詢購物車(before adding lines into cart)](https://shopify.dev/api/storefront/2022-10/queries/cart)
  - scope: `unauthenticated_read_product_listings`, `unauthenticated_read_customers` and `unauthenticated_write_checkouts`

- [建立購物車(before adding lines into cart)](https://shopify.dev/api/storefront/2022-10/mutations/cartCreate)
  - scope: `unauthenticated_read_product_listings`, `unauthenticated_read_customers` and `unauthenticated_write_checkouts`

## References

- [GraphQL Client In Rust](https://github.com/graphql-rust/graphql-client)
- [Http Client In Rust](https://github.com/hyperium/hyper)
