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
