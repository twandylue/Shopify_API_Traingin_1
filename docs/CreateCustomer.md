# Create a Customer

創建帳號(消費者)

## APIs

- [建立帳號(customerCreate)](https://shopify.dev/api/storefront/2022-10/mutations/customerCreate)
  - scope: `unauthenticated_write_customers`

- [取得帳號 Access Token(customerAccessTokenCreate)](https://shopify.dev/api/storefront/2022-10/mutations/customerAccessTokenCreate)
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

- Variables

  ```graphql
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

  ```graphql
  {
    "data": {
      "customer": {
        "id": "Z2lkOi8vc2hvcGlmeS9DdXN0b21lci82NTcyOTg4MjAzMTg0",
        "firstName": "Andy",
        "lastName": "Lu",
        "acceptsMarketing": true,
        "email": "YoUrEmAil@???",
        "phone": "+11111111111",
      }
    }
  }
  ```

### Second step (customerAccessTokenCreate)

- Request(Mutation)

  ```graphql
  mutation customerAccessTokenCreate {
    customerAccessTokenCreate(input: {email: "YoUrEmAil@???", password: "---???***"}) {
      customerAccessToken {
        accessToken
      }
      customerUserErrors {
        message
      }
    }
  }
  ```

- Response

  ```graphql
  {
    "data": {
      "customerAccessTokenCreate": {
        "customerAccessToken": {
          "accessToken": "********************************"
        },
        "customerUserErrors": []
      }
    }
  }
  ```
