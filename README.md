# actix graphql starter

## add the following to .env

```env
POSTGRES_USER=postgres
POSTGRES_PASSWORD=postgres
POSTGRES_DB=phamorder

DATABASE_URL=postgres://postgres:postgres@localhost:5432/phamorder
```

## start docker database with

```bash
    docker-compose up -d
```

## run migrations with

```bash
diesel migration run
```

## start server with

the starter app contains a user model

```bash
cargo run
```
