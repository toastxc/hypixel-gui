mold --run cargo b -r
install -D target/release/hypixel-gui .
flatpak install flathub org.gnome.Sdk//45
flatpak-builder build-dir xyz.toastxc.Bazaar.json  --force-clean
flatpak-builder --user --install --force-clean build-dir xyz.toastxc.Bazaar.json
