# macOS release FFI archive 경계

`--no-default-features` release archive SHA-256: `d8d3aa400241d67292360ca7499020d78f31994f6779eca423ea6391067d12d9`. 30개 양수·음수 scene ID 검색 결과 0개, adapter와 validator 문자열 검색 결과 0개다.

`c04-runtime-gpu-fixture-test` positive-control archive SHA-256: `b9ce0bf209b94ff821513653ac0a752d47273b5e6318a36b3ce23e999b31e74f`. 30/30 scene ID와 `computed_text_indent`, `validate_computed_text_indent` 두 symbol 문자열이 있다.

해시, release build log, raw match 목록은 이 디렉터리에 있다. 이는 macOS static archive의 feature 경계만 확인하며 Android APK, iOS app 또는 release bundle 전체에서 fixture가 빠진다는 뜻은 아니다.
