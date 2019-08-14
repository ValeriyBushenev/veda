BUILD_PATH=$PWD

#!/bin/sh
rm *.log
rm ./logs/*.log
rm -r ./logs

if [ ! -f ./ontology/config.ttl ]
then
  cp ./ontology/config.ttl.cfg ./ontology/config.ttl
fi
./tools/update-version-ttl.sh

export CARGO_TARGET_DIR=$PWD/tmp

if [ -z $1 ] || [ $1 == "az" ] || [ $1 == "veda-az" ] ; then

    cd source/authorization
    cargo build --release
    cd $BUILD_PATH
    cp $CARGO_TARGET_DIR/release/libauthorization.so ./source/lib64/libauthorization.so
    cp $CARGO_TARGET_DIR/release/libauthorization.a ./source/lib64/libauthorization.a
    sudo cp $CARGO_TARGET_DIR/release/libauthorization.so $PWD
    sudo cp $CARGO_TARGET_DIR/release/libauthorization.so /usr/local/lib
    sudo ldconfig

fi

if [ -z $1 ] || [ $1 == "exim-master" ] || [ $1 == "veda-exim-master" ] ; then

    echo start make VEDA-EXIM-MASTER
    rm ./veda-veda-exim-master

    cd source/veda-exim-master
    cargo build --release
    cd $BUILD_PATH
    cp $CARGO_TARGET_DIR/release/veda-exim-master $PWD

    echo end make VEDA-EXIM-MASTER
fi

if [ -z $1 ] || [ $1 == "exim-slave" ] || [ $1 == "veda-exim-slave" ] ; then

    echo start make VEDA-EXIM-SLAVE
    rm ./veda-veda-exim-slave

    cd source/veda-exim-slave
    cargo build --release
    cd $BUILD_PATH
    cp $CARGO_TARGET_DIR/release/veda-exim-slave $PWD

    echo end make VEDA-EXIM-SLAVE
fi

if [ -z $1 ] || [ $1 == "extractor" ] || [ $1 == "veda-extractor" ] ; then

    echo start make VEDA-EXTRACTOR
    rm ./veda-extractor

    cd source/veda-extractor
    cargo build --release
    cd $BUILD_PATH
    cp $CARGO_TARGET_DIR/release/veda-extractor $PWD

    echo end make VEDA-EXTRACTOR
fi

if [ -z $1 ] || [ $1 == "ccus" ] || [ $1 == "veda-ccus" ] ; then

    echo start make VEDA-CCUS
    rm ./veda-ccus

    cd source/veda-ccus
    cargo build --release
    cd $BUILD_PATH
    cp $CARGO_TARGET_DIR/release/veda-ccus $PWD

    echo end make VEDA-CCUS
fi

if [ -z $1 ] || [ $1 == "ontologist" ] || [ $1 == "veda-ontologist" ] ; then

    echo start make VEDA-ONTOLOGIST
    rm ./veda-ontologist

    cd source/veda-ontologist
    cargo build --release
    cd $BUILD_PATH
    cp $CARGO_TARGET_DIR/release/veda-ontologist $PWD

    echo end make VEDA-ONTOLOGIST
fi

if [ -z $1 ] || [ $1 == "geo-indexer" ] || [ $1 == "veda-geo-indexer" ] ; then

    echo start make VEDA-GEO-INDEXER
    rm ./veda-geo-indexer

    cd source/veda-geo-indexer
    cargo build --release
    cd $BUILD_PATH
    cp $CARGO_TARGET_DIR/release/veda-geo-indexer $PWD

    echo end make VEDA-GEO-INDEXER
fi

if [ -z $1 ] || [ $1 == "webserver" ] || [ $1 == "veda-webserver" ] ; then

    echo start make VEDA-WEBSERVER
    rm ./veda-webserver

    cd source/veda-webserver
    cargo build --release
    cd $BUILD_PATH
    cp $CARGO_TARGET_DIR/release/veda-webserver $PWD

    echo end make VEDA-WEBSERVER
fi

if [ -z $1 ] || [ $1 == "bootstrap" ] || [ $1 == "veda" ] ; then

    ./tools/build-component.sh veda-bootstrap bootstrap
    rm veda
    rename "s/veda-bootstrap/veda/g" *
fi

if [ -z $1 ] || [ $1 == "mstorage" ] || [ $1 == "veda-mstorage" ] ; then
    ./tools/build-component.sh veda-mstorage mstorage
fi

if [ -z $1 ] || [ $1 == "ro-storage" ] || [ $1 == "veda-ro-storage" ] ; then
    rm ./veda-lmdb-srv
    ./tools/build-component.sh veda-ro-storage ro-storage
fi

if [ -z $1 ] || [ $1 == "authorization" ] || [ $1 == "veda-authorization" ] ; then
    ./tools/build-component.sh veda-authorization authorization
fi

if [ -z $1 ] || [ $1 == "fanout-email" ] || [ $1 == "veda-fanout-email" ] ; then
    ./tools/build-component.sh veda-fanout-email fanout-email
fi

if [ -z $1 ] || [ $1 == "fanout-sql-np" ] || [ $1 == "veda-fanout-sql-lp" ] ; then
    ./tools/build-component.sh veda-fanout-sql-np fanout-sql-np
    ./tools/build-component.sh veda-fanout-sql-lp fanout-sql-lp
fi

if [ -z $1 ] || [ $1 == "scripts" ] || [ $1 == "veda-scripts" ] ; then
    rm ./veda-scripts-main
    rm ./veda-scripts-lp
    rm ./veda-ltr-scripts
    ./tools/build-component.sh veda-scripts scripts
fi

if [ -z $1 ] || [ $1 == "ft-indexer" ] || [ $1 == "veda-ft-indexer" ] ; then
    ./tools/build-component.sh veda-ft-indexer ft-indexer
fi

if [ -z $1 ] || [ $1 == "ttlreader" ] || [ $1 == "veda-ttlreader" ] ; then
    ./tools/build-component.sh veda-ttlreader ttlreader
fi

if [ -z $1 ] || [ $1 == "ft-query" ] || [ $1 == "veda-ft-query" ] ; then
    ./tools/build-component.sh veda-ft-query ft-query
fi

if [ -z $1 ] || [ $1 == "input-queue" ] || [ $1 == "veda-input-queue" ] ; then
    ./tools/build-component.sh veda-input-queue input-queue
fi

if [ -z $1 ] || [ $1 == "gowebserver" ] || [ $1 == "veda-gowebserver" ]; then

    if [ -z $GOROOT ]; then
	export GOROOT=/usr/local/go
    else 
	echo "var GOROOT is set to '$GOROOT'"; 
    fi

    export PATH=$PATH:$GOROOT/bin:$GOPATH/bin
    export GOPATH=$HOME/go

    echo build gowebserver
    cd source/veda-gowebserver
    go build
    cd $BUILD_PATH
    cp source/veda-gowebserver/veda-gowebserver ./veda-gowebserver
fi
