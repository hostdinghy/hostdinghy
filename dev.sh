cargo build --package server --release --target=x86_64-unknown-linux-musl
rsync target/x86_64-unknown-linux-musl/release/hostdinghy smadmin@p1.goodserver.ch:/tmp/hostdinghy

docker build -t registry.p1.goodserver.ch/hostdinghy/studio -f dockerfiles/Dockerfile .
docker push registry.p1.goodserver.ch/hostdinghy/studio

ssh -t smadmin@p1.goodserver.ch 'sudo mv /tmp/hostdinghy /usr/local/bin/hostdinghy && sudo systemctl restart hostdinghy && cd /mnt/md0/huus/hostdinghy && sudo docker compose up -d --pull always'
