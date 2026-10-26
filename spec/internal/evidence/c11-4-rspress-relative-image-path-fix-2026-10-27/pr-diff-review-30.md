# C11.4 Rspress 상대 이미지 경로 수정 적대 검토 · 30개 관점

- 검토 브랜치: docs/c11-4-rspress-image-paths-20261027
- 기준: origin/main / HEAD fcaba40af649a1a33dacad5b1a35d10e41d29f89
- 검토 diff: runtime README, precomparison README, precomparison SHA256SUMS의 추적 파일 3개
- 확인: Rspress 2.0.22 실패 로그, 수정된 selected-root 문서와 기존 selected output, 실제 이미지 파일, source-revision.json, 8개 체크섬
- 방식: read-only diff·파일·해시 대조. 새 docs build/test/browser 실행은 하지 않았다.

## 판정

**PASS 30 / PARTIAL 0 / FAIL 0.** 네 Markdown 이미지 링크 모두 ./ 상대 경로로 수정됐고 각 원본 PNG가 존재한다. Rspress 2.0.22의 재현 로그는 ./ 없는 경로를 모듈 import로 처리하고 ./를 해결책으로 제안한다. 기존 selected output의 렌더 페이지는 네 이미지에 대응하는 hash-named static asset을 포함하고, 그 asset bytes는 원본 PNG와 일치한다. precomparison SHA256SUMS는 8/8 검증됐다.

source-revision.json은 runtime 캡처 당시의 source snapshot 및 app hash를 기록하는 manifest로 그대로 유지됐다. 이번 추적 변경은 문서 두 개와 checksum manifest뿐이며 앱 코드, runtime result/log, simulator metadata, 원본 캡처 PNG, source-revision.json은 변경되지 않았다. 이 검토에서는 docs build를 새로 실행하지 않았다.

## 적대적 검토 30개

