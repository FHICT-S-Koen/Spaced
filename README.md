# Spaced

### Prequisites

- Docker
- Protoc
- Rustc & Cargo
- sqlx-cli
- Node 18 + NPM

### Usage

```sh
npm i
```

```sh
docker compose up -d
```

```sh
npm run tauri dev
```

**Reset DB**

```sh
docker compose down
docker compose up -d
```

```sh
cd tauri
sqlx migrate run
```

```sh
cargo sqlx prepare --workspace -- --all-targets --all-features
```

```sh
psql postgres://admin:password@localhost:5432/spaced
```

```sql
INSERT INTO item (id, x, y, w, h, name, stylesheet, schema, user_id) VALUES (DEFAULT, 5, 5, 50, 50, 'test', NULL, 'test', 1);
```

**Protoc**

https://www.npmjs.com/package/grpc-web

```sh
protoc proto/item.proto --js_out=import_style=esm:generated --grpc-web_out=import_style=esm,mode=grpcwebtext:generated
```

**Build services**

```sh
docker buildx bake
```

_`docker buildx bake` ignores profiles and builds the services anway._

**DNS**

Ensure to add the kafka container id to `/etc/hosts`, to ensure it can be resolved.

_See <https://stackoverflow.com/questions/43103167/failed-to-resolve-kafka9092-name-or-service-not-known-docker-php-rdkafka>_

**Cert**

```sh
openssl req -new -newkey rsa:2048 -days 365 -nodes -x509 -keyout key.pem -out cert.pem
```

```sh
curl --header "Content-Type: application/json" -X POST http://127.0.0.1:8080/register --data '{"email":"test@example.com", "username": "test", "password": "test"}'
```
