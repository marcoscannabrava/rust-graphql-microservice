# Rust GraphQL Microservice

## Main Dependencies
- Actix
- Juniper
- MySQL

> See others in `Cargo.toml`

## Prerequisites

- Docker+Docker-Compose or MySQL

## Setup and Run

```sh
cp .env.example .env     # Copy env file (only needed the first time)
```

**Option 1: Generic**

```sh
# Set up Database (if using Docker)
chmod +x mysql/start.sh  # give execution rights to db start script (only needed the first time)
mysql/start              # start MySQL database (runs seed script on the first time)

# Compile and Start Server
cargo run
```

**Option 2: VSCode Dev Environment**

This repo includes `.vscode` settings to bootstrap the dev environment with the dockerized database when starting the debugger and requires the `rust-analyzer` extension.

Shortcut: `Ctrl+Shift+D`, `Enter`.


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

# Resources

- [Actix Documentation](https://actix.rs/docs/)
- [Juniper Book (GraphQL server for Rust)](https://graphql-rust.github.io/juniper/master/index.html)

___

# TODO

- [ ] filter by any column
- [ ] automatically generate resolvers from struct items
- [ ] schema/resolver boilerplate generator