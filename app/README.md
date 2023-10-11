# Spaced frontend

**Generate gRPC boilerplate**

```sh
protoc -I=proto item.proto --js_out=import_style=typescript:app/grpc --grpc-web_out=import_style=typescript,mode=grpcwebtext:app/grpc
```
