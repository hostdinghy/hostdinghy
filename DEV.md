```bash
rsync ../target/release/huus smadmin@p1.goodserver.ch:/usr/local/bin

cargo b --release
rsync ../target/release/huus smadmin@p1.goodserver.ch:/tmp/huus
ssh -t smadmin@p1.goodserver.ch 'sudo mv /tmp/huus /usr/local/bin/huus && sudo chown root:root /usr/local/bin/huus && sudo chmod 4755 /usr/local/bin/huus'
``
