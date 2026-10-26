# C11.4 image auto/max mixed-track plan review — pass05 · fresh 20관점

**검토 기준:** `/private/tmp/spinon-c11-4-image-auto-max-preplan-20261026`, branch `plan/c11-4-image-auto-max-mixed-track-20261026`. Checkout `HEAD`와 local `origin/main`은 `a33b245983fae050b66efb9ff259b3fe1132a051`이다. 읽기 전용 `git ls-remote`와 GitHub compare 결과로 현재 remote `main`은 `040a5360be92f73e141797aa7dcd4ef00ab24942`까지 진행했고, 차이는 merged PR #642를 포함한 3 commits임을 확인했다. `gh pr list --head`에는 image plan PR이 없고 authored plan/fixture/evidence는 untracked이며 parent plan은 modified다. 최신 candidate plan, fixture, evidence, parent entry, pass04와 기존 plan PR diff-30 report를 검토했다. 이 보고서 자체는 대상에서 제외했다. 테스트·브라우저·시뮬레이터는 실행하지 않았다.

## 판정

**PASS 14 / PARTIAL 4 / FAIL 2.** 이미지 sizing 및 overflow 경계는 이번 payload에서 명확히 제한되어 있다. 다만 현재 branch base가 remote `main`보다 3 commits 뒤이고 parent plan의 현재 내용이 child plan과 다르므로 계획 PR 제출/병합 관문은 **BLOCKED**다. evidence README의 영어 설명 문단도 문서 언어 규칙에 맞춰야 한다. 기존 pass04의 bounded sizing implementation-start 판정은 기능 계약에 대한 판정이며, 이 pass05에서 발견한 base·parent 동기화 FAIL을 해소하지 않는다.

## Fresh failure perspectives 20개

