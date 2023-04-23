APK_PATH=../../src-tauri/gen/android/mobile_base64/app/build/outputs/apk/universal/release/

cd build/
rm app-universal-release-unsigned-aligned.apk
zipalign -v -p 4 $APK_PATH/app-universal-release-unsigned.apk app-universal-release-unsigned-aligned.apk

apksigner sign --ks stormlight.jks --out base64-converter.apk ./app-universal-release-unsigned-aligned.apk
cd ..