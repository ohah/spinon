# C11.4 cyclic percentage auto-repeat 행 profile PR 변경 검토 30개

## 검토 기준

- 원격 `main` fetch 후 `HEAD`, `origin/main`, `FETCH_HEAD`는 모두 `6150d4f289c0e2aabfd2b696748dc3c3be546827`이고 기준 tree는 `55b79d5616dfff4381766c83e1625053fd37c57a`다. 최종 staged PR은 71개 경로, 4,518 insertions / 63 deletions로, 이전 run의 PNG/log를 제외하고 최신 증거 및 review 기록을 포함한다.
- 이번 검토는 기존 계획 20개 및 구현/runtime 20개와 겹치지 않게 staged PR의 통합 경계, 실행물 provenance, fixture artifact 한계와 문서 전달을 적대적으로 확인했다. 기존 결과를 새 테스트로 재실행하지 않았고, 빌드·테스트·브라우저도 실행하지 않았다.
- 결과 표시는 `PASS`, `NONBLOCKING OPEN`, `POST-CREATE`, `POST-MERGE`로 구분한다. 최신 staged snapshot에서 **차단 결함은 발견하지 못했다.** 남은 항목은 PR 본문 이미지 렌더 확인, 제한 profile 외의 일부 mutation coverage, 병합 후 roadmap/Tailnet 동기화다.

## 기준·diff·scene 연결

1. **PR base가 검토 중 이동했는가 — PASS.** 원격 `main`을 fetch한 뒤 `HEAD`, `origin/main`, `FETCH_HEAD`가 모두 `6150d4f289c0e2aabfd2b696748dc3c3be546827`이고 merge-base도 같은 commit이다. 현재 staged delta의 기준을 갱신된 원격과 비교했다.

2. **무관한 작업물이 staged PR에 섞였는가 — PASS.** 현재 index에는 C11.4 구현·검증 자료만 있다. 기존 untracked evidence/cache 경로는 포함하지 않았고, `spec/STATUS.md`, `spec/internal/0059-c11-grid-layout.md`, `spec/public/roadmap.html`도 이번 PR에서 변경하지 않았다.

3. **최종 실행이 현재 코드와 문서 revision을 구분하는가 — PASS.** 양 simulator metadata의 capture-time tracked diff SHA는 `f0f43809c99079c416fcefa6d26a93098e6d59581d33a0b91b78390fbc312570`이며 전체 tracked diff hash다. 현재 code-only staged diff SHA `6e16c33893a15ad5e083cde731f0883b6f2e2931d92191c1a6de6c388032ac06` 및 source/build/capture 시각은 `source-revision-verification.json`에 있다. 캡처 후 변경은 evidence/document packaging이며 implementation source는 바뀌지 않았다.

4. **두 플랫폼이 서로 다른 소스 revision에서 나왔는가 — PASS.** Android/iOS 최종 `simulator.json`의 commit, tree, capture-time diff SHA 및 dirty-path 목록이 같다. Android APK SHA `9f045a13…ca77c81d`, iOS executable SHA `99be8687…71425d05`는 플랫폼별 설치본 검증과 각각 연결된다.

5. **캡처 이후 앱 소스가 바뀌어 화면 근거가 stale해졌는가 — PASS.** 관련 layout, test, Android, iOS source mtime은 두 앱 build와 캡처보다 이르다. 캡처 뒤 바뀐 tracked 자료는 evidence README/review이며 앱 source가 아니다.

6. **같은 evidence 폴더 안의 오래된 run을 최종 근거로 잘못 읽을 수 있는가 — PASS.** 이전 캡처 35개는 index에서 제외했다. README와 두 검토 문서는 각각 Android `20261025T182325255Z-dd43faa6`, iOS `20261025T182411249Z-0b566316` 최종 디렉터리로 직접 연결하고, 최신 폴더마다 별도 `runPrefix`, 결과, PNG hash manifest를 보관한다.

7. **run 이름의 UTC 시각과 로그의 한국 시각이 충돌하는가 — PASS.** 최신 Android run의 `20261025T182325255Z`와 iOS run의 `20261025T182411249Z`는 각각 runtime log의 2026-10-26 03:23, 03:24 KST와 맞는다. 각 result의 scene ID 및 run prefix가 4 scene 로그와 일치한다.

8. **선택적 capture가 네 scene 중 일부를 누락하거나 중복하는가 — PASS.** capture script는 중복 요청과 미해결 ID를 `:4946–4959`에서 거부한다. module-boundary test `:677–722`는 요청된 ID 순서와 `selectedSceneCount=4`를 확인하고 두 최종 `result.json`에도 각 기대 ID가 한 번씩 있어 scene count가 4다.

