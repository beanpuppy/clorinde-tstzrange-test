Ran with:

```bash
clorinde schema schema.sql

docker run  \
  --name postgres-test \
  -p 5435:5432 \
  -e POSTGRES_USER=postgres \
  -e POSTGRES_PASSWORD=postgres \
  -e POSTGRES_DB=postgres \
  -v $(pwd)/schema.sql:/docker-entrypoint-initdb.d/schema.sql \
  postgres:17

cargo run
```
