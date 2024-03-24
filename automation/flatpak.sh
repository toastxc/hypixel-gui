mold --run cargo b -r
install -D target/release/hypixel-gui .
flatpak-builder build-dir xyz.toastxc.Bazaar.yaml  --force-clean
flatpak-builder --user --install --force-clean build-dir xyz.toastxc.Bazaar.yaml
flatpak --user run xyz.toastxc.Bazaaar