| # | 실패 관점 | 판정 및 근거 |
|---:|---|---|
| 1 | 이미지 경로 수정 외에 앱이나 무관한 파일이 추적 diff에 섞인다. | **PASS** — 추적 diff는 runtime README, precomparison README, precomparison SHA256SUMS 세 파일이며 app/runtime source는 없다. |
| 2 | runtime README의 Android 경로에 ./가 빠져 Rspress가 다시 bare module로 해석한다. | **PASS** — Android PNG 링크가 ./android-avd-c114-fit-content-auto-neighbor-android-avd-20261026T190540661Z-27b72972/fit-content-auto-neighbor-wpt.png로 시작한다. |
| 3 | runtime README의 iOS 경로에 ./가 빠져 같은 오류가 남는다. | **PASS** — iOS PNG 링크가 ./ios-simulator-c114-fit-content-auto-neighbor-ios-simulator-20261026T190530681Z-85dd0569/fit-content-auto-neighbor-wpt.png로 시작한다. |
| 4 | precomparison README의 첫 Chrome 캡처만 기존 module-resolution 오류를 재현한다. | **PASS** — 첫 캡처 링크가 ./chrome-wpt-live-fit-content-auto-neighbor.png로 수정됐다. |
| 5 | precomparison README의 target 캡처는 수정되지 않았다. | **PASS** — target 링크도 ./chrome-wpt-live-fit-content-auto-neighbor-target.png로 수정됐다. |
| 6 | ./를 붙였지만 경로 기준이 repo root 또는 잘못된 폴더다. | **PASS** — Markdown 경로는 각 README가 있는 디렉터리를 기준으로 해석되고 네 대상 파일 모두 해당 디렉터리 아래 실제로 존재한다. |
| 7 | 링크 파일명 대소문자·확장자 또는 scene ID가 PNG 파일과 맞지 않는다. | **PASS** — Android/iOS runtime PNG 두 개와 Chrome precomparison PNG 두 개가 정확히 일치하는 이름·대소문자로 존재한다. |
| 8 | 에러가 Rspress 이미지 import와 무관하거나 다른 버전에만 해당한다. | **PASS** — 실패 로그의 import trace는 @rspress/core 2.0.22이고 Markdown image import에서 Module not found를 기록한다. package.json과 bun.lock도 2.0.22다. |
| 9 | 제안된 ./ 수정이 로그의 원인과 맞지 않는다. | **PASS** — 로그는 bare 경로를 module request로 해석했다고 설명하고 각 경로에 Did you mean './...'를 제안한다. 현재 변경이 그 제안과 같다. |
| 10 | 변경이 문서 경로를 무관한 절대 URL이나 root-relative URL로 바꾼다. | **PASS** — 네 링크는 각각 README 디렉터리 기준의 상대 이미지 경로를 유지하며 image source를 다른 route로 바꾸지 않았다. |
| 11 | Android 링크는 이름만 같고 잘못된 PNG bytes를 사용한다. | **PASS** — 원본 Android PNG SHA-256은 35b3d269a54e01f7019215f74c4aa6ccbec6f62c6afaf83527c30e1366f5ad7c다. selected output의 fit-content-auto-neighbor-wpt.f1b96bb90d.png도 같은 hash다. |
| 12 | iOS 링크는 이름만 같고 다른 캡처 bytes를 사용한다. | **PASS** — 원본 iOS PNG SHA-256은 d4d94e106634755cabd3dd204de3e241ec47a336f96127cf15ef36dee19fb51f다. selected output의 fit-content-auto-neighbor-wpt.ffa167036e.png도 같다. |
| 13 | 첫 Chrome precomparison 이미지는 누락되거나 다른 파일로 빌드된다. | **PASS** — 원본 chrome-wpt-live-fit-content-auto-neighbor.png와 selected static asset SHA-256이 e387f20ad60768800d6d085a966f89e2061bd5ee7eacb6fcc4db5dfae4a6b9bc로 일치한다. |
| 14 | 대상 assertion Chrome 캡처가 두 번째 링크에 잘못 연결된다. | **PASS** — target PNG는 별도 경로를 가지며 원본과 selected static asset SHA-256이 cad9624bc782cdd512ea3fe40378d4a5cd47ad369c9563641e9710a3062e4232로 같다. |
| 15 | 수정 후 생성된 selected output에 runtime/precomparison 이미지가 빠진다. | **PASS** — 기존 selected output의 두 README.html에는 각각 두 캡처 img가 생성됐고 네 hash-named static assets가 모두 존재한다. |
| 16 | image 파일이 없는 상태에서도 HTML만 생성되어 성공처럼 보인다. | **PASS** — 네 static output asset의 존재와 바이트 길이·SHA를 원본 파일과 대조했다. 이 보고서는 별도 새 build pass를 주장하지 않는다. |
| 17 | image alt text나 캡션을 수정하면서 플랫폼/캡처 의미가 바뀐다. | **PASS** — diff는 image destination 앞에 ./를 추가했으며 Android/iOS 및 Chrome 캡션·alt 문구는 유지한다. |
| 18 | runtime README의 source-revision 설명과 실제 캡처 manifest가 충돌한다. | **PASS** — source-revision.json의 sourceRevision에는 캡처 commit/tree, trackedDiffSha256, trackedDirtyPaths, sourceSnapshotSha256가 기록돼 있고 이번 diff에서 manifest 자체는 바뀌지 않았다. |
| 19 | 과거 capture-time source snapshot을 현재 README 수정 후 소스 해시로 오인한다. | **PASS** — 새 문단은 manifest snapshot이 캡처 당시 app/runtime 입력이라고 밝히며 이번 문서 수정이 앱·sim 실행·캡처 pixel을 바꾸지 않는다고 한정한다. 현재 checkout이 그 과거 snapshot과 같다고 주장하지 않는다. |
| 20 | 이번 doc-only 수정이 Android/iOS 제품 코드를 바꿨는데 숨긴다. | **PASS** — git diff --name-only에는 플랫폼 코드·Rust source·build configuration이 없다. |
| 21 | APK/iOS app binary가 새로 빌드되어 이전 runtime capture와 다른데 같은 증거로 제시된다. | **PASS** — 이번 diff에 build log나 app manifest 변경은 없고 README도 새 앱 빌드를 주장하지 않는다. |
| 22 | Android/iOS result.json에 수정 이후 다른 결과가 섞인다. | **PASS** — 두 result.json은 diff에 없으며 capture-time manifest가 가리키는 run ID와 결과 경로가 유지된다. |
| 23 | runtime logs 또는 PixelCopy/Metal sample이 업데이트되어 캡처 기준이 섞인다. | **PASS** — runtime log, install verification, simulator metadata와 sample logs는 모두 변경되지 않았다. |
| 24 | source-revision.json 자체가 새 문서 수정 후 임의로 갱신된다. | **PASS** — source-revision.json은 diff에 없고 현재 변경은 README와 precomparison checksum manifest에 한정된다. |
| 25 | Android/iOS screenshots가 수정되거나 capture manifest의 screenshot hash와 달라진다. | **PASS** — 두 PNG는 diff에 없으며 실제 bytes는 unchanged source-revision screenshot hashes와 일치한다. |
| 26 | Chrome 관찰 JSON이 바뀌어 README가 가리키는 값과 캡처 증거가 달라진다. | **PASS** — chrome-wpt-live-observation.raw.json, expected.json, source-pins.json은 변경되지 않았다. |
| 27 | Agent Browser version 또는 cleanup evidence가 바뀌었는데 checksum은 덮어쓰지 않는다. | **PASS** — 해당 파일들은 수정되지 않았고 SHA256SUMS 검증에서 각각 OK다. |
| 28 | checksum manifest가 잘못된 README hash를 쓰거나 일부 entries를 빠뜨린다. | **PASS** — manifest는 8 entries이며 수정된 precomparison README hash d2786af78e55ebe894c77eb854c4a567d0481403b8021c3f530126527f051187를 포함한다. shasum -a 256 -c SHA256SUMS 결과 8/8 OK다. |
| 29 | 실패 로그를 수정 후 성공 증거로 과장한다. | **PASS** — rspress-build-image-path-blocker-pass01.log.raw는 pre-fix Module not found와 exit 1을 기록한다. 현재 보고서는 그 실패를 원인 증거로만 사용하며 새 docs build를 실행했다고 쓰지 않는다. 기존 selected output의 생성 파일만 별도 경로 확인에 사용했다. |
| 30 | app/evidence 변경이 없다는 note가 원본 캡처 또는 raw evidence 변경을 숨긴다. | **PASS** — 실제 diff는 README 두 개와 checksum manifest뿐이다. 캡처 PNG, raw JSON, result/log, simulator metadata, source-revision manifest는 변경되지 않았다. 문서/checksum 변경 자체를 app/capture 변경으로 표현하지 않는다. |

## 결론과 한계

- 문서의 네 로컬 이미지 링크는 Rspress module-resolution 오류를 피하는 경로이고 각 대상 파일 및 selected output asset을 확인했다.
- precomparison SHA256SUMS 8개 항목은 전부 통과했다.
- source-revision.json은 capture-time snapshot으로 유지되며 이번 README/checksum 변경은 앱 실행이나 캡처 pixel의 재생성이 아니다.
- 새 Rspress build는 검토자가 실행하지 않았다. 실패 로그는 원인 확인에만 썼고 기존 selected output의 HTML/asset 결과를 대조했다.
- stage 또는 commit하지 않았다.
