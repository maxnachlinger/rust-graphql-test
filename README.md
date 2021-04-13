# rust-graphql-test

> A test GraphQL API using actix-web and async-graphql

## How to run

- Clone this repo
- Run `cargo run`
- The GraphQL Playground is available at: http://localhost:8080/graphql

## Queries

```graphql
{
  test {
    message(message: "hello 😊")
    headers {
      name
      value
    }
  }
}
```

- `headers` - echoes back all headers sent in the request
- `message` - either returns a default message or echoes back the message value sent.
