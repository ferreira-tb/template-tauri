```bash
cargo install manatsu tauri-cli
manatsu create

cd PROJECT_NAME
npm run dev
```

Just remember to change `PROJECT_NAME` to the name you chose.

## Database

### Migration

```sh
# Generate a new migration file.
sea-orm-cli migrate generate MIGRATION_NAME

# Apply all pending migrations.
sea-orm-cli migrate up

# Rollback last applied migration
sea-orm-cli migrate down

# Drop all tables from the database, then reapply all migrations.
sea-orm-cli migrate fresh

# Rollback all applied migrations, then reapply all migrations.
sea-orm-cli migrate refresh

# Rollback all applied migrations.
sea-orm-cli migrate reset
```

### Generating Entities

[`DATABASE_URL`](https://www.sea-ql.org/SeaORM/docs/generate-entity/sea-orm-cli/#configure-environment) must be set as an environment variable.

```sh
sea-orm-cli generate entity -o src-tauri/src/entities --with-serde both
```

## License

[MIT](https://raw.githubusercontent.com/tsukilabs/manatsu/main/LICENSE)

Copyright (c) 2024 Andrew Ferreira
