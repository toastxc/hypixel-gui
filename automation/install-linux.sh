# compile
mold --run cargo b -r
# remove old
sudo rm /bin/hypixel-gui
sudo rm /usr/share/applications/hypixel-gui.desktop
sudo rm /usr/share/icons/hypixel-gui.png

sudo cp ./target/release/hypixel-gui /bin/hypixel-gui
sudo cp ./hypixel-gui.png /usr/share/icons/hypixel-gui.png
# install application
echo "[Desktop Entry]
      Name=Bazaar
      Exec=/bin/hypixel-gui
      Icon=/usr/share/icons/hypixel-gui.png
      Type=Application
      Categories=Utility;
" | sudo tee /usr/share/applications/hypixel-gui.desktop > /dev/null

# update desktop
sudo update-desktop-database