9. **fixture catalog 총수가 일부만 갱신됐는가 — PASS.** 새 네 scene을 추가한 뒤 capture catalog는 `:4943`에서 496, 중복 ID는 `:4944–45`에서 금지한다. 변경된 `fixture-module-boundary.test.mjs`의 catalog expectation도 496으로 갱신됐고 `tools/css-reference`에서 구 492 상수는 남지 않았다.

10. **AVD/Simulator 결과를 실기기 결과로 오인하게 쓰는가 — PASS.** Android metadata는 API 37 ARM64 AVD `emulator-5560` / `sdk_gphone16k_arm64`, iOS metadata는 iPhone 17 Pro / iOS 26.2 Simulator UDID를 적는다. README `:28, :61`은 software renderer, drawable readback 시점, 실기기 미검증을 구분한다.

## 실행물·pixel·artifact 증거

11. **Android 실행 앱이 빌드 결과와 다를 수 있는가 — PASS.** 최신 Android `simulator.json`과 `installed-apk-verification.json`의 build/install APK SHA-256은 `9f045a13d4a4839ca96be09bfdeff80e84ee3384e3fb762984e1334cca77c81d`로 같다.

12. **iOS 설치본을 기록된 app hash에 바이트 단위로 결합했는가 — PASS.** 최신 iOS app executable의 build와 설치본 SHA-256은 `99be8687d3f23b70dae8630437ae1dbb7ab71ff16c7ded9891452e9c71425d05`로 같다. `installed-app-verification.json`에 UDID, bundle ID, executable 및 일치 결과를 남겼다.

13. **최종 run JSON이 장면 하나를 빠뜨렸는가 — PASS.** Android와 iOS 최종 `result.json`은 각각 `sceneCount=4`, 기대 네 `sceneId`, root/child node frames, `drawPresented=true`, `supersededRetries=0`을 담는다. Rows/Columns 최종 track 값도 README `:9–14`의 네 행과 맞는다.

14. **README의 pixel 20/20 주장이 원본 로그와 어긋나는가 — PASS.** 최종 Android scene log의 `SPINON_C114_GRID_PIXEL_MATRIX_SUMMARY` 및 iOS `SPINON_C114_METAL_PIXEL_SUMMARY`는 각각 scene당 `expected=5, observed=5, matched=5`를 기록한다. 네 log를 합친 수가 README `:16`의 플랫폼별 20/20과 일치한다.

15. **PNG가 해당 run 이후 교체됐는지 감지할 수 있는가 — PASS.** 각 최신 run에는 네 PNG의 SHA-256 manifest가 있고, 검토 중 파일 hash를 다시 계산해 모두 일치시켰다.

16. **README에 적힌 최종 검증을 PR artifact로 추적할 수 있는가 — PASS, 범위 명시.** 최종 focused layout/runtime test, 세 크레이트 Clippy, rustfmt, fixture-boundary, Android/iOS build 및 capture 로그가 index에 있다. workspace 전체 테스트/기본 Clippy는 앞선 반복에서 통과했다고 README가 구분해 서술하며 이번 review가 다시 돌리지는 않았다.

17. **fixture archive negative/positive scan의 결과 원자료가 보존됐는가 — PASS, 범위 명시.** `ffi-default-final-archive-scan.json`과 `ffi-fixture-final-archive-scan.json`에 각 macOS ARM64 archive SHA, Rust toolchain, 심볼 목록, scene-string 목록이 있다. 실제 archive는 build output이므로 PR에는 포함하지 않았다.

18. **static archive 결과를 APK/IPA 전체 제외 증거로 확대했는가 — PASS.** README `:54–57`은 product/fixture-test archive를 positive·negative control로 비교하면서 Android APK/iOS IPA payload의 전체 제외를 증명하지 않는다고 명시한다. PR diff에는 이 한계를 지우거나 일반 product exclusion이라고 부르는 claim이 없다.

19. **기본 feature를 켜서 fixture layout을 제품으로 노출했는가 — PASS, 제한 해석.** 새 layout profile 모듈은 `grid.rs:5–6`에서 `c11-4-grid-track-sizing-fixture-test`로 감싸고 FFI module boundary test `:683–688`는 FFI fixture dispatcher의 feature gate를 확인한다. archive negative/positive control은 README `:54–55`에 있다. 이 판정은 Rust static archive 범위에 한정되고, 앱 adapter에 들어간 ID/검사 코드까지 APK/IPA에서 제거된다는 판정은 아니다.

20. **README/PR 본문이 화면을 inline으로 보여주는가 — POST-CREATE.** README는 대표 PNG 파일로 연결한다. PR 본문에는 Android/iOS PNG를 `![alt](raw-image-url)`로 embed할 계획이며, 생성 뒤 Agent Browser에서 렌더링을 확인해야 이 항목을 닫는다.

