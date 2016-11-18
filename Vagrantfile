# -*- mode: ruby -*-
# vi: set ft=ruby :
VAGRANTFILE_API_VERSION = "2"

$script = <<SCRIPT
echo "Installing clang..."
sudo apt-get update && sudo apt-get install clang -y
echo "Installing rust as `id`..."
curl https://sh.rustup.rs -sSf -o ~/rust-init.sh
chmod u+x ~/rust-init.sh
~/rust-init.sh -y
export PATH="$PATH:/home/ubuntu/.cargo/bin/"
rustup install nightly
rustup default nightly
ulimit -c unlimited
export RUST_BACKTRACE=1
export RUST_LOG=error
SCRIPT

Vagrant.configure(VAGRANTFILE_API_VERSION) do |config|
  config.vm.provision "shell", inline: $script, privileged: false
  config.vm.box = "ubuntu/xenial64"
  config.vm.synced_folder ".", "/data"
  config.vm.provider "virtualbox" do |v|
      v.memory = 512
  end
end
