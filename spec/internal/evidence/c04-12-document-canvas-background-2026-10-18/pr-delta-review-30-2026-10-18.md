# C04.12 문서 Canvas 배경 PR diff 적대 검토

**PR:** [#309](https://github.com/ohah/spinon/pull/309)
**핵심 diff 검토 기준:** `0e9194f548ea1eb073bb0abde4bf499cc4f8c36c..995435763909e8da8165abca57a804bedfe0ef1b`
**후속 문서 검토 기준:** `995435763909e8da8165abca57a804bedfe0ef1b..ea46dce6`
**범위:** 구현·명세·증거·PR 본문과 라벨.

| # | 독립 실패 관점 | 최종 확인 |
|---:|---|---|
| 1 | 불투명 html보다 body 색을 먼저 고르는 cascade 순서 | 순수 resolver와 세션 테스트가 computed html `(18,52,86)`을 우선한다. |
| 2 | 투명 root에서 body의 inline/stylesheet 색이 사라짐 | inline 기본 앱 및 stylesheet/`var()` 세션 테스트에서 body sRGB가 Canvas 결과까지 전파된다. |
| 3 | HTML body node가 없을 때 잘못된 색이나 panic | body `None` 분기는 scheme fallback이며 unit test가 light/dark 결과를 검사한다. |
| 4 | root와 body가 투명할 때 OS scheme의 반영 범위 | `#ffffff`/`#121212`는 fallback에서만 선택되고, Chrome light/dark 표본 및 Rust 테스트가 구분한다. |
| 5 | alpha 0을 opaque 색으로 양자화 | body alpha 0을 transparent로 보고 dark fallback을 쓰는 세션 테스트가 통과한다. |
| 6 | 무관한 반투명 body가 opaque root Canvas를 막음 | 초안 resolver의 순서 결함을 발견해 수정했다. root opaque + body alpha 0.5 end-to-end 테스트에서 root 색을 반환한다. |
| 7 | 선택되는 반투명 후보를 임의 색으로 게시 | transparent root/body의 선택 후보 alpha 0.5는 `-15`로 실패한다. 불투명 root가 우선이면 표현 가능한 body partial alpha는 무시한다. |
| 8 | 8-bit 경계 밖 sRGB를 일반 반올림 | exact byte 경계 검사와 Chrome fractional-channel oracle의 CSSOM 230 / pixel 229 차이를 대조해 해당 색을 거부한다. |
| 9 | HSL/HWB/wide gamut을 sRGB source처럼 취급 | Stylo absolute color의 source-space tag가 보존되며 sRGB 외 값은 Canvas converter에서 실패한다. HSL/HWB와 fractional 입력 negative case가 있다. |
| 10 | NaN·무한대·0..1 밖 channel 및 alpha | absolute color 변환의 finite 검사와 Canvas 경로의 범위 검사를 확인했다. byte cast 전 실패한다. |
| 11 | Stylo가 계산 중 제거하는 system-color keyword | stylesheet/inline computed style 전에 parsed declaration을 검사해 `Canvas`와 `CanvasText`를 실패시킨다. |
| 12 | `var()` custom property에 숨은 system color | 선언 scanner가 background-color와 custom property를 검사하고 stylesheet 및 inline 회귀 입력이 거부된다. |
| 13 | 함수 중첩·문자열의 키워드 오인 및 malformed CSS | cssparser nested token을 재귀하고 string token은 색 identifier로 보지 않는다. 파서 오류는 fail-closed다. |
| 14 | `@media` 안의 선언을 검사하지 않거나 비활성 분기를 놓침 | media 자식 규칙을 모두 검사한다. 현재 query 결과와 상관없이 unsupported system color가 있으면 profile 전체를 거부하는 제한이 기록됐다. |
| 15 | inline `style`에 있는 system color 누락 | document view를 순회하며 모든 inline background-color/custom property를 별도로 확인한다. |
| 16 | gradient/image를 평탄한 단색으로 조용히 생략 | 지원 속성 allowlist 밖에서 cascade 오류를 반환하며 gradient regression test가 있다. |
| 17 | foreign namespace의 `body`를 문서 body로 선택 | root HTML namespace와 직접 자식 HTML `body`를 함께 확인한다. 찾지 못하면 body fallback을 사용하지 않는다. |
| 18 | body ID와 computed style snapshot이 서로 다른 문서에서 옴 | body ID와 style snapshot은 같은 calculation request에서 만들어진 `RuntimeUaCascadeRoot`에 실린다. node/style 불일치는 `MissingRootStyle`로 실패한다. |
| 19 | synthetic html/body validator 완화가 기존 profile로 번짐 | non-transparent background 예외는 root HTML 계산 profile에서만 켜고, 다른 runtime layout profile 분기는 유지한다. |
| 20 | 새 background-color allowlist가 Flex 등 기존 profile을 넓힘 | `background-color`와 새 converter 분기가 root HTML media/custom-properties profile에만 연결돼 있다. 기존 paint profile 테스트도 유지된다. |
| 21 | document/style/environment 갱신 중 stale 색 게시 | generation, document, render-tree, style, environment key와 snapshot viewport revision을 모두 일치시킨 뒤 반환한다. |
| 22 | 명시 root 색을 metadata/OS scheme이 덮음 | used scheme은 fallback 용도로만 전달되고 opaque root/body 색이 우선한다. `color-scheme` media 재계산은 환경 revision을 증가시킨다. |
| 23 | JavaScript-only session에 root HTML 동작이 유입 | `new()`에는 root HTML metadata가 없고 기존 오류·constructor 경계가 유지된다. root profile만 새 cascade 로직을 호출한다. |
| 24 | FFI 오류 코드 누락, output 부분 기록 또는 timeout 우회 | `-15`가 Rust·C header·0077에 동기화됐고, resolver 성공 뒤에만 output struct를 쓴다. null·timeout upper bound 검증도 유지한다. 마지막 diff에서 발견한 주석의 중복 `중복`은 commit `99543576`에서 수정했다. |
| 25 | Android evidence가 다른 빌드나 다른 AVD를 가리킴 | 최신 branch 코드로 Android build가 성공했고 `emulator-5554` API 37 ARM64에서 기본 app root boot와 `canvas_color=#1e8c46`를 기록했다. configuration change 뒤에도 성공했다. |
| 26 | Android screenshot에 색 없는 영역 또는 sample 위치 혼입 | 1080×2400 PNG의 content 표본 3곳이 `(30,140,70,255)`다. 화면은 native content root 색으로만 설명했다. |
| 27 | iOS evidence가 실기기·잘못된 bundle을 가리킴 | iPhone 17 Pro / iOS 26.2 Simulator 빌드 성공, bundle ID `dev.spinon.bootstrap`, viewport `402×874`, DPR 3, root boot pass와 Canvas 색을 로그에서 확인했다. |
| 28 | iOS screenshot·asset이 source와 다름 | 1206×2622 PNG 세 표본이 Android와 같은 RGBA8이다. `index.html` source와 iOS app bundle SHA-256이 일치한다. AVD·Simulator 외 실기기 주장은 없다. |
| 29 | PR 화면 이미지가 링크 오류·읽을 수 없는 이미지거나 browser가 남음 | Agent Browser에서 PR #309 본문을 열어 raw Android/iOS 이미지가 둘 다 렌더됨을 눈으로 확인하고 session을 닫았다. `pgrep -x agent-browser`는 프로세스가 없음을 반환했다. |
| 30 | 증거·상태·merge gate가 검증 범위를 과장 | `STATUS`의 C04.12 상위 체크/완료 수를 유지하고 0077은 draft로 남겼다. plan·oracle·implementation review·이 PR review를 연결하고 checksum을 검증했다. runtime 182 passed/3 ignored, style 179, integration 3·16 및 Clippy가 통과했다. GitHub check rollup과 required checks는 비어 있으며 branch protection은 선형 이력만 강제하므로 병합은 Rebase and merge로 해야 한다. |

## 판정

30개 실패 관점을 확인했다. 처음 실제 PR diff에서 FFI 주석 중복 단어를 발견해 수정했고, 수정된 최종 head `99543576`에서 다시 확인했다. 다른 blocker는 남지 않았다. 검증은 이 제한 Canvas/background slice에만 해당하며 C04.12 전체 HTML rendering·body box paint·system UI palette·compositor pixel equivalence 또는 실기기 증거가 아니다.

## 후속 문서 diff 확인

- `ea46dce6`의 추가분은 PR 검토 링크, 구현 계획·계약·상태 문서의 해당 링크, SHA manifest 및 이 검토 기록이다. 제품 코드와 플랫폼 소스는 바뀌지 않았다.
- README·계획·0077·STATUS에 새로 추가한 PR review 상대 링크가 파일로 해석되고, C04.12 상위 체크는 미완료이며 내부 계약 숫자 버전은 `0.1.0-draft`로 유지됨을 확인했다.
- SHA manifest는 plan review를 포함해 30개 증거 파일을 검증하며 manifest 자신은 포함하지 않는다.
- PR 본문에는 이 검토 기록 링크가 추가됐고, 두 raw screenshot URL은 그대로다. 직전 Agent Browser 화면에서 두 이미지가 렌더되는 것을 확인했다.
- PR status rollup과 branch protection의 required checks는 비어 있다. 로컬 runtime/style 테스트, 두 integration target, Clippy는 통과했다. 병합 방식은 repository의 선형 이력 규칙에 따라 Rebase and merge다.
- 이 기록 정리와 checksum 재생성 뒤 `30`개 numbered review 관점, 30개 manifest 경로 및 네 신규 상대 링크를 재검증했다. 이 마지막 후속 diff에는 제품 코드나 캡처 변경이 없다.
