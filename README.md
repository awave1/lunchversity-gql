# lunchversity-gql

> Lunchversity is a stamp management platform that incentivizes students to use reusable containers at the university restaurants

`lunchversity-gql` is a GraphQL API server for Lunchversity app

## Prerequisites

Ensure you have `cargo` and `rust` installed. After that, go through a process to setup the [`diesel_cli`](https://diesel.rs/). The project also uses `docker` to host the database.

## Development

```bash
# Clone the repo
git clone git@github.com:awave1/lunchversity-gql.git

# open the directory
cd lunchversity-gql

# Run the server
cargo run
```

Once the server is up and running, you can access the data at [localhost:8080/graphiql](localhost:8080/graphiql).


## Contributing

Pull requests and any issues are welcome. For any major changes, please open an issue first to discuss what you would like to change.
