# List All Products

列出所有商品

## APIs

- [列出商品清單_1(collections)](https://shopify.dev/api/storefront/2022-10/queries/collections)
  - scope: Product(`unauthenticated_read_product_listings`)

- [列出商品清單_2(products)](https://shopify.dev/api/storefront/2022-04/queries/products)
  - scope: Product(`unauthenticated_read_product_listings`)

## Sequence

### First step (products)

- Request

  ```graphql
  {
      products(first: 3) {
          edges {
              node {
                  id
                  title
                  totalInventory
                  variants(first: 3) {
                      edges {
                          node {
                              id
                              title
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
          "products": {
              "edges": [
                  {
                      "node": {
                          "id": "gid://shopify/Product/1001131651",
                          "title": "Dog food 1",
                          "totalInventory": -3,
                          "variants": {
                              "edges": [
                                  {
                                      "node": {
                                          "id": "gid://shopify/ProductVariant/2844131843",
                                          "title": "Default Title"
                                      }
                                  }
                              ]
                          }
                      }
                  },
                  {
                      "node": {
                          "id": "gid://shopify/Product/34191638552",
                          "title": "亞朵洗護系列一2",
                          "totalInventory": -6,
                          "variants": {
                              "edges": [
                                  {
                                      "node": {
                                          "id": "gid://shopify/ProductVariant/160399327256",
                                          "title": "Default Title"
                                      }
                                  }
                              ]
                          }
                      }
                  },
                  {
                      "node": {
                          "id": "gid://shopify/Product/36947558424",
                          "title": "朵朵洗護系列二",
                          "totalInventory": -1,
                          "variants": {
                              "edges": [
                                  {
                                      "node": {
                                          "id": "gid://shopify/ProductVariant/170883874840",
                                          "title": "Default Title"
                                      }
                                  }
                              ]
                          }
                      }
                  }
              ]
          }
      }
  }
  ```