21. **pixel 결과가 border stroke 또는 root 바깥 overflow까지 입증하는가 — PASS, 범위 명시.** README `:16`과 계획 `:89`는 다섯 표본이 root 안의 overlap/track overflow를 다루며 border도 root 밖도 확인하지 않는다고 적는다. 따라서 색 표본을 사용자가 이전에 문제 제기한 border/outside 영역 테스트로 확대하지 않는다.

22. **renderer 결과가 최종 사용자 화면의 동일성을 뜻한다고 읽히는가 — PASS, 범위 명시.** README `:28`은 Android AVD software renderer와 iOS compositor 전 Metal drawable readback을 구분한다. 실기기 GPU, Android hardware GPU, iOS compositor 후 pixel을 검증했다고 쓰지 않는다.

## Fail-closed·status·후속 전달

23. **부분 fixture 구현을 공식 C11.4 완료로 승격했는가 — PASS.** `spec/STATUS.md:273`의 C11.4 항목은 unchecked이고 README `:61`과 계획 `:38`은 제한 scene 결과와 공식 `72/213`을 유지한다. 변경된 source가 있다고 체크박스나 공식 완료 수를 올리지 않았다.

24. **내부 계약·roadmap을 PR에서 완료 상태로 미리 바꿨는가 — POST-MERGE.** `0059`, STATUS, public roadmap은 현재 diff에서 변경되지 않았다. 계획 `:92`는 merge 뒤 실제 범위와 evidence를 반영하되 C11.4 checkbox 및 72/213을 유지하도록 예약하므로 지금 상태는 과장보다 안전하다. 병합 후 문서 동기화가 별도 완료되어야 한다.

25. **Tailnet 미리보기를 현재 PR evidence로 오인하게 하는가 — POST-MERGE.** 계획 `:92`는 merge 후 실제 root/config와 활성 process를 확인해 선택 파일을 동기화하고 local/served hash와 HTTP 상태, Agent Browser 화면을 확인하도록 적는다. 이번 staged PR에는 그런 serving/browser evidence가 없고 README도 배포 완료를 주장하지 않는다. 이 계획 단계가 끝나기 전에는 로드맵 미리보기 갱신 완료로 보고하지 않는다.

26. **계획의 fail-closed 변형표가 회귀 mutation으로 충분히 고정됐는가 — NONBLOCKING OPEN.** 이번 mutation 세트는 positioning key와 root sizing policy, named lines/areas, root box/style, child dimension/min-size 등 12개 추가 변형을 다룬다. 계획표의 child margin/padding/border와 일부 repeat 변형을 각각 독립 변형으로 고정하지는 않았으며, 현행 gate의 제한 프로필 범위를 넘는 조합 coverage gap으로 기록한다.

27. **positioning map에 엉뚱한 node key가 있어도 exact profile로 수용되는가 — PASS.** gate가 root/prefix/primary/secondary의 정확한 key 집합을 요구하고 generalized mutation test가 대체 key를 넣어 `UnknownPositioningNode` owner에서 거부되는지 확인한다.

28. **같은 style/grid라도 다른 root sizing policy가 opt-in을 통과하는가 — PASS.** gate는 허용된 `ResolveWithin` policy를 검사한다. `Match`는 `RootSizeMismatch`, `BlockFormatting`은 profile gate 오류로 배제하는 mutation이 있다.

29. **runtime trace에서 row는 맞지만 fixed column 진단이 빠지거나 변한 경우를 adapter가 거르는가 — PASS.** Android/iOS matcher는 Columns pass 1개, `[24px]` track, `FixedTrackEarlyReturn` stage를 검사하고, 최신 4-scene capture는 양 플랫폼에서 이 adapter 계약을 통과한다.

30. **Android가 필수 false boolean trace field 누락을 iOS와 똑같이 실패 처리하는가 — PASS.** Android adapter는 `gutter`/`collapsed`를 실제 Boolean으로 읽고 누락·다른 타입을 실패 처리한다. 최신 build/capture가 이 matcher를 포함한다.

## 결론

- **차단 결함:** 없음. 최신 Android/iOS 실행은 모두 4/4 scene, scene당 pixel 표본 5/5로 통과했다. 설치 앱 hash, screenshot manifest, 빌드·집중 검증 로그와 scan report가 index에 포함됐다.
- **남은 확인:** #20은 PR 본문 생성 뒤 Agent Browser에서 이미지 렌더를 확인해야 닫힌다. #26은 제한 profile 외의 일부 child box-decoration/repeat 조합을 별도 mutation으로 추가할 수 있는 비차단 coverage gap이다.
- **병합 후 별도 작업:** 24–25번. 공식 C11.4 checkbox 및 `72/213`은 그대로 두고 내부 계약·STATUS·public roadmap과 Tailnet 미리보기를 실제 병합 변경 범위에 맞춰 동기화한다.
- 검토 수행에서는 시뮬레이터를 다시 실행하거나 테스트·빌드를 돌리지 않았다. 최종에 `git diff --check`와 staged diff whitespace check를 별도로 실행한다.
