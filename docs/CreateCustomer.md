# Create a Customer

創建帳號(消費者)

## APIs

- [建立Customer(customerCreate)](https://shopify.dev/api/storefront/2022-10/mutations/customerCreate)
  - scope: `unauthenticated_write_customers`

- [取得Customer Access Token(customerAccessTokenCreate)](https://shopify.dev/api/storefront/2022-10/mutations/customerAccessTokenCreate)
  - scope: `unauthenticated_write_customers`

## Sequence

### First step (customerCreate)

- Request

  ```graphql
  mutation customerCreate($input: CustomerCreateInput!) {
    customerCreate(input: $input) {
      customer {
        firstName
        lastName
        email
        phone
        acceptsMarketing
      }
      customerUserErrors {
        field
        message
        code
      }
    }
  }
  ```

- Input Variables

  ```json
  {
    "input": {
      "firstName": "Andy",
      "lastName": "Lu",
      "email": "YoUrEmAil@???",
      "phone": "+11111111111",
      "password": "---???***",
      "acceptsMarketing": true
    }
  }
  ```

- Response

  ```json
  {
      "data": {
          "customerCreate": {
              "customer": {
                  "firstName": "Andy",
                  "lastName": "Lu",
                  "email": "test123@91app.gmail.com",
                  "phone": "+15146669998",
                  "acceptsMarketing": true
              },
              "customerUserErrors": []
          }
      }
  }
  ```

### Second step (customerAccessTokenCreate)

- Request

  ```graphql
  mutation customerAccessTokenCreate($input: CustomerAccessTokenCreateInput!) {
    customerAccessTokenCreate(input: $input) {
      customerUserErrors {
        code
        field
        message
      }
      customerAccessToken {
        accessToken
        expiresAt
      }
    }
  }
  ```

- Input Variables

  ```json
  {
    "input": {
      "email": "test123@91app.gmail.com",
      "password": "123454351234"
    }
  }
  ```

- Response

  ```json
  {
      "data": {
          "customerAccessTokenCreate": {
              "customerUserErrors": [],
              "customerAccessToken": {
                  "accessToken": "********************************",
                  "expiresAt": "2023-02-22T09:15:21Z"
              }
          }
      }
  }
  ```
