# For GitHub Actions

sudo apt-get install make git-core cmake python2 python3 -y

# export VITASDK=/usr/local/vitasdk
# export PATH=$VITASDK/bin:$PATH

git clone https://github.com/vitasdk/vdpm
cd vdpm
./bootstrap-vitasdk.sh
./install-all.sh
