# hlsocial
Simple Social Network as an educational project

## Run locally
* install rust
```
curl --proto '=https' --tlsv1.3 https://sh.rustup.rs -o ./sh.rustup.rs && \
chmod +x ./sh.rustup.rs && \
./sh.rustup.rs -y 
```
* install npm and react
```curl -fsSL https://deb.nodesource.com/setup_21.x | sudo -E bash - &&\
sudo apt-get install -y nodejs
```
* build and run app
```
make clean
make build
make run
```

## Run app inside docker
```
make clean
make docker_build
make docker_run
```
