# Rust GraphQL Microservice

## Main Dependencies
- Actix
- Juniper
- MySQL

> See others in `Cargo.toml`

## Prerequisites

- Docker and Docker-Compose or MySQL

## Setup

```sh
# Copy env file
cp .env.example .env     # (only needed the first time)

# Set up Database
chmod +x mysql/start.sh  # give execution rights to db start script (only needed the first time)
mysql/start.sh           # start MySQL database (runs seed script on the first time)
```

## Run

```sh
cargo run
```

## Test

Open `http://localhost:8080/graphiql`. GraphQL provides its own documentation. Click the "docs" link in the top right of the GraphiQL UI to see what types of queries and mutations are possible.

```
# Copy-paste this and press ctrl-enter to run this query
{
  user(id: "2") {
    id
    name
    email
    products {
      id
      name
      price
    }
  }
}
```
