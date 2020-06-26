#!/bin/bash
# скрипт устанавливает среду для последующей компиляции, берет исходники зависимостей из github, но не собирает

./tools/install-repo-libs.sh

GO_VER=go1.12.1
MSGPUCK_VER=2.0
NANOMSG_VER=1.1.5
LMDB_VER=0.9.22
XAPIAND_VER=1.0.0

INSTALL_PATH=$PWD

# Get other dependencies
LIB_NAME[6]="cmake"
LIB_NAME[7]="libtool-bin"
LIB_NAME[8]="pkg-config"
LIB_NAME[9]="build-essential"
LIB_NAME[10]="autoconf"
LIB_NAME[11]="automake"
LIB_NAME[12]="curl"
LIB_NAME[13]="python"

LIB_OK="Status: install ok installed"
F_UL=0

### LIBS FROM APT ###

for i in "${LIB_NAME[@]}"; do

    L1=`dpkg -s $i | grep 'install ok'`

    echo CHECK $i .... $L1

    if  [ "$L1" != "$LIB_OK" ]; then

      if [ $F_UL == 0 ]; then
          sudo apt-get update
          F_UL=1
      fi

    echo INSTALL $i
        sudo apt-get install -y $i
    fi

done

sudo apt-get install build-essential

### RUST LANG ###

if [ "$1" = force ] || ! rustc -V ; then
    echo "--- INSTALL RUST ---"
    curl https://sh.rustup.rs -sSf | sh -s -- -y
    source $HOME/.cargo/env
else
    echo "--- UPDATE RUST ---"
    rustup update stable
fi

whereis rustc
rustc -V
cargo -V

### GO LANG ###
if [ "$1" = force ] || ! go version | grep $GO_VER ; then
    echo "--- INSTALL GOLANG ---"
    mkdir tmp
    cd tmp
    wget https://storage.googleapis.com/golang/$GO_VER.linux-amd64.tar.gz
    tar -xf $GO_VER.linux-amd64.tar.gz

    if env | grep -q ^GOROOT=
    then
        sudo rm -rf $GOROOT
    else
        export GOROOT=/usr/local/go
        export PATH="$PATH:$GOROOT/bin:$GOPATH/bin"
        echo 'export GOROOT=/usr/local/go'  >> $HOME/.profile
        echo 'export PATH=$PATH:$GOROOT/bin:$GOPATH/bin'  >> $HOME/.profile
    fi

    export GOPATH=$HOME/go
    echo 'export GOPATH=$HOME/go'  >> $HOME/.bashrc
    source ~/.bashrc

    sudo rm -rf /usr/local/go
    sudo rm -rf /usr/bin/go
    sudo rm -rf /usr/bin/gofmt
    sudo mv go $GOROOT

    go version
    cd ..
else
    echo "--- GOLANG INSTALLED ---"
fi

#lmdb-go
#go get -v github.com/muller95/lmdb-go/lmdb
go get github.com/itiu/lmdb-go/lmdb

#fasthttp
go get -v github.com/itiu/fasthttp

#go-nanomsg
go get -v github.com/op/go-nanomsg

go get github.com/tarantool/go-tarantool
go get github.com/gorilla/websocket
go get github.com/divan/expvarmon
go get -v gopkg.in/vmihailenco/msgpack.v2
cp -a ./source/golang-third-party/cbor $GOPATH/src
ls $HOME/go/src

if [ "$1" = force ] || [ "$1" = force-tarantool ] || ! ldconfig -p | grep libtarantool ; then
    echo "--- INSTALL LIBTARANTOOL ---"
    TTC=d93096a9d39e36c456af82e5e53c6ca4f4be608f

    mkdir tmp
    cd tmp

    wget https://github.com/tarantool/tarantool-c/archive/$TTC.tar.gz -P .
    tar -xvzf $TTC.tar.gz

    wget https://github.com/tarantool/msgpuck/archive/$MSGPUCK_VER.tar.gz -P third_party/msgpuck -P .
    tar -xvzf $MSGPUCK_VER.tar.gz

    cp msgpuck-$MSGPUCK_VER/* tarantool-c-$TTC/third_party/msgpuck
    cd tarantool-c-$TTC

    mkdir build
    cd build
    cmake ..
    make
    sudo make install
    sudo ldconfig

    cd ..
    cd ..

else
    echo "--- LIBTARANTOOL INSTALLED ---"
fi

    echo "--- MAKE LIBAUTHORIZATION ---"

    cd $INSTALL_PATH
    cd source/libauthorization
    cargo build --release
    cd ..
    cd ..
    sudo cp ./source/lib64/libauthorization.so /usr/local/lib
    sudo ldconfig


sudo libtool --mode=install install -c $INSTALL_PATH/source/lib64/libxapianm/libxapianm.la /usr/local/lib/libxapianm.la


sudo apt-get install -y libglib2.0-dev
ldd --version
