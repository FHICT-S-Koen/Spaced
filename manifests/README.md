create cluster & registry

```sh
k3d registry create spaced --port 5001
k3d cluster create spaced --servers 1 --agents 1 --port 9080:80@loadbalancer --registry-use spaced:5001
```

reset cluster & registry

```sh
k3d registry delete spaced
k3d cluster delete spaced
```

install postgres (for debugging)

```sh
helm install debug oci://registry-1.docker.io/bitnamicharts/postgresql \
--set "global.postgresql.auth.secretKeys.userPasswordKey=password" \
--set "global.postgresql.auth.database=spaced" \
--set "global.postgresql.auth.username=admin" \
--set "global.postgresql.service.ports.postgresql=5432" \
--set "postgresql.fullnameOverride=debug-postgresql" \
--set "postgresql.enabled=false"
```

tag services (as debug)

```sh
docker tag spaced/user_service:debug localhost:5001/user-service:debug
docker tag spaced/item_service:debug localhost:5001/item-service:debug
docker tag spaced/web_socket:debug localhost:5001/web-socket:debug
```

push services (as debug)

```sh
docker push localhost:5001/user-service:debug
docker push localhost:5001/item-service:debug
docker push localhost:5001/web-socket:debug
```

deploy services (as debug)

```sh
kubectl apply -f manifests/deployment-user-service.yaml
kubectl apply -f manifests/deployment-item-service.yaml
kubectl apply -f manifests/deployment-web-socket.yaml
```

debug with executor

```sh
kubectl exec -it pod_name -- ls
```

PORT forwarding

```sh
kubectl port-forward [POD_NAME] 3000
```
