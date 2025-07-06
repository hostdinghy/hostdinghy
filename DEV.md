```bash
rustup target add x86_64-unknown-linux-musl

rsync ../target/release/huus smadmin@p1.goodserver.ch:/usr/local/bin

cargo build --release --target=x86_64-unknown-linux-musl
rsync ../target/x86_64-unknown-linux-musl/release/huus smadmin@p1.goodserver.ch:/tmp/huus
ssh -t smadmin@p1.goodserver.ch 'sudo mv /tmp/huus /usr/local/bin/huus && sudo chown root:root /usr/local/bin/huus && sudo chmod 4755 /usr/local/bin/huus'
``
