```sh
k3d cluster create spaced --servers 1 --agents 1 --port 9080:80@loadbalancer --registry-use spaced:5001
k3d registry create spaced --port 5001
docker build -t localhost:5001/api:v0.1 .
docker push localhost:5001/api:v0.1
kubectl apply -f charts/deployment.yaml
kubectl apply -f charts/service.yaml
kubectl apply -f charts/ingress.yaml
```
