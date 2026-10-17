# C15.3 HostDocument text paint · 구현 실패 관점 20개

**검토 범위:** `feat/c15-3-hostdocument-text-paint-20261017`의 HostDocument→layout snapshot→glyph raster→WGPU fixture 경로와 플랫폼 adapter 소스. 계획 검토 표를 재사용하지 않았다.

**검토 종류:** 소스·불변 조건·전체 FFI·앱 실행·Android PixelCopy·iOS Metal drawable·artifact 경계 확인.

**상태:** 전체 FFI 49/49 passed(2 ignored), Android PixelCopy 6/6, iOS Metal drawable 6/6, default/opt-in archive symbol/font 경계를 확인했다.

## 공격 관점

| # | 실패 관점 | 공격·통제와 확인 결과 |
|---:|---|---|
| 1 | 제품 빌드가 fixture code/font를 우연히 끌어온다 | 최종 분할 뒤 default `libspinon_ffi` archive에서 두 C15.3 fixture FFI symbol 및 Ahem/Noto Sans KR 전체 byte payload를 찾지 못했다. opt-in positive control에서 symbol과 두 font payload를 모두 찾았다. |
| 2 | 기존 C15.3 glyph fixture가 새 경로에 덮인다 | host의 새 bool은 별도 필드이며 생성자는 새 경로에서만 설정한다. `wait_and_publish`는 HostDocument flag 우선, 기존 glyph flag는 기존 adapter로 분기한다. 전체 FFI suite 49개가 통과했고, 현재 HostDocument 앱 경로는 Android/iOS 양쪽에서 별도 표면 결과를 냈다. |
| 3 | 초기화되지 않은 flag가 새 route를 켠다 | `RuntimeGpuHost` 기본 생성에서 새 flag를 `false`로 초기화한다. Android/iOS argument도 fixture feature가 켜졌을 때만 활성 route가 된다. |
| 4 | 앱 argument 오타가 legacy fixture로 조용히 fallback한다 | Android는 별도 boolean extra/JNI constructor를 사용한다. iOS 첫 실행은 C13 route validator가 HostDocument argument를 거부해 실제 결함을 드러냈고, validator allowlist와 regression assertion을 수정했다. 이후 Simulator launch가 전용 route로 성공했고 PixelCopy/Metal summary가 모두 `matched`다. |
| 5 | Android JNI가 기본 Java artifact에서 unresolved native method가 된다 | adapter는 C15.3 전용 Java source set에만 놓고 main demo에서는 reflection으로 접근한다. API 37 AVD Gradle APK build, install, launch와 PixelCopy 6/6이 통과했다. |
| 6 | iOS 기본 target에서 fixture selector가 export된다 | header/Objective-C++ 선언은 C15.3 fixture define guard 안에 있다. feature-enabled iOS simulator app은 Xcode build/install/launch에 성공했고 Metal drawable 6/6을 읽었다. Rust default/opt-in archive 경계도 독립 확인했다. |
| 7 | HostDocument가 빈 상태거나 text handle이 다른 document에서 온다 | fixture가 단일 `HostDocument` 안에서 root, paragraph, text handle을 예약하고 하나의 `DocumentChangeBatch`로 create/insert한다. commit이 실패하면 생성 함수가 error를 반환한다. |
| 8 | 잘못된 node parent 순서로 root가 아닌 tree가 된다 | root는 `HostParent::Root`, p는 root child, Text는 p child로 삽입하며 각 element는 XHTML `div`/`p`다. commit snapshot과 layout source revision을 이후 검사한다. |
| 9 | paragraph가 다른 frame owner를 갖는다 | Taffy output에서 paragraph `LayoutFrame`을 조회하고 frame이 `0,0,15,10`인지 확인한 뒤 같은 frame을 runtime text origin/paragraph box로 전달한다. |
| 10 | owner 폭과 높이를 상수로 paint에 재구성한다 | paragraph box는 `LayoutFrame`에서 구성한다. layout 검증은 available width 15, text width 20, height 10, 한 줄 advance 20, baseline 8을 확인한다. |
| 11 | `overflow:hidden` 기본값 때문에 두 번째 X가 잘린다 | root와 paragraph 양 축에 `LayoutOverflow::Visible`을 명시하고 RuntimeRenderText에 unconstrained clip을 준다. Android PixelCopy와 iOS drawable 모두 owner 경계 바깥 두 번째 glyph의 4/4 표본을 정확한 잉크로 읽었다. |
| 12 | layout 결과가 다른 HostDocument snapshot에서 왔다 | `LayoutSourceRevision::HostDocument`의 generation·document revision·render-tree revision이 fixture snapshot과 정확히 같아야 한다. |
| 13 | stale layout/text revision을 그린다 | input, output, `LayoutTextSnapshot` revision이 전부 같아야 한다. negative test는 변경된 output revision을 넣어 paint 전 거부하는 경로를 확인했다. |
| 14 | text node가 아닌 source 또는 inline span으로부터 문자열을 조립한다 | `LayoutTextSnapshot.input`은 paragraph owner, 직접 Text node 하나, 문자열 `XX`, 빈 inline ancestor를 요구한다. mismatch는 거부한다. |
| 15 | UTF-16/paragraph byte range가 glyph 위치와 어긋난다 | 두 source span `[0,1]`, `[1,2]`와 UTF-8 byte span을 검사한다. 두 shaped glyph의 source node/range, x 위치 0/10, advance 10을 검사한다. |
| 16 | layout과 raster가 서로 다른 font registry를 쓴다 | 하나의 `TextLayoutEngine`에 Ahem을 등록하고 같은 engine으로 Taffy measure와 raster를 모두 수행한다. 이전 구현에서 registry revision 불일치를 재현해 이 결함을 수정했고 focused test 2/2가 통과했다. |
| 17 | glyph 순서, ID, mask payload가 다른 layout에서 온다 | 별도 raster 모듈에서 glyph ID/source와 alpha-mask payload를 검증하고 sample 추출은 pixel buffer를 bounds-check한다. runtime bridge는 bitmap geometry와 buffer를 그대로 전달한다. |
| 18 | sample 좌표가 임의 보정이거나 paragraph 안에만 있다 | 좌표는 raster mask coverage 255에서 유도한다. 첫 glyph 표본은 `x<15`, overflow glyph 표본은 `x>=15`, viewport 안쪽이어야 하며 중복 좌표는 거부한다. Android PixelCopy와 iOS Metal drawable이 이 sample들을 모두 불투명 `[9,33,112,255]` 잉크로 읽었다. |
| 19 | paint command가 다른 owner/key를 참조한다 | RuntimeRenderText owner는 paragraph `NodeId`, origin은 같은 layout frame, scene/text key는 runtime 입력 key를 사용한다. draw 순서는 root box→paragraph box→text이며 최종 scene key 검사에 걸린다. |
| 20 | 앱이 report만 보고 pixel을 통과 처리한다 | 두 플랫폼은 report 문자열과 분리된 surface readback을 수행했다. Android summary `status=matched`, iOS `SPINON_C153_HOSTDOCUMENT_METAL_PIXEL_SUMMARY status=matched`이며 각각 inside 2/2·overflow 4/4다. iOS에서는 route rejection을 고친 뒤 fresh install/launch 및 readback을 다시 확인했다. |

