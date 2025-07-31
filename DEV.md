```bash
rustup target add x86_64-unknown-linux-musl

rsync ../target/release/huus smadmin@p1.goodserver.ch:/usr/local/bin

cargo build --release --target=x86_64-unknown-linux-musl
rsync ../target/x86_64-unknown-linux-musl/release/huus smadmin@192.168.1.11:/tmp/huus
ssh -t smadmin@192.168.1.11 'sudo mv /tmp/huus /usr/local/bin/huus'
```

# Remove postgresql
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
sudo rm -rf /mnt/md0/huus/postgresql
```
