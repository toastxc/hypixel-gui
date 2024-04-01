mold --run cargo b -r
# install for user
sudo flatpak-builder  --user build-dir xyz.toastxc.Bazaar.yaml  --force-clean
sudo flatpak-builder --install --force-clean build-dir xyz.toastxc.Bazaar.yaml