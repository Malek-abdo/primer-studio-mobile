# Primer Studio Mobile

محرر فيديو موبايل مفتوح المصدر مبني بـ **Rust + Tauri 2**، بواجهة عربية متجاوبة مستوحاة من سير عمل تطبيقات التحرير الاحترافية.

> هذا المشروع مستقل وغير تابع لـ Adobe، ولا ينسخ العلامة التجارية أو الأصول المحمية.

## الحالة الحالية
- واجهة محرر متجاوبة للموبايل والسطح المكتبي.
- مكتبة وسائط تجريبية، معاينة، خط زمني متعدد المسارات.
- أدوات التحرير الأساسية: قص وتقسيم، نص، فلاتر، انتقالات.
- لوحة خصائص للتحويل والعتامة والصوت.
- نقطة بداية Rust/Tauri لإضافة منتقي ملفات Android، معالجة الفيديو FFmpeg، وحفظ المشاريع.

## البناء
```bash
. "$HOME/.cargo/env"
export ANDROID_HOME="$HOME/android-sdk"
export ANDROID_SDK_ROOT="$ANDROID_HOME"
export PATH="$HOME/.cargo/bin:$ANDROID_HOME/cmdline-tools/latest/bin:$ANDROID_HOME/platform-tools:$PATH"
cargo tauri android init
cargo tauri android build --apk
```

الناتج يوجد عادةً داخل `src-tauri/gen/android/app/build/outputs/apk/`.
