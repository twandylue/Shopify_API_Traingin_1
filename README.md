# Shopify_API_Trainging_1

<!-- [![Github actions Status](https://github.com/graphql-rust/graphql-client/workflows/CI/badge.svg?branch=main&event=push)](https://github.com/graphql-rust/graphql-client/actions) -->

Arch Team Trainging

## Requirements

<!-- TODO: -->

## Finite State Machine

<!-- TODO: -->

### Account

<!-- TODO: -->

### Customer

<!-- TODO: -->

### Cart

<!-- TODO: -->

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

## TODOs

- [ ] Configable
- [ ] Unit tests (with mocking)
- [ ] Documents
- [ ] Panic handler and messages

## References

- [GraphQL Client In Rust](https://github.com/graphql-rust/graphql-client)
- [Http Client In Rust](https://github.com/seanmonstar/reqwest)
- [Cli In Rust](https://rust-cli.github.io/book/index.html)
