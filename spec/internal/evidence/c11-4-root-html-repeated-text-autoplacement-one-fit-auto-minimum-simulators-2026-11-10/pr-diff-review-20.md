# C11.4 one-fit auto-placement 구현 PR diff 검토 · 20개 관점

PR #1322의 실제 원격 diff를 검토했다. 원격 head는 `5128b65b1886a971f57933ac6bd30a30c11f9ba0`, 변경 파일은 35개이며, 파일 목록을 로컬 commit diff와 대조했다. 기존 구현/runtime 검토와 분리해 코드 변경, 대조 행렬, 오류 경계, 문서 및 제출 근거를 살폈다.

| # | PR diff 실패 관점 | 판정과 근거 |
|---:|---|---|
| 1 | 한-fit 자동 최소 크기 변경이 임의의 six-member Grid에 새어 나가는가? | **통과.** Layout의 새 허용식은 기존 auto-placement track-list classifier와 exact repeated-text profile을 함께 요구한다. |
| 2 | Layout track-list classifier가 0개 또는 여러 fit track까지 허용하는가? | **통과.** 기존 classifier는 여섯 track, fit-content 하나, `min-content ×2`, `max-content ×2`, `auto ×1`만 허용한다. 별도 rejection test가 zero/three fit과 잘못된 multiplicity를 확인한다. |
| 3 | Style projection과 Layout 중 하나만 더 넓은 입력을 허용하는가? | **통과.** 두 계층에 독립 gate가 있고 양쪽 모두 repeated-text automatic-minimum profile 및 여덟-item 조건을 확인한다. RuntimeSession positive matrix는 양쪽 경로를 거친다. |
| 4 | explicit-row control이 함께 `min-height:auto`가 되는가? | **통과.** 두 계층은 첫 control의 computed/layout minimum `0`을 요구한다. control을 `auto`로 바꾸는 rejection test가 있다. |
| 5 | 여덟 text item 중 일부만 자동 minimum이어도 허용되는가? | **통과.** 모든 item이 `auto`여야 하며 mixed `auto`/`0`와 고정 nonzero minimum rejection이 있다. |
| 6 | scrollable/hidden overflow에서도 cap 아래 automatic minimum이 적용되는가? | **통과.** 자동 minimum 경로가 X/Y visible overflow를 요구한다. item `auto`, `scroll`, `hidden` 및 Grid `overflow:auto` 변형을 거부한다. |
| 7 | 0px/0% 또는 px/% cap의 authored type·순서가 소실되는가? | **통과.** Candidate matrix는 authored/computed typed track order를 비교한다. 0%, 20%, 120%, 20px 및 unlisted 7px/13% control을 포함한다. |
| 8 | percentage cap의 basis가 바뀌거나 indefinite 상태로 처리되는가? | **통과.** 허용 Grid는 definite height 220px다. Height를 `auto`로 바꾸는 negative mutation이 있다. |
| 9 | 9-child topology가 달라지거나 text placement가 explicit로 바뀌는가? | **통과.** Missing/extra child, nested payload, explicit placement mutation이 거부된다. Grid geometry·row flow·LTR·align-content도 고정한다. |
| 10 | Chrome 관측 장면이 줄거나 중복 key가 비교를 통과하는가? | **통과.** Candidate와 baseline은 각각 1,440건이며 dimensions, unique key와 key-set equality를 확인한다. |
| 11 | 기존 `min-height:0` baseline과 서로 다른 authored 장면을 비교하는가? | **통과.** key는 fit 위치, tail 순서, cap, alignment를 포함하며 baseline map과 candidate set의 완전 일치를 요구한다. |
| 12 | computed style이 Chrome 기대값과 다른데 frame만 비교하는가? | **통과.** Track-list typed value·serialized `grid-auto-rows`, alignment, 각 item의 computed `min-height`를 함께 비교한다. |
| 13 | item 또는 payload의 frame 차이가 전체 scene에서 숨는가? | **통과.** 9개 item frame과 8개 payload frame을 모든 장면에서 개별 비교한다. |
| 14 | glyph 수나 wrapping 줄 수가 바뀌어 floor 기대값이 틀리는가? | **통과.** Payload별 glyph count와 distinct baseline count를 Chrome 기대와 비교하고 고정 font/text profile은 별도 validation한다. |
| 15 | 작은 cap에서 floor 회복이 누락되거나 content가 Grid 밖으로 넘치는가? | **통과.** 1,920 fit-row occurrence를 검사하고 baseline 아래였던 600개 회복과 overflow 0을 aggregate assertion한다. |
| 16 | `align-content:stretch`가 intrinsic/fit-content row까지 늘리는가? | **통과.** 720 track-list pair 각각의 row 합을 180px/220px로 확인하고 auto row 외 delta 0, auto row 증가 합 40px를 검사한다. |
| 17 | zero-cap trace가 contribution source와 최종 row base를 잘못 연결하는가? | **통과.** `AutomaticMinimum` content-based contribution, 단일 auto-min track span, item floor와 같은 final track base를 확인한다. |
| 18 | 잘못된 media revision이 기존 frame을 partial output으로 게시하거나 세션을 오염하는가? | **통과.** Dark three-fit 입력 뒤 completed output이 없어야 하고, 같은 session의 valid light revision이 새 key와 전체 frame을 복구해야 한다. |
| 19 | 기존 two-fit auto-placement/row-span admission이 사라지거나 one-fit 거부가 오래된 suite에 남는가? | **통과.** 1-fit 거부 사례를 one-fit positive/rejection suite로 옮겼고 2-fit auto-placement 및 row-span 회귀를 별도 matrix로 실행했다. |
| 20 | 증거·공식 상태·PR 본문이 지원 범위를 과장하거나 화면 링크가 깨지는가? | **통과.** iOS/Android light/dark 4개 screenshot은 PR 본문에서 commit-pinned raw URL로 로드됐고 browser DOM에서 모두 `complete`, nonzero dimensions였다. Android는 llvmpipe software로 표기한다. Status와 0059는 candidate/unmerged, `78/214`, contract `0.1.0`, C11.4/C11 미완료를 유지한다. Agent Browser 세션을 확인 뒤 종료했다. |

## 판정

서로 다른 PR diff 실패 관점 20개에서 수정이 필요한 결함을 찾지 못했다. 이 검토는 고정 candidate profile의 증거이며, GitHub check run이나 independent approving review가 있었다는 뜻이 아니다. Public roadmap과 Tailnet preview는 PR 병합 뒤 별도 동기화한다.
