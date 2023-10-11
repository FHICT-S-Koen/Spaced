# Spaced

### Prequisites

- Docker
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
