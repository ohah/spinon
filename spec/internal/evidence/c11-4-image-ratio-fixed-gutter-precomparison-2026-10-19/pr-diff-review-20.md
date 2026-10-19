# C11.4 fixed-gutter 계획 PR diff 검토 · 20개 실패 관점

대상: 집중 계획, C11.4 상위 계획·공식 상태·공개 roadmap 변경, Chrome fixture와 사전 비교 자료 전체. 구현 PR 검토와 구분해 검토했다.

| # | 실패 관점 | 확인 기준 | 결과 |
| ---: | --- | --- | --- |
| 1 | 계획 PR이 제품 구현을 몰래 포함하는가 | 변경 파일은 계획·상태·roadmap·fixture·관찰/검토 자료이며 Rust/C/Swift/JS 제품 계산 코드는 없다. | PASS |
| 2 | C11.4 완료나 체크 수가 조기 증가하는가 | 공식 `C11.4`는 계속 `[ ]`; 새 줄은 하위 계획 설명이며 체크 항목이나 완료 수를 추가하지 않는다. | PASS |
| 3 | Simulator나 실기기 통과를 주장하는가 | plan, README, STATUS, public roadmap 모두 구현·Rust 회귀·Android/iOS 실행이 아직 없다고 적는다. | PASS |
| 4 | Chrome을 규범적 정답으로 취급하는가 | §6.6 기대값과 Chrome 155 관찰을 별도 열·문장으로 기록하고 차이를 interop 차이로 남긴다. | PASS |
| 5 | 표준 문서 버전이 떠다니는가 | 계획과 oracle은 고정된 2025-03-26 CSS Grid CRD §6.6 링크를 쓴다. | PASS |
| 6 | item span 밖 track/gutter가 한도에 포함되는가 | middle-span fixture는 양 끝에 큰 track을 두고 span 내부의 `3px + 3px gap + 4px`만 기대값에 사용한다. | PASS |
| 7 | gutter를 누락하거나 두 번 더하는가 | 2-track/3-track 및 zero-gap 값표가 `N−1` 내부 gutter를 적용한다; 구현 조건은 이미 반영된 geometry에 재가산하지 말라고 명시한다. | PASS |
| 8 | row와 column 중 한 축만 다루는가 | fixture와 기대 표에 두 방향의 2·3-track span 및 middle-span 사례가 있다. | PASS |
| 9 | `minmax(auto, fixed)` max와 고정 min을 혼동하는가 | fixed max function을 대상으로 제한하고 `fit-content()` 및 flexible max를 negative control로 둔다. | PASS |
| 10 | `fit-content()`와 `fr`이 all-fixed predicate를 통과하는가 | 두 별도 control 장면은 clamp를 적용하지 않아야 한다고 계획과 oracle에 적는다. | PASS |
| 11 | content-box 변환에서 padding/border를 중복 처리하는가 | area limit `10px`에서 source edges `4px`를 한 번 빼고 ratio 후 target edges `4px`를 한 번 더하는 값표가 있다. | PASS |
| 12 | natural ratio와 authored ratio를 혼동하는가 | fixture는 natural `24×16`, authored `3/2`, preferred input `36px`를 별도 수집한다. | PASS |
| 13 | Chrome used track 값을 고정 max function으로 오인하는가 | README는 Chrome used track list(일부는 authored max를 초과)를 실제 관찰로 보존하고 §6.6 기대 limit은 max sizing function에서 계산한다고 분리한다. | PASS |
| 14 | fixture 장면 ID가 중복되거나 matrix가 빠지는가 | Agent Browser가 그린 fixture에는 고유 ID 10개가 있고 zero-gap·gutter·span·control·content-box 사례가 모두 포함된다. | PASS |
| 15 | 관찰 환경 metadata가 결과와 맞지 않는가 | normalized JSON 확인: Chrome `155.0.8059.85`, macOS `26.5.1` ARM64, DPR 1, inner viewport `1280×633`; README도 같은 값을 쓴다. | PASS |
| 16 | screenshot이 Spinon 또는 mobile pixel 검증으로 과장되는가 | README는 Chrome 페이지 화면을 표시하며 Spinon·Android/iOS pixel 증거가 아니라고 제한한다. | PASS |
| 17 | 저장된 fixture/관찰값/screenshot이 바뀌었는데 hash가 오래됐는가 | `shasum -a 256 -c SHA256SUMS`에서 fixture, normalized/raw JSON, screenshot, plan review가 모두 일치했다. | PASS |
| 18 | 새 Markdown 링크가 문서 빌드를 추가로 깨뜨리는가 | Rspress 전체 build는 기존 문서의 dead link로 실패했지만 최종 로그에 이 작업의 새 경로 진단은 0건이다. | PASS |
| 19 | 계획 검토 20회를 PR diff 검토로 중복 계산하는가 | `plan-review-20.md`에 20개 번호 행이 있고, 이 표는 별도 변경 경계·문서·fixture 실패 관점으로 작성했다. | PASS |
| 20 | 원본 C11.3 사용자 작업이 섞이거나 바뀌는가 | 이 PR branch는 최신 `origin/main`의 `5b7f62f2`에서 분리했고 원본 C11.3 checkout은 수정하지 않는다. | PASS |

문서 build 한계: `packages/docs`에서 실행한 `mise exec -- bun run docs:build`는 저장소 안의 기존 여러 dead link 때문에 exit 1이다. 해당 실행에서 이 PR 범위의 새 dead-link 진단은 발견되지 않았다. 이 검토는 구현이나 Simulator 결과를 대신하지 않는다.
