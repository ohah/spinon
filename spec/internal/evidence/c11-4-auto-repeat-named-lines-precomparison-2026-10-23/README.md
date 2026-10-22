# C11.4 auto-repeat named line 사전 비교

## 범위와 기준

반복 fragment 안의 line name이 `auto-fill`/`auto-fit` 확장 뒤에도 named placement와 같은 물리 line을 가리키는지 고정 입력으로 관찰한다. 이 결과는 Chrome 동작만 기록하며 현재 Spinon의 통과, CSS 적합성 또는 제품 지원을 뜻하지 않는다.

- 규범 기준: [CSS Grid Layout Module Level 1, 2025 CRD §7.2.3 auto-repeat](https://www.w3.org/TR/2025/CRD-css-grid-1-20250326/#auto-repeat) 및 [§7.3 line-based placement](https://www.w3.org/TR/2025/CRD-css-grid-1-20250326/#line-placement) — repeat line names와 occurrence 기반 배치.
- 기준 브라우저: Agent Browser가 연 Google Chrome `156.0.8078.48`, macOS `26.5.1`, ARM64, DPR 1. User-Agent와 viewport는 [`chrome156-observation.json`](./chrome156-observation.json)에 보존했다.
- 고정 입력: [`grid-auto-repeat-named-lines.html`](./grid-auto-repeat-named-lines.html). SHA-256은 아래 목록에 기록한다.
- 화면: [Chrome 관찰 screenshot](./chrome156-auto-repeat-named-lines.png). CSS `getComputedStyle()`과 root-relative `getBoundingClientRect()`는 저장된 JSON에서 확인할 수 있다.
- browser workflow: 공식 spec과 로컬 고정 fixture는 Agent Browser로 확인했다. `terminal-browser`는 사용하지 않았다. 관찰 뒤 Agent Browser session을 닫고 세션/프로세스가 없는 것을 확인했다.

## 독립 계산과 관측값

`auto-fill`은 definite grid size 안에 들어가는 최대 반복 횟수를 사용한다. `40px` 단일 track과 `10px` gutter는 `180px`에 세 개가 들어가며 네 개는 `190px`를 요구하므로 count는 3이다. item의 `cell 2 / cell 3`은 두 번째 track, `cell -2 / cell -1`은 마지막 track을 선택한다. `span 2 cell`은 첫 line에서 두 track과 그 사이 `10px` gutter까지 덮어 `90px` item이 된다.

같은 `180px`에서 `auto-fit`은 먼저 3개 explicit repeated track을 만든 뒤, 두 번째 track만 점유한 fixture에서 나머지 두 빈 track을 접는다. 이때 line occurrence는 접힌 후 새로 번호를 매기지 않는다. Chrome computed value는 `0px 40px 0px`이고 item은 root 시작점에 놓인다.

| scene | 독립 기대 / Chrome 관측 (root-relative) | 무엇을 확인하는가 |
| --- | --- | --- |
| `positive` | `40px × 3`; item `(50,0) 40×40px` | positive occurrence `2 cell / 3 cell` |
| `bare` | `40px × 3`; item `(0,0) 40×40px` | omitted occurrence selects the first named line |
| `negative` | `40px × 3`; item `(100,0) 40×40px` | negative occurrence `-2 cell / -1 cell` |
| `span` | `40px × 3`; item `(0,0) 90×40px` | `span 2 cell`과 gutter 포함 |
| `fit` | collapsed tracks `0,40,0px`; item `(0,0) 40×40px` | auto-fit collapse 후 line index 보존 |
| `neighbors` | prefix/suffix `20/10px`, 2-track pattern 2회, item `(120,0) 80×40px` | 반복 pattern, fixed neighbor와 line occurrence |
| `merge` | prefix/suffix `10/10px`, repeated track 3회, item `(20,0) 80×40px` | repeat 시작/끝과 바깥 line-name 그룹의 물리 line 병합 |
| `row` | row `40px` 3회; item y=`50px`, height `40px` | row axis named placement |

`neighbors`의 반복 크기는 매 회 `40 + 10 gutter + 30 = 80px`이다. fixed prefix/suffix와 모든 외부 gutter까지 더하면 count 2의 사용량은 `220px`, count 3은 `310px`이므로 `260px`에서 두 회다. 두 번째 `loop`부터 두 번째 `loop-end`까지 item은 40px track, 10px 내부 gutter, 30px track을 덮는다.

`merge`는 prefix/suffix 각 10px, repeated track 20px, gutter 10px에서 count 3일 때 `120px`을 채운다. `same 1 / same 4`는 반복 첫 line에서 시작해 세 번째 track의 끝 line에서 끝나므로 item은 `(20,0) 80×40px`이다. 같은 이름이 repeat 끝과 다음 반복 시작, 또는 바깥 component 경계에 중복 기재되어도 occurrence는 물리 line 순서로 계산한다. Chrome computed string에 한 line의 동일 이름이 두 번 표시되는 경우도 원본에 남겼다.

## 해석 한계

- 이 비교는 한 Chrome build의 CSS layout 관찰이다. 다른 브라우저, WPT, Spinon layout/runtime, Android/iOS 화면은 포함하지 않는다.
- 계획 acceptance는 이 8개 fixed input으로 한정한다. line-name grammar 전체, named areas와 auto-repeat 결합, implicit line fallback, multiple auto-repeat components per axis, 일반 CSS admission을 주장하지 않는다.
- 기존 auto-repeat track sizing subset, 반복 수 상한과 fail-closed 정책은 별도로 유지한다.

## 입력 및 관측 hash

- HTML fixture: `2df81bb12217c18d80f96d9791723c9de73818e9b290e6169c661a18b440b92e`
- Chrome observation JSON: `0672eb661b5f454601daaa0ed641245218be24afde306bd5c901197c9cd352d2`
- Chrome screenshot (`1280×633` PNG): `d38199c8154c30fd760127d6b65925bd1a385e6ac3c4971f909afbfacc4bfa73`