## 실행 근거와 남은 통제

- 전체 opt-in FFI suite가 `49 passed; 0 failed; 2 ignored`로 통과했다. 두 ignored test는 기존 Android/iOS 앱 실행용 V8 runtime tests다. 명령은 [full-ffi-test-command.txt](full-ffi-test-command.txt), 로그는 [full-ffi-test.log](full-ffi-test.log)다.
  ```sh
  CARGO_TARGET_DIR=/Volumes/ZTSBuild/spinon-c15-3-20261017/target \
    CARGO_BUILD_JOBS=1 CARGO_PROFILE_TEST_DEBUG=0 CARGO_INCREMENTAL=0 \
    cargo test --locked -p spinon-ffi --features c15-3-wgpu-glyph-paint-fixture-test
  ```
- `mise exec -- node --test tools/css-reference/fixture-module-boundary.test.mjs` 6/6, `git diff --check` 통과.
- 최종 소스 보정 뒤 `cargo test` 49 passed/0 failed/2 ignored, feature-enabled Clippy와 workspace Clippy가 모두 `-D warnings`로 통과했고 `cargo fmt --all -- --check`도 통과했다. [최종 lint 기록](rust-lint-final.md).
- Android API 37 ARM64 AVD APK build/install/launch 성공. PixelCopy 6/6 exact; image hash는 [sha256.txt](android-api37-avd-20261017T082555Z/sha256.txt), 화면과 raw readback은 같은 디렉터리다.
- iPhone 17 Pro/iOS 26.2 Simulator build/install/launch 성공. Metal drawable 6/6 exact, `renderer=metal-drawable-before-compositor`; Xcode/run log와 screenshot은 [route-fix capture](ios-simulator-iphone17pro-route-fix-20261017T083816Z/ios-xcodebuild.log)에 연결된 디렉터리에 있다.
- 최종 분할 소스의 default/opt-in `libspinon_ffi` archive를 `nm -gU`와 exact font byte subsequence scan으로 비교했다. 결과 JSON은 [artifact-boundary-scan-final.json](artifact-boundary-scan-final.json)이다. 이는 Android/iOS 제품 archive 전체 symbol scan을 뜻하지 않는다.

이 고정 fixture 결과가 완성되어도 일반 DOM text admission, CSS font projection, C15.3 전체 또는 C15/C16 완료를 뜻하지 않는다. PR diff review/CI/merge는 이 기록 이후의 별도 관문이다.
