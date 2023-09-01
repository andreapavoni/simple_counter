# Kountr



## Handling models entities

1. Generate migrations
```bash
sea migrate generate -d migration -u $DATABASE_URL create_counters_table     
```

2. Run migrations
```bash
sea migrate up -d migration
```

3. Generate entity
```sh
sea generate entity --database-url $DATABASE_URL -l -o entity/src
```


## Run app locally

```bash
cargo run
```
