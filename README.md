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
```
curl -fsSL https://deb.nodesource.com/setup_21.x | sudo -E bash - &&\
sudo apt-get install -y nodejs
```
* build app
```
make clean
make docker_build
```

## Run app
```
make run
make run_wi
```

## Run app inside docker (TODO)
Not ready yet
```
make docker_run
```
