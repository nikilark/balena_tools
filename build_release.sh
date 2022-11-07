version=$(cargo pkgid | grep -o '#.*' | cut -c2-10)
for arch in "x86_64-pc-windows-gnu" "x86_64-unknown-linux-gnu"
do
cargo build --release --target $arch
mkdir -p ./releases/$version/$arch
cp ./target/$arch/release/balena_tools* ./releases/$version/$arch
rm ./releases/$version/$arch/balena_tools.d
done