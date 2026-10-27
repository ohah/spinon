# C11.4 fit-content / definite min-height 사전 비교

## 목표

Pinned WPT `css/css-grid/grid-definition/fit-content-track-sizing.html`의 두 번째 assertion을 구현 전에 고정한다. 기대값은 WPT 문구나 Spinon 결과만 복사하지 않고 CSS Grid 2025 CRD §11.2·§11.5·§11.8에서 독립 계산한다.

## 출처와 브라우저 관찰

- WPT revision `c999c58338ee1d223df5ad62e48be1202ced0d37`.
- 대상 DOM은 [원본 27행](https://github.com/web-platform-tests/wpt/blob/c999c58338ee1d223df5ad62e48be1202ced0d37/css/css-grid/grid-definition/fit-content-track-sizing.html#L27), assertion은 [87–88행](https://github.com/web-platform-tests/wpt/blob/c999c58338ee1d223df5ad62e48be1202ced0d37/css/css-grid/grid-definition/fit-content-track-sizing.html#L87-L88).
- 고정 source SHA-256은 `828160951bba50f6ce1c09a69b804480df10d5e622c92a047452f432cb790736`다. 2026-10-27 Agent Browser 관찰에서 live `wpt.live` source hash가 pinned checkout과 일치했다. [full-suite screenshot/README](../c11-4-fit-content-auto-neighbor-runtime-precomparison-2026-10-27/README.md)와 같은 evidence directory의 `source-pins.json`에 raw hash·관찰값이 있다.
- 2026-10-28 Agent Browser로 upstream 페이지를 다시 열어 Harness status `OK`, `Found 13 tests`, `13 Pass`, min-height assertion Pass를 확인했다. 이 실행은 raw log를 별도 파일로 저장하지 않았으므로 이전 hash 관찰의 재검증이라고 주장하지 않는다.

## 규범 기대값

CSS Grid 2025 CRD [§11.2](https://www.w3.org/TR/2025/CRD-css-grid-1-20250326/#algo-terms)는 `fit-content()` track의 max sizing function을 max-content로 취급한다. [§11.8](https://www.w3.org/TR/2025/CRD-css-grid-1-20250326/#algo-stretch)는 free space가 indefinite일 때 definite grid `min-height`를 사용해 stretch 단계의 남은 공간을 계산하지만, 실제 확장 대상은 `auto` max track sizing function인 track뿐이다. 따라서 `min-height:300px`은 사용된 root height와 stretch 단계 free space에는 영향을 주지만 fit-content row를 늘리지 않는다.

고정 WPT 입력에서 child의 block-size 및 minimum contribution은 10px이다. `fit-content(40px)` row의 base와 growth limit은 모두 10px이다. definite `min-height` 기준 stretch 단계의 양수 여유 공간은 `300 - 10 = 290px`이지만 auto max track이 없으므로 `align-content:normal` 단계 실행 전후 row는 10px이다. Root used height는 300px이다. 기대 frame은 root `[0,0,60,300]`, item `[0,0,60,10]`, child `[0,0,60,10]`이다. `expected.json`은 이 산술과 허용값을 기계 판독형으로 고정한다.

이 자료에는 Spinon layout 또는 app 실행 결과가 없다. Chrome upstream pass는 Spinon WPT pass가 아니다.

## 문서 빌드 경계

2026-10-28 `mise exec -- bun run docs:build`는 RSPress 2.0.22에서 exit code 1로 끝났다. 첫 실행에서 이 README의 raw `.json` markdown link도 dead link로 보고되어 plain-text artifact path로 바꿨다. 재실행에는 976개의 dead-link 문서 보고가 남았지만 이 evidence README와 새 계획 파일은 error list에 없었다. 다른 파일은 이 계획 PR에서 수정하지 않는다.
