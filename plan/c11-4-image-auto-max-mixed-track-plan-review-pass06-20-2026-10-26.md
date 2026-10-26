# C11.4 image auto/max mixed-track plan review — pass06 · fresh 20관점

**검토 기준:** `/private/tmp/spinon-c11-4-image-auto-max-preplan-20261026`, branch `plan/c11-4-image-auto-max-mixed-track-20261026`. Checkout `HEAD`, local `origin/main`, remote `main`은 모두 `040a5360be92f73e141797aa7dcd4ef00ab24942`다. 최신 child plan, 수정된 parent entry, fixture/evidence, pass05 follow-up 및 기존 plan PR diff-30 보고서(addendum 포함)를 읽기 전용으로 대조했다. PR은 아직 열리지 않았고 authored plan/fixture/evidence는 worktree candidate다. 이 보고서 자체는 대상에서 제외했다. 테스트·브라우저·시뮬레이터·커밋은 실행하지 않았다.

## 판정

**PASS 17 / PARTIAL 3 / FAIL 0 (문구 재확인 반영).** 이전 두 overflow blocker는 final scope에서 해소됐고 parent/base 불일치도 수정됐다. 남은 partial은 (1) 실제 원격 PR diff가 아직 없음, (2) CSS Images raw response preimage 부재, (3) Agent Browser cleanup 원문 log 부재다. Sizing 계산, overflow claim boundary, status/link, local evidence hash chain, README 문서 언어에는 FAIL이 없다. README #17 문구 재확인은 [후속 기록](./c11-4-image-auto-max-mixed-track-plan-review-pass06-follow-up-2026-10-26.md)에 남겼다.

## Fresh failure perspectives 20개

| # | 독립 실패 관점 | 판정 | 확인 근거 |
|---:|---|:---:|---|
| 1 | plan branch가 현재 기준 main을 포함하는가 | PASS | checkout `HEAD`와 local `origin/main`은 `040a5360`; `git ls-remote`의 remote `main`도 같은 SHA다. Child plan은 이를 branch base로, `a33b2459`는 capture source로 구분한다. |
| 2 | 현재 검토 대상이 이미 원격 PR diff/CI payload인가 | PARTIAL | `gh pr list --head plan/c11-4-image-auto-max-mixed-track-20261026`은 비어 있고 authored plan/fixture/evidence는 local untracked 상태다. 이 리뷰는 현재 후보 문서만 평가한다. |
| 3 | parent entry가 final child scope와 결과 이력을 반대로 적는가 | PASS | parent entry는 pass04 `18/2`, pass05 pre-fix `14/4/2`, pass05 follow-up을 연결한다. overflow rejection control을 제거하고 Chrome `<img>` `clip/clip`을 CSSOM-only로 설명하며 fresh recheck pending이라고 제한한다. |
| 4 | child/evidence/review 간 상대 링크가 끊기는가 | PASS | child plan과 evidence README의 local Markdown target은 모두 존재한다. Parent entry의 child, evidence, pass04, pass05, pass05 follow-up target도 모두 존재한다. |
| 5 | C11.4 또는 공식 완료 수가 candidate plan 때문에 승격되는가 | PASS | parent plan은 C11.4 Stage A/B와 상위 checkbox 미완료, `72/213`을 유지하고 `spec/STATUS.md` C11.4 항목도 unchecked다. |
| 6 | capture source와 current implementation base를 한 revision으로 혼동하는가 | PASS | plan/JSON/metadata는 Chrome capture를 `a33b2459`에 묶고 branch base는 `040a5360`로 기록한다. |
| 7 | final positive fixture에 author overflow가 다시 삽입되는가 | PASS | HTML image CSS rule에는 overflow 선언이 없다. Plan은 positive HTML/V8 scene도 overflow를 author하지 않는다고 고정한다. |
| 8 | Chrome `clip/clip`을 authored input, Spinon computed value 또는 projection output으로 취급하는가 | PASS | observation은 root `visible/visible`, `<img>` `clip/clip`을 기록한다. Plan/evidence는 image 값을 Chrome UA CSSOM observation 및 §6.6 applicability context로만 쓴다. |
| 9 | app overflow clipping, scrolling, paint 또는 projection이 implicit acceptance인가 | PASS | Child plan과 JSON은 app computed overflow 및 projection/clip/scroll/paint를 sizing scope 밖으로 명시한다. |
| 10 | 첫 prior FAIL의 authored `overflow:visible` positive scene이 남아 있는가 | PASS | Final plan/fixture에는 해당 declaration 및 `Some(Visible)` app expectation이 없다. 기존 FAIL #27은 해소된 상태다. |
| 11 | 두 번째 prior FAIL의 x-only/y-only overflow controls가 잘못된 adapter error를 약속하는가 | PASS | Final negative matrix에는 `Clip/Visible`, `Visible/Clip` mutation이나 overflow-specific error contract가 없다. 기존 FAIL #28은 해소된 상태다. |
| 12 | 남은 negative matrix의 boundary가 기존 image adapter gates와 맞지 않는가 | PASS | controls는 grid/style, geometry, placement, topology/source, decode, dimension, revision/profile gate만 열거하고 해당 단계 전 snapshot/frame/trace atomicity를 요구한다. |
| 13 | fixture가 다른 image, extra item 또는 explicit placement를 사용해 span-1 계산을 바꾸는가 | PASS | Fixed PNG path와 root/item ID가 고정돼 있고 direct child image 하나가 첫 column/row에 자동 배치된다. CSSOM placement는 모두 `auto`다. |
| 14 | natural-size 기대값 및 fixed/intrinsic mixed arithmetic가 evidence와 어긋나는가 | PASS | PNG는 97 bytes, `24×16`; expected columns `[24,17]`, gutter `5`, row `16`, trailing free space `54`, root/image frames가 독립 산술과 일치한다. |
| 15 | trace `null`을 내부 `+∞`의 증명 또는 runtime 값으로 확대하는가 | PASS | Plan/JSON은 모든 non-finite 값의 `None/null` projection을 설명한다. `null`은 projection만 확인하고 finite stages는 `Some(finite)`를 기대한다. |
| 16 | plan/evidence가 custom Spinon scene이 이미 실행됐다고 주장하는가 | PASS | Plan은 runtime/simulator를 후행 gate로 두고 evidence README는 현재 수치를 Chrome oracle로 한정한다. 기존 공통 builder baseline test의 기록도 custom six-scene Spinon 실행이 아니라고 구별한다. |
| 17 | README가 저장소의 한글 문서 규칙을 완전히 지키는가 | PASS | README line 10은 `재캡처 기준 커밋은`이라고 기록한다. `docs/project-rules.md`의 문서 언어 기준을 충족한다. 재확인 결과와 현재 README SHA-256은 [후속 기록](./c11-4-image-auto-max-mixed-track-plan-review-pass06-follow-up-2026-10-26.md)에 고정했다. |
| 18 | HTML/PNG/observation/metadata/screenshot hash 참조가 다른 payload를 가리키는가 | PASS | 재계산 SHA-256과 expectation/metadata 연결값이 fixture `e3b6edfe…06c92c2b`, asset `5d8c9509…bce4a2`, observation `82c020de…ce635f4`, metadata `5043ffde…8f1bb90`, screenshot `4182f752…36625eb`에서 일치한다. Capture screenshot은 320×80 PNG다. |
| 19 | CSS Images raw response hash를 보존된 preimage에서 재현할 수 있는가 | PARTIAL | JSON/README는 dated CRD, retrieval date, 353703 bytes와 SHA-256을 기록하지만 raw response 자체가 evidence directory에 없어 preimage를 재검산할 수 없다. |
| 20 | Agent Browser 종료 상태가 원문 audit transcript로 재현되는가 | PARTIAL | Metadata는 `active:false,pid:null`, 빈 active-session list, dedicated Chrome/crashpad process match `0`을 적지만 CLI/process inventory 원문 log는 없다. |

