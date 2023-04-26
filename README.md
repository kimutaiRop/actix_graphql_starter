# actix graphql starter

## add the following to .env

```env
POSTGRES_USER=postgres
POSTGRES_PASSWORD=postgres
POSTGRES_DB=phamorder

DATABASE_URL=postgres://postgres:postgres@localhost:5432/phamorder
SECRET_KEY=somesecrertkey
ELASTIC_API_KEY=ELASTIC_MAIL_API_KEY
```

## start docker database with

```bash
    docker-compose up
```

## run migrations with

the starter app contains a user model with a migration

```bash
diesel setup

diesel migration run
```

## start server with

```bash
cargo run
```
