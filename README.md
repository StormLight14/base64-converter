## .bashrc
```bash
export JAVA_HOME="/usr/lib/jvm/java-11-openjdk" # if you are using Android Studio, the location is different, see the section above about JDK
export ANDROID_HOME="$HOME/.android" # if you are using Android Studio, the sdk location will be at `~/Android/Sdk`
export NDK_HOME="$ANDROID_HOME/ndk/25.0.8775105"
export ANDROID_SDK_ROOT=$ANDROID_HOME
export PATH=$PATH:$ANDROID_HOME/platform-tools:$ANDROID_HOME/tools
export PATH="$PATH:~/.android/emulator"
GTK_MODULES=canberra-gtk-module
PK_GTK_MODULE=pk-gtk-module
export PKG_CONFIG_PATH=/usr/share/pkgconfig/:/usr/lib64/pkgconfig/:/var/lib/flatpak/runtime/org.freedesktop.Sdk/x86_64/22.08/2652112cf6f02f590602d3379a36a0e59d801a120f8ba4bb43170e75e3ab73da/files/lib/x86_64-linux-gnu/pkgconfig/:$PKG_CONFIG_PATH
export PATH=$PATH:/home/storm/Android/Sdk/ndk-bundle/toolchains/llvm/prebuilt/linux-x86_64/bin/
export LD_LIBRARY_PATH=/usr/lib64/:/var/lib/flatpak/runtime/org.freedesktop.Sdk/x86_64/22.08/2652112cf6f02f590602d3379a36a0e59d801a120f8ba4bb43170e75e3ab73da/files/lib/x86_64-linux-gnu/:$LD_LIBRARY_PATH
```

## dnf (fedora 38)
```
sudo dnf install webkit2gtk4.1-devel \
    openssl-devel \
    curl \
    wget \
    libappindicator-gtk3 \
    librsvg2-devel \
    javascriptcoregtk4.1-devel-2.40.0-2.fc38 \
    libgcc-devel
sudo dnf install @development-tools
```