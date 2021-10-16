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
    cookies {
      name
      value
    }
  }
}
```

- `message` - either returns a default message or echoes back the message value sent.
- `headers` - echoes back all headers sent in the request
- `cookies` - echoes back all cookies sent in the request
