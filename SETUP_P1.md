
normal setup then add stuff for encrypted drive.

```md
sudo apt install cryptsetup
```

## Setup new system

only needed once
```bash
sudo cryptsetup luksFormat --type luks2 /dev/md0
sudo cryptsetup luksOpen /dev/md0 md0_open
sudo mkfs.ext4 /dev/mapper/md0_open
sudo mkdir -p /mnt/md0

# create keyfile
sudo dd if=/dev/urandom of=/root/md0.key bs=512 count=1
sudo chmod 0400 /root/md0.key
sudo cryptsetup luksAddKey /dev/md0 /root/md0.key

# setup cryptsetup
sudo blkid /dev/md0
sudo nano /etc/crypttab
# md0_open UUID=48f48a20-ab3f-4741-a867-6838e04ff858 /root/md0.key luks

# setup fstab
sudo blkid /dev/mapper/md0_open
sudo nano /etc/fstab
# UUID=8d861d28-24ee-450c-bad1-2e5556165893 /mnt/md0 ext4 defaults 0 2
sudo update-initramfs -u
```


## Open encrypted drive

```bash
sudo cryptsetup luksOpen /dev/md0 md0_open
sudo mount /dev/mapper/md0_open /mnt/md0
```
