# C15.3 WGPU glyph paint 앱 route 계획 실패 관점 20개

## 범위와 판정

이 검토는 고정 글꼴 glyph mask를 기존 runtime GPU demo host에 연결하고 Android·iOS Simulator에서 실행하는 **아직 구현하지 않은 앱 route 연결**만 대상으로 한다. 이미 구현된 Rust WGPU renderer와 headless readback은 검토 대상의 배경이며, 이 계획 검토를 그 구현의 사후 승인으로 해석하지 않는다.

입력은 opt-in `c15-3-wgpu-glyph-paint-fixture-test`, C19.2 고정 runtime scene, 기존 C15.3 고정 Noto Sans KR fixture, 고정 text/origin/color/clip 값이다. 예상 결과는 Android API 37 ARM64 AVD와 iOS Simulator의 실제 앱 WGPU surface에 같은 shaped glyph mask가 보이고, eval report에 `c15_3_text=attached`와 고정 glyph 수가 나타나며, 저장한 simulator screenshot에서 표시와 clip 경계가 확인되는 것이다. Chrome screenshot antialias 전체 일치는 기준으로 삼지 않는다. Android screenshot과 iOS screenshot은 각 시뮬레이터 화면의 시각 자료이며 각각 PixelCopy/Metal drawable readback과 동등하다고 확대 해석하지 않는다.

## 독립 실패 관점

1. Cargo feature가 기본 feature에 합쳐져 Noto font payload와 C15 FFI symbol이 제품 기본 artifact에 들어간다.
2. Android build flag 검증은 0/1만 확인하고 C04.10 GPU host가 꺼진 조합을 거부하지 않는다.
3. iOS simulator script는 C15.3 feature를 Rust에 전달하지만 Objective-C++ define에는 전달하지 않는다.
4. Xcode Swift compilation condition과 C preprocessor define이 서로 다른 flag를 읽는다.
5. C header 선언 guard와 Rust의 `cfg(feature)` 이름이 대응하지 않아 기본 빌드가 우연히 링크된다.
6. Android Gradle source set이 opt-in일 때도 추가되어 adapter JNI 심볼만 빠진 상태로 런타임 reflection이 실패한다.
7. iOS가 `--spinon-c153-glyph-paint` 인자를 runtime GPU demo 선택 조건에 포함하지 않아 기존 화면으로 빠진다.
8. Android와 iOS에서 서로 다른 argument spelling을 사용해 한 플랫폼만 fixture를 실행한다.
9. route 충돌 검증이 C15.3과 C19.2 또는 C18 image fixture의 동시 선택을 허용한다.
10. feature-disabled 실행이 조용히 일반 scene으로 돌아가 성공처럼 보인다.
11. host 생성은 C15.3 전용 FFI 함수가 아니라 C19.2 host를 사용해 glyph paint가 추가되지 않는다.
12. eval 경로가 C19.2 일반 eval을 불러 `DrawText` command가 snapshot에 제출되지 않는다.
13. report 문자열이 잘려 `c15_3_text=attached`가 없는데도 플랫폼 adapter가 성공을 표시한다.
14. viewport가 C19.2 source의 고정 360×240과 달라 owner frame 또는 clip 계산이 바뀐다.
15. display scale을 DPR 1로 고정하지 않아 AVD/iOS의 물리 scale이 mask 위치를 바꾼다.
16. Android SurfaceView 크기/상태바 inset을 CSS viewport로 오인해 glyph가 화면 밖으로 잘린다.
17. iOS canvas drawable이 준비되기 전에 screenshot을 찍어 검은 surface를 결과로 남긴다.
18. 앱 screenshot만 보고 shader mask가 실제 WGPU draw에 포함되었다고 단정하고, renderer report나 focused readback을 누락한다.
19. clip 바깥에 남은 glyph pixel을 검사하지 않아 경계 누출을 놓친다.
20. simulator capture 경로가 재사용되거나 파일을 덮어써 Android와 iOS 또는 서로 다른 source revision의 화면을 혼동한다.

## 구현 전 수용 기준

- Android/iOS build flag, Cargo feature, C define, Swift condition, Java source set과 route argument가 같은 opt-in flag에 묶인다.
- fixture disabled 기본 빌드에서는 C15 FFI 심볼과 Noto font payload가 없어야 하고, opt-in positive control에서는 둘 다 있어야 한다.
- 두 simulator에서 C15.3 전용 FFI 생성·eval 로그와 `c15_3_text=attached`가 확인되어야 한다.
- Android와 iOS 각각 캡처를 분리 저장하고 source revision·simulator identity·viewport·DPR·capture 시각을 기록한다.
- surface pixel clip 경계는 renderer readback test로 검사하고, simulator screenshot은 앱 화면 증거로 별도 분류한다.
- layout consumer, runtime text admission, CSS overflow ancestor mapping, hit-test와 C15 완료 판정은 이 app route로 주장하지 않는다.
