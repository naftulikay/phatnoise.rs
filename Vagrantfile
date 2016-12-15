# -*- mode: ruby -*-
# vi: set ft=ruby :

# Vagrantfile API/syntax version. Don't touch unless you know what you're doing!
VAGRANTFILE_API_VERSION = "2"

Vagrant.configure(VAGRANTFILE_API_VERSION) do |config|

  # CentOS 7 Development Machine
  config.vm.define "devel", autostart: true, primary: true do |devel|
    devel.vm.box = "bento/centos-7.2"

    # set the hostname
    devel.vm.hostname = "rust-devel"

    # Create a private network, which allows host-only access to the machine using a specific IP.
    devel.vm.network "private_network", type: "dhcp"

    devel.vm.provider "virtualbox" do |vb|
      vb.cpus = 4
      vb.memory = 2048
      vb.linked_clone = true
    end

    devel.vm.provision "ansible_local" do |ansible|
      # define playbook
      ansible.playbook = "vagrant/playbook.yml"
      # allow custom arguments to ansible
      if ENV.has_key?("ANSIBLE_ARGS")
        ansible.raw_arguments = ENV.fetch("ANSIBLE_ARGS").split(/\s+/)
      end
    end
  end
end
