# Prerequis
```sh
# Running on CPU
wget https://download.pytorch.org/libtorch/cpu/libtorch-shared-with-deps-2.8.0%2Bcpu.zip
unzip libtorch-shared-with-deps-2.8.0+cpu.zip

export LIBTORCH=/home/huo/Downloads/libtorch/
export LIBTORCH_INCLUDE="$LIBTORCH"
export LIBTORCH_LIB="$LIBTORCH"
export LIBTORCH_BYPASS_VERSION_CHECK=true
export LD_LIBRARY_PATH="$LIBTORCH/lib"
```
