# Prerequis
```sh
# Running on CPU
# On Linux
wget https://download.pytorch.org/libtorch/cpu/libtorch-shared-with-deps-2.8.0%2Bcpu.zip
unzip libtorch-shared-with-deps-2.8.0+cpu.zip
# On MacOS
wget https://download.pytorch.org/libtorch/cpu/libtorch-macos-arm64-2.8.0.zip
unzip libtorch-macos-arm64-2.8.0.zip

export LIBTORCH=/home/huo/Downloads/libtorch/
export LIBTORCH_INCLUDE="$LIBTORCH"
export LIBTORCH_LIB="$LIBTORCH"
export LIBTORCH_BYPASS_VERSION_CHECK=true
# On Linux
export LD_LIBRARY_PATH="$LIBTORCH/lib"
# On MacOS
#sudo xattr -r -d com.apple.quarantine $LIBTORCH/lib/*.dylib
export DYLD_FALLBACK_LIBRARY_PATH="$LIBTORCH/lib"
```