| # | 별도 실패 관점 | 판정 | 확인 근거 |
|---:|---|:---:|---|
| 1 | 검토 대상이 이미 원격 PR payload인가 | PARTIAL | `gh pr list --head plan/c11-4-image-auto-max-mixed-track-20261026`은 결과가 없다. child plan, fixture, evidence는 untracked이고 parent plan은 modified여서 실제 PR diff/check 상태는 입증되지 않는다. |
| 2 | 계획 branch가 현재 remote base에 올라 있는가 | FAIL — blocker | checkout `HEAD`/local `origin/main`은 `a33b2459`지만 `git ls-remote origin refs/heads/main`은 `040a5360`이다. GitHub compare는 `a33b2459...040a5360`이 3 commits ahead라고 보고하고, merged PR #642가 그 변경에 포함된다. PR을 만들기 전에 최신 main으로 branch와 parent 변경을 재정렬해야 한다. |
| 3 | parent 계획의 최신 entry가 child의 scope와 review 상태를 반영하는가 | FAIL — blocker | local [`c11-4-grid-track-sizing.md`](./c11-4-grid-track-sizing.md) entry는 pass03의 `20 PASS / 0 PARTIAL`을 현재 유효 판정처럼 쓰고 `단일 축 overflow 거부 control`이 포함된다고 적는다. child plan은 pass04 `18 PASS / 2 PARTIAL`을 최신으로 기록하고 authored overflow 및 overflow acceptance/controls를 제거했다고 명시한다. entry는 pass04/pass05도 링크하지 않는다. |
| 4 | positive HTML fixture가 author overflow를 숨겨 다시 넣는가 | PASS | HTML image rule과 `authored.imageRule`에는 `overflow` 선언이 없다. 계획도 positive V8 scene에서 author하지 않는다고 고정한다. |
| 5 | Chrome `<img>` `clip/clip`이 authored input 또는 Spinon 결과로 오인되는가 | PASS | observation JSON의 root는 `visible/visible`, `<img>`는 computed `clip/clip`이다. plan/evidence는 후자를 Chrome UA CSSOM observation 및 §6.6 applicability context로만 사용한다. |
| 6 | app computed overflow나 projection 값이 acceptance에 숨어 있는가 | PASS | child plan과 expected JSON은 app computed overflow, extraction/projection, clipping, scrolling, paint를 scope 밖에 둔다. |
| 7 | 제거한 x-only/y-only overflow rejection controls가 예상 error로 남아 있는가 | PASS | fail-closed matrix에는 기존 grid/style/topology/source/decode/revision/profile controls만 있다. `Clip/Visible`, `Visible/Clip`, `Some(Visible)` 또는 overflow-specific adapter error acceptance는 없다. |
| 8 | direct image slice가 text, multiple image, span 또는 general CSS로 넓어지는가 | PASS | plan은 fixed PNG 1개, direct item 1개, auto placement span 1, columns `minmax(auto,max-content) 17px`로 제한하고 일반 image loader 및 CSS admission을 제외한다. |
| 9 | fixture 상대 `src`가 다른 PNG 또는 잘못된 경로를 선택하는가 | PASS | `tests/fixtures/css/c11/...html`의 `../../resources/c14-1-png-basic.png`는 `tests/fixtures/resources/c14-1-png-basic.png`를 가리킨다. 파일 hash/크기도 expectation과 일치한다. |
| 10 | body default margin이나 root box model이 frame을 이동/변경하는가 | PASS | HTML은 `html, body { margin: 0 }`, root와 image의 margin/padding/border 0, `border-box`를 선언한다. Chrome root frame은 `(0,0,100,40)`이다. |
| 11 | root grid declarations 순서가 의도한 gap 또는 track 값을 덮는가 | PASS | `gap:0` 다음 `column-gap:5px`, `row-gap:0`을 둔다. raw CSSOM은 `5px/0px`, tracks `24px 17px`와 row `16px`를 기록한다. |
| 12 | screenshot의 outline이 sizing box/frame expectation에 섞이는가 | PASS | image의 1px outline은 paint-only fixture 표시다. 계획은 border/pixel/overflow paint를 acceptance에서 제외하고 image frame을 CSS box로만 고정한다. |
| 13 | 문서 전체 `scrollWidth/scrollHeight`를 grid/image overflow로 확대하는가 | PASS | observation의 2301×1356 값은 document element 값이다. 계획은 이를 image/root overflow 증거로 사용하지 않는다고 명시하고 overflow behavior를 제외한다. |
| 14 | 이미지가 decode되기 전에 natural size/CSSOM을 캡처하는가 | PASS | fixture는 `image.decode()` 완료 뒤 capture를 노출한다. observation은 `complete=true`, natural size `24×16`이다. |
| 15 | CSS Images source hash가 raw response 없이 독립 재검산 가능한가 | PARTIAL | README/JSON은 dated 2023-12-18 CRD URL, retrieval date, 353703 bytes와 SHA-256을 기록하지만 해당 raw response bytes 파일이 evidence directory에 없다. 현재 hash preimage를 로컬에서 재계산할 수 없다. |
| 16 | independent sizing arithmetic가 image, gutter, free space와 row를 함께 만족하는가 | PASS | expectation JSON은 columns `[24,17]`, gutter `5`, remaining inline space `54`, row `16`, root/image frame을 따로 고정한다. Chrome CSSOM은 별도 oracle이고 규범 계산은 Taffy output에서 역산하지 않았다고 기록한다. |
| 17 | trace `null`이 내부 positive infinity나 runtime 결과로 과장되는가 | PASS | plan/evidence/JSON은 모든 non-finite 값의 lossy `None/null` projection을 설명하고 내부 `+∞` 증명과 구분한다. Runtime expectation은 아직 실행하지 않은 acceptance prediction으로 표시한다. |
| 18 | evidence 문장이 저장소 한글 규칙을 지키는가 | PARTIAL | evidence README의 마지막 runtime-scope 문단은 `Overflow is outside this slice's runtime contract...`로 시작하는 영어 설명 문단이다. `docs/project-rules.md`는 문서를 한글로 작성하도록 요구한다. 내용을 한글로 바꾸면 scope 주장은 유지할 수 있다. |
| 19 | capture artifact끼리 다른 fixture/commit을 가리키는가 | PASS | 재계산 SHA-256은 HTML `e3b6edfe…06c92c2b`, PNG asset `5d8c9509…bce4a2`, observation `82c020de…ce635f4`, metadata `5043ffde…8f1bb90`, screenshot `4182f752…6625eb`이며 README/metadata/expectations의 연결값과 일치한다. Capture source `a33b2459`는 최신 branch base 주장과 구분한다. |
| 20 | browser cleanup claim을 session summary보다 강한 증거로 표현하는가 | PARTIAL | metadata에는 session `active:false,pid:null`, 빈 active-session list 및 dedicated Chrome/crashpad match `0`이 있다. 원문 CLI/process inventory transcript는 evidence directory에 없어 사후 독립 audit은 제한된다. |

## Blocker와 다음 관문

- **Blocker 1 — stale base:** branch를 remote main `040a5360be92f73e141797aa7dcd4ef00ab24942`에 맞춘 뒤 parent 변경을 재적용해야 한다. 현재 local `origin/main` ref 또한 remote보다 오래됐다.
- **Blocker 2 — parent entry:** `plan/c11-4-grid-track-sizing.md`의 image entry에서 overflow rejection-control claim을 제거하고 pass04/pass05의 현재 결과·scope와 링크를 반영해야 한다. 이번 검토에서는 authored plan을 수정하지 않았다.
- **비차단 provenance/documentation:** evidence README 영어 문단은 계획 PR 전 한글로 고친다. CSS Images raw bytes와 browser cleanup 원문 log 부재는 근거 재현성의 PARTIAL로 남는다.

기능 계산 관점에서는 image-only sizing contract에 새 blocker를 찾지 않았다. 그러나 현재 candidate를 plan PR ready 또는 merge-ready라고 판정하지 않는다. 최신 main/base, parent entry, 문서 언어와 실제 PR diff를 정리한 다음 plan PR diff-30 결과를 다시 대조해야 한다. 이 보고서는 plan/fixture/evidence/product source를 수정하지 않았고 테스트·브라우저·시뮬레이터를 실행하지 않았다.
