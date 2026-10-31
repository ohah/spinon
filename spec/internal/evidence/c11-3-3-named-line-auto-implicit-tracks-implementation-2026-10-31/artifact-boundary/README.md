# C11.3.3 fixture Release archive 경계

fixed-only no-area `auto` validation guard를 복원한 최종 소스에서 macOS ARM64 Release FFI archive의 fixture 경계를 확인했다. [최종 검사 기록](../post-validator-guard-20261031/README.md)은 기본 archive의 fixture symbol 0개와 신규 scene payload marker 0개, `c04-runtime-gpu-fixture-test` opt-in archive의 symbol 2개와 scene marker 3개를 대조한다. 검사 뒤 기본 archive를 복원했다.

이 검사는 macOS ARM64 Rust static archive의 feature 경계만 증명한다. Android APK와 iOS IPA 전체에서 fixture adapter 문자열까지 제외되는지는 별도 검증하지 않았다.
