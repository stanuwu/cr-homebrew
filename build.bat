@echo off
rm out/*
pushd swt
xargo clean
xargo check --release --target aarch64-none-elf
xargo build --release --target aarch64-none-elf
cp target\aarch64-none-elf\release\libswt.a lib
popd
make