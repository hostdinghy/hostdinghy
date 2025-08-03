```bash
rustup target add x86_64-unknown-linux-musl

rsync ../target/release/hostdinghy smadmin@p1.goodserver.ch:/usr/local/bin

cargo build --release --target=x86_64-unknown-linux-musl
rsync ../target/x86_64-unknown-linux-musl/release/hostdinghy smadmin@p1.goodserver.ch:/tmp/hostdinghy
ssh -t smadmin@p1.goodserver.ch 'sudo mv /tmp/hostdinghy /usr/local/bin/hostdinghy'
```

## Deploy webui
```bash
docker build -t registry.p1.goodserver.ch/hostdinghy/studio -f dockerfiles/Dockerfile .
docker push registry.p1.goodserver.ch/hostdinghy/studio
```

## Remove postgresql
```bash
sudo systemctl stop postgresql
sudo apt-get --purge remove postgresql\* postgresql-client\* postgresql-contrib\*
sudo rm -rf /etc/postgresql/
sudo rm -rf /etc/postgresql-common/
sudo rm -rf /var/lib/postgresql/
sudo rm -rf /var/log/postgresql/
sudo rm -rf /var/run/postgresql/
sudo deluser postgres
sudo delgroup postgres
sudo apt-get autoremove
sudo apt-get autoclean
sudo rm -rf /mnt/md0/hostdinghy/postgresql
```