## Blockers와 후속 처리

- 기능/overflow 계약에 **FAIL blocker는 없다.** 기존 #27/#28은 author overflow와 adapter error acceptance를 제거해 해소됐다.
- README 문서 언어 PARTIAL은 targeted follow-up에서 해소됐다. 실제 원격 PR이 열리면 final head/base/diff를 다시 확인하고, CSS Images raw response 및 browser cleanup log provenance partial은 별도 범위로 남겨 둔다.
- 실제 PR이 아직 없으므로 현재 30관점 follow-up은 local candidate 확인이다. PR을 열면 head/base 및 diff identity를 다시 고정해야 한다. CSS Images raw bytes와 browser cleanup raw log PARTIAL은 별도 provenance 한계로 남는다.
- 이 fresh pass06 report는 child plan과 parent entry에 아직 링크되어 있지 않다. 이번 요청은 review-only라 authored plan을 수정하지 않았다. 계획 PR payload를 정리할 때 review 기록 링크와 pass06 판정을 반영해야 한다.

이 검토는 독립 계획·evidence readback과 local hash 계산에 한정된다. product/runtime 결과, tests, browser, simulator 또는 remote PR CI를 새로 실행하지 않았다.

## README 문구 재확인 · 2026-10-26

작성자가 README line 10을 `재캡처 기준 커밋은`으로 수정했다. 기대한 SHA-256 `53f36529c9838ca34bf42591b58a3bb507bdf027c05c1bacdd253f0ee63a9725`와 일치하고 문서 언어 규칙을 충족하므로 row 17은 PASS로 바꿨다. 나머지 19관점은 재검토하지 않았다. 자세한 범위·해시는 [후속 기록](./c11-4-image-auto-max-mixed-track-plan-review-pass06-follow-up-2026-10-26.md)에 있다.
