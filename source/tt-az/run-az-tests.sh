#cargo build
#cp target/debug/libtarantool_authorization.so tt/
cargo build --release
cp target/release/libtarantool_authorization.so tt/db/tarantool/
cd tt
#./clean-db.sh
tarantool init-az.lua