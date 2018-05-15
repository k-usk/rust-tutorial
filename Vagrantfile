### should change ###
ipaddress = "192.168.33.11"

Vagrant.configure("2") do |config|
  config.vm.box = "centos/7"
  config.vm.network "private_network", ip: ipaddress
  config.vm.synced_folder ".", "/var/www", id: "user", :nfs => true
  config.vm.box_check_update = false
  
  # config.vm.provider "virtualbox" do |vb|
  #   vb.memory = "1024"
  # end

   config.vm.provision "shell" do |shell|
     shell.path = "settings/init.sh"
   end
end
