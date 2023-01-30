# Add to Cart and Get Checkout Url

加入購物車並取得結帳連結

## APIs

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

## Sequence

### First step (cartCreate)

- Request

  ```graphql
  mutation cartCreate {
    cartCreate {
      cart {
        id
        checkoutUrl
      }
      userErrors {
        field
        message
      }
    }
  }
  ```

- Variables

  ```json
  {
    "input": {
      "buyerIdentity": {
        "customerAccessToken": "********************************"
      },
      "lines": [
        {
          "merchandiseId": "gid://shopify/ProductVariant/2844131843",
          "quantity": 1
        }
      ]
    }
  }
  ```

- Response

  ```json
  {
      "data": {
          "cartCreate": {
              "cart": {
                  "id": "gid://shopify/Cart/51248df9eb2305a7394f020563966ec7",
                  "checkoutUrl": "https://eat-your-own-dog-food.myshopify.com/cart/c/51248df9eb2305a7394f020563966ec7"
              },
              "userErrors": []
          }
      }
  }
  ```

### Second step (cartLinesAdd)

Before: [Getting product id](ListAllProducts.md)

- Request

  ```graphql
  mutation cartLinesAdd($cartId: ID!, $lines: [CartLineInput!]!) {
    cartLinesAdd(cartId: $cartId, lines: $lines) {
      cart {
        # Cart fields
        id
        checkoutUrl
        buyerIdentity{
            email
            phone
        }
        lines(first:100) {
            edges{
                node{
                    id
                    quantity
                }
            }
        }
      }
      userErrors {
        field
        message
      }
    }
  }
  ```

- Input Variables

  ```json
  {
    "cartId": "gid://shopify/Cart/fa816cce12a47ca22e99b7ae0ff4f189",
    "lines": {
      "merchandiseId": "gid://shopify/ProductVariant/2844131843",
      "quantity": 1
    }
  }
  ```

- Response

  ```json
  {
      "data": {
          "cartLinesAdd": {
              "cart": {
                  "id": "gid://shopify/Cart/51248df9eb2305a7394f020563966ec7",
                  "checkoutUrl": "https://eat-your-own-dog-food.myshopify.com/cart/c/51248df9eb2305a7394f020563966ec7",
                  "buyerIdentity": {
                      "email": null,
                      "phone": null
                  },
                  "lines": {
                      "edges": [
                          {
                              "node": {
                                  "id": "gid://shopify/CartLine/383ba3b7-01ee-4148-bc4e-a5b923f2e091?cart=51248df9eb2305a7394f020563966ec7",
                                  "quantity": 5
                              }
                          }
                      ]
                  }
              },
              "userErrors": []
          }
      }
  }
  ```

### Third step (cart)

- Request

  ```graphql
  {
      cart(id: "gid://shopify/Cart/51248df9eb2305a7394f020563966ec7" )
      {
          id
          checkoutUrl
          buyerIdentity
          {
              email
              phone
          }
          cost
          {
              totalAmount
              {
                  amount
                  currencyCode
              }
          }
          lines(first:100)
          {
              edges
              {
                  node
                  {
                      id
                      quantity
                      cost
                      {
                          amountPerQuantity
                          {
                              amount
                              currencyCode
                          }
                          subtotalAmount
                          {
                              amount
                              currencyCode
                          }
                          totalAmount
                          {
                              amount
                              currencyCode
                          }
                      }
                  }
              }
          }
      }
  }
  ```

- Response

  ```json
  {
      "data": {
          "cart": {
              "id": "gid://shopify/Cart/51248df9eb2305a7394f020563966ec7",
              "checkoutUrl": "https://eat-your-own-dog-food.myshopify.com/cart/c/51248df9eb2305a7394f020563966ec7",
              "buyerIdentity": {
                  "email": null,
                  "phone": null
              },
              "cost": {
                  "totalAmount": {
                      "amount": "6.0",
                      "currencyCode": "USD"
                  }
              },
              "lines": {
                  "edges": [
                      {
                          "node": {
                              "id": "gid://shopify/CartLine/d3ce92d7-49a4-41c5-8dcc-265ff1353e76?cart=51248df9eb2305a7394f020563966ec7",
                              "quantity": 3,
                              "cost": {
                                  "amountPerQuantity": {
                                      "amount": "2.0",
                                      "currencyCode": "USD"
                                  },
                                  "subtotalAmount": {
                                      "amount": "6.0",
                                      "currencyCode": "USD"
                                  },
                                  "totalAmount": {
                                      "amount": "6.0",
                                      "currencyCode": "USD"
                                  }
                              }
                          }
                      }
                  ]
              }
          }
      }
  }
  ```
