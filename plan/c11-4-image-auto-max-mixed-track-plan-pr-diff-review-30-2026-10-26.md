# C11.4 image auto/max mixed-track 계획 PR payload 검토 — 30 관점

**초기 검토 기준:** `/private/tmp/spinon-c11-4-image-auto-max-preplan-20261026`, branch `plan/c11-4-image-auto-max-mixed-track-20261026`, 당시 HEAD와 local `origin/main`은 `a33b245983fae050b66efb9ff259b3fe1132a051`이었다. 계획·증거·fixture와 관련 source 경계를 읽기 전용으로 대조했다. 제품 코드와 authored 문서는 수정하지 않았고 테스트, 브라우저, 시뮬레이터도 실행하지 않았다. PNG screenshot은 초기 검토에서 로컬 증거 파일로 열어 확인했다. 최신 remote `main`은 후속 addendum에서 별도로 확인한다.

## 판정

**초기 결과 PASS 25 / PARTIAL 3 / FAIL 2. 최종 후보 payload의 기존 30관점 유효 결과 PASS 27 / PARTIAL 3 / FAIL 0.** Final candidate에서 overflow authored positive와 overflow negative/error acceptance가 제거되어 초기 FAIL #27/#28은 계속 해소 상태다. Parent plan scope/status mismatch #9도 수정되어 PASS로 돌아갔다. 원격 PR이 없어 #2는 PARTIAL로 남고 CSS Images raw response 부재 #15 및 browser cleanup 원문 로그 부재 #26도 유지된다. 초기 capture에 근거한 #23의 `visible/visible` 서술은 최종 recapture의 image `clip/clip`으로 supersede되며 아래 addendum에서 경계를 정리한다. 별도 pass06에서 README 언어 PARTIAL로 기록했던 line 10은 follow-up에서 번역을 확인해 해소했다. 이 발견은 기존 30관점 점수에 포함하지 않는다.

## 초기 수정 제안 (최종 후보에서 superseded)

다음 단락은 authored overflow acceptance가 남아 있던 초기 payload에 대한 제안이며 현재 판정이 아니다. 최종 후보는 app acceptance에서 overflow를 제외하고 UA computed CSSOM 관측만 보존해 두 blocker를 해소했다.

최소 안전 수정은 **app fixture를 computed initial `visible/visible`만 쓰는 default-visible case로 한정**하고 authored overflow declarations를 빼는 것이다. HTML Chrome oracle은 authored visible을 유지할 수 있지만, app/runtime 성공 기대를 초기값 추출로 명확히 분리해야 한다. 이 선택이면 adapter가 검사할 visible typed values를 계획의 성공 계약으로 남길 수 있다. Negative controls는 authored `overflow*` 선언의 rejection을 검증하려면 cascade `UnsupportedInlineAuthorProperty` boundary와 actual error contract로 바꾸고, image-adapter typed predicate의 x/y 동작은 직접 typed snapshot을 입력하는 focused adapter test로 따로 검증해야 한다. End-to-end `Clip/Visible` authored controls를 반드시 요구한다면 이는 현재 범위를 넘는 CSS profile/property admission 및 projection 설계 변경이므로 별도 계획 scope로 분리해야 한다.

## 초기 PR payload 실패 관점 30개

| # | 독립적으로 확인한 실패 모드 | 판정 | 근거와 다음 조치 |
|---:|---|:---:|---|
| 1 | 캡처 commit을 현재 코드 기준으로 오인하는가 | PASS | Checkout HEAD와 `origin/main`은 모두 `a33b2459`다. JSON의 `source_commit=b58de5df`는 capture 당시 baseline이라고 child plan L13에서 구분한다. |
| 2 | 검토 대상이 실제 원격 PR diff라고 잘못 단정하는가 | PARTIAL | `git status --short`에서 parent plan은 modified, child plan/evidence/HTML fixture는 untracked다. 이번 결과는 현재 worktree의 후보 payload 검토이며 pushed branch나 GitHub PR의 실제 diff·check 상태를 입증하지 않는다. 커밋/PR 후 payload identity를 확인해야 한다. |
| 3 | C11.4 공식 완료 상태를 이 계획 때문에 승격하는가 | PASS | Parent plan L3, L11과 `spec/STATUS.md` L273–274는 C11.4 미완료 및 공식 `72/213`을 유지한다. 계획 성공을 공식 완료로 오인하지 않게 기록했다. |
| 4 | pass01/02/03 수치를 하나의 최신 결과처럼 섞는가 | PASS | Child plan L5, L101은 pass01 `12/8`, pass02 `17/3`, pass03 최초 `19/1` 및 addendum 후 유효 `20/0`을 구분한다. |
| 5 | targeted addendum가 이전 19개 항목까지 재검토했다고 주장하는가 | PASS | Addendum L7은 pass03 #20만 재확인했고 다른 19개/JSON/hash는 재검토하지 않았다고 범위를 제한한다. |
| 6 | 상위 계획의 새 링크가 자식/evidence/review 산출물과 끊기는가 | PASS | Parent plan L9–11의 세부 계획, README, pass03, addendum 링크가 현재 파일에 있다. Child plan L118–121과 evidence README L7은 fixture/JSON 링크도 해석된다. |
| 7 | 이전 소스 상태와 현재 상위 상태가 혼재하는가 | PASS | Child plan L13은 `b58de5df`를 precomparison 시점으로 한정하고 `a33b2459`를 branch base로 쓴다. JSON L4의 baseline 값도 capture provenance로 유지한다. |
| 8 | 수정 payload가 제품 동작이나 일반 CSS 지원이 이미 완료됐다고 주장하는가 | PASS | Child plan L5, L74, L99–114와 evidence README L3, L44–54는 runtime/simulator 미실행, opt-in fixture scope, C11.4 미완료를 반복해 제한한다. |
| 9 | parent entry가 새 slice를 상위 C11.4 범위와 연결하지 못하는가 | PASS | Parent plan L9–11은 child plan 및 Chrome precomparison evidence를 연결하고 `Stage A/B`와 공식 상태가 그대로 미완료임을 명시한다. |
| 10 | 계획이 여러 이미지·일반 loading으로 넓게 읽히는가 | PASS | Child plan L9–11, L64–74, L114는 direct single PNG, 고정 source/ID, span 1, opt-in image adapter만 대상으로 삼고 live loading·다른 format·일반 CSS admission을 제외한다. |
| 11 | 저장소 HTML hash가 README와 달라졌는가 | PASS | HTML SHA-256 재계산 `2f5305ac…eaf8541`이 evidence README L8 및 observation/metadata JSON의 capture hash와 일치한다. |
| 12 | fixture DOM에 복수/중첩 item이나 explicit placement가 숨어 있는가 | PASS | HTML L47은 direct `img` 하나를 root 안에 두며 placement를 지정하지 않는다. Observation JSON L57–60은 네 grid placement computed 값이 `auto`임을 기록한다. |
| 13 | border/padding 또는 box sizing이 24px contribution을 바꾸는가 | PASS | HTML L13–29, L32–42는 root/image box model을 고정한다. Observation JSON L54–68은 item box model/margins/padding/border를 보존하고 root frame을 100×40으로 기록한다. |
| 14 | 실제 PNG가 고정 크기·바이트 주장과 다른가 | PASS | `file`은 asset을 24×16 RGBA PNG로 확인했다. 재계산 SHA-256 `5d8c9509…bce4a2`, 97 bytes는 evidence README L9와 JSON L8–14에 기재된 값에 맞는다. |
| 15 | CSS Images source pin을 다른 버전 또는 최신 mutable URL에 연결하는가 | PARTIAL | README L21/JSON L28–34는 dated 2023-12-18 CRD URL, publication/retrieval date, section, 응답 byte 수/hash를 고정한다. 다만 raw response bytes 자체가 evidence directory에 없으므로 기록된 hash preimage를 오프라인에서 재검증할 수 없다. URL에서 다시 받아 hash를 확인하거나 raw snapshot을 보존해야 한다. |
| 16 | CSS Grid/Box Sizing 근거의 문서 버전·적용 절을 빠뜨리는가 | PASS | Child plan L50과 expectations JSON L16–35는 Grid 2025 CRD §6.6/§7.2.1/§10.5/§11.3–11.6, Box Sizing dated WD, CSS Images dated CRD와 절을 따로 고정한다. |
| 17 | 독립 산술이 track/gutter/free-space 합과 맞지 않는가 | PASS | Expectations JSON L71–100은 `[24,17] + 5 = 46`, definite 100px에서 trailing free space 54px, row 16px, root/image frame을 계산한다. Child plan L22–24의 설명과 같은 값이다. |
| 18 | automatic minimum 숫자는 맞지만 source flags가 빠지는가 | PASS | JSON L102–111과 L135–145는 `AutomaticMinimum`의 span/content/replaced flags 및 세 fixed-limit `null`을 고정하고 다른 contribution의 source를 분리한다. |
| 19 | Taffy trace의 초기 unbounded 값을 `null`만으로 증명한다고 오해하는가 | PASS | Child plan L30, expectations JSON L113–120, evidence README L19는 모든 non-finite 값이 `None/null`로 손실 투영됨을 적는다. `null`은 projection만 검사하고 finite stage만 `Some(finite)`를 요구한다. |
| 20 | 설명 표와 full stage JSON이 서로 다른 track 상태를 가리키는가 | PASS | Child plan L34–39의 Columns `[0/null,5/5,17/17]`에서 `[24/24,5/5,17/17]`, Rows `0/null`에서 `16/16`은 expectations JSON L125–263의 ordered stages 및 gutter index와 일치한다. |
| 21 | Chrome 결과를 normative 계산이나 Spinon output으로 둔갑시키는가 | PASS | README L3, L15–19, L44–50과 child plan L50–56은 독립 규범 계산, Chrome CSSOM 관측, 아직 실행하지 않은 runtime acceptance를 각각 분리한다. |
| 22 | Browser capture 환경 식별이 빠지거나 viewport가 다르게 기록되는가 | PASS | Metadata JSON L11–43은 Chrome 156.0.8078.48, macOS 26.5.1 ARM64를 기록한다. Observation L9–14는 HeadlessChrome UA, DPR 1, 320×80 viewport를 보존하고 child plan L54와 맞는다. |
| 23 | authored/computed CSSOM이나 geometry 측정이 기대값과 어긋나는가 | PASS | Observation JSON L15–68, L71–87은 computed tracks `24px 17px`, row 16px, item `visible/visible`, natural frame과 root/image rect를 기록한다. Evidence README L27–38 및 expectations JSON L268–277가 이를 교차 연결한다. |
| 24 | screenshot hash/크기가 observation과 다른 파일을 가리키는가 | PASS | PNG SHA-256 재계산 `4182f752…36625eb`이 README L42 및 두 JSON capture provenance와 일치한다. `file`은 화면 artifact를 320×80 RGB PNG로 확인했다. 화면은 Chrome page capture로만 사용되며 앱 화면은 아니다. |
| 25 | page `scrollWidth/Height`를 image overflow 증거로 확대하는가 | PASS | Observation JSON L89–92의 2301×1356 값은 문서 전체 scroll extent다. Child plan/evidence는 이를 item/root overflow assertion으로 쓰지 않는다. |
| 26 | Agent Browser 종료를 첨부 자료만으로 독립 재현할 수 있는가 | PARTIAL | README L40–42와 child plan L56은 전용 session `active:false,pid:null`, profile/crashpad 종료를 서술하지만 session-info/process inventory 원문 로그는 evidence directory에 없다. 종료 주장은 명시돼 있으나 제3자가 재검산할 artifact는 부족하다. |
| 27 | Positive app case가 계획된 `overflow:visible` typed style까지 도달하는가 | FAIL — blocker | Child plan L67은 V8 scene의 inline item style에 `overflow:visible`을 넣는다. `image_scene_source`는 `scene.item_style`을 inline `style`에 합친다(`crates/spinon-ffi/src/runtime_gpu/fixtures/c11_4.rs:1329–1351`). C11.4 allowlist는 overflow/overflow-x/y를 포함하지 않는다(`crates/spinon-style/src/stylo_dom/cascade/runtime_grid.rs:126–152`); cascade는 typed snapshot 생성 전에 inline declarations를 거부한다(`runtime_grid.rs:299–305`; scanner `runtime_layout.rs:392–410`). 그래서 계획된 `Some(Visible)` 및 image adapter 성공 assertion은 실행될 수 없다. 최소 수정은 app scene에서 authored overflow를 제거하고 C11.4 profile의 computed initial `Some(Visible)`로 scope를 제한하는 것이다. |
| 28 | 축별 negative가 계획된 adapter error/code와 atomicity boundary를 검증하는가 | FAIL — blocker | Child plan L82–86/L97 및 JSON L293–313은 `Clip/Visible`, `Visible/Clip` mutation이 `validate_item_style`에서 `unsupported_c114_image_case`로 거부된다고 한다. 그러나 같은 inline-property preflight가 adapter 전에 이 두 author declarations를 `UnsupportedInlineAuthorProperty`로 거부한다(`runtime_grid.rs:299–305`, scanner `runtime_layout.rs:392–410`). CSS 입력 자체의 fail-closed는 관찰해도 adapter predicate/error/detail은 검증하지 못한다. Adapter를 단위 검증하려면 typed snapshot 직접 입력을 분리하고, end-to-end controls의 예상 boundary는 실제 cascade error로 바꿔야 한다. |
| 29 | 추출된 typed overflow를 projection이 그대로 전달한다고 과장하는가 | PASS | `has_overflow`는 `RuntimeBlockFlexGridTrackSizingV1`에서도 typed x/y를 채운다(`element.rs:49–56,118–119`). 하지만 projection의 `supports_overflow_clip`은 C11.4 profile을 포함하지 않아 `LayoutOverflow::Visible` 기본값을 쓴다(`style_values.rs:251–262,878–885`). 계획은 현재 일반 projection 지원을 확대하지 않는다고 L66/L72에서 제한한다. 다만 explicit author path가 막혀 있으므로 값 일치 assertion은 blocker #27 해결 전까지는 default-only 관측으로 좁혀야 한다. |
| 30 | Simulator/artifact 근거를 이미 존재하는 성과로 보고하는가 | PASS | Child plan L99–108, L112–114는 Android/iOS scene allowlist, app hash, frame/trace/draw-present, screenshot·PR image link 확인과 artifact boundary를 **구현 후 관문**으로 둔다. README L44–54는 runtime/Simulator 미실행을 명시한다. 실행 성공이나 제품 artifact 결과를 현재 claim하지 않는다. |

## 초기 차단 항목 (최종 후보에서 superseded)

- **Blocking FAIL #27:** `overflow:visible` authored inline style은 C11.4 author allowlist에 없다. Current plan scope로는 positive app scene이 cascade를 통과할 수 없다. Default-visible-only로 수정하면 일반 author-overflow admission을 넓히지 않고 구현 가능하다.
- **Blocking FAIL #28:** x/y negative author mutations도 같은 preflight에서 먼저 거부돼 adapter-level error contract를 실현하지 못한다. End-to-end rejection boundary를 cascade error로 바꾸고 adapter x/y predicate는 typed snapshot unit boundary에서 별도로 검증하거나, 별도 C11.4 profile admission 계획으로 분리해야 한다.
- **PARTIAL #2/#15/#26:** 아직 GitHub PR payload가 아니며, CSS Images raw response 및 Agent Browser close/process transcript가 보존되지 않았다. 이는 위 CSS allowlist blocker와 별도로 PR artifact provenance를 더 재현 가능하게 하는 개선점이다.

이번 리뷰는 읽기 전용이었다. 검증은 source/문서/hash readback과 로컬 screenshot 열람에 한정한다. 테스트, Agent Browser, simulator, 실제 PR 상태 확인은 하지 않았다.

## Initial candidate targeted recheck addendum · superseded

### 기존 FAIL 2개와 PARTIAL 3개

| 초기 관점 | 최종 후보 처분 | 근거 |
|---|---|---|
| #27 — positive `overflow:visible` author style이 C11.4 cascade preflight에서 거부됨 | **해소 → PASS** | 최종 HTML/V8 scene에는 authored overflow가 없다. Child plan/evidence는 Chrome `<img>` UA computed `clip/clip`만 CSSOM 관측으로 기록하고, app computed value·projection·clipping·scrolling·paint를 acceptance에서 제외한다. `Some(Visible)` app assertion도 없다. |
| #28 — 축별 overflow controls가 adapter가 아닌 cascade preflight에서 먼저 거부됨 | **해소 → PASS** | 최종 fail-closed matrix에서 `Clip/Visible`, `Visible/Clip` controls 및 overflow-specific adapter error 기대를 제거했다. Plan은 overflow author declaration/rejection controls를 범위에서 제외한다. |
| #2 — 실제 원격 PR diff가 아님 | **PARTIAL 유지** | 현재 `gh pr list --head plan/c11-4-image-auto-max-mixed-track-20261026`은 비어 있고 authored plan/fixture/evidence는 local untracked 상태다. 아직 GitHub PR diff/check를 검토할 대상이 없다. |
| #15 — CSS Images raw response preimage 미보존 | **PARTIAL 유지** | 날짜가 고정된 URL, retrieval date, `353703` bytes와 SHA-256은 기록됐지만 해당 raw response 파일이 evidence directory에 없어 오프라인 preimage 검증은 불가능하다. |
| #26 — Agent Browser cleanup의 raw audit log 미보존 | **PARTIAL 유지** | metadata의 `active:false,pid:null`, 빈 active list, dedicated Chrome/crashpad `0` summary는 남아 있지만 원문 CLI/process inventory transcript는 없다. |

### 추가 targeted sync 조정

- **초기 #9 PASS → 당시 PARTIAL.** 아래 final follow-up에서 parent entry 수정 후 PASS로 복원했다.
- **초기 #23 내용 superseded, 판정은 PASS 유지.** 옛 observation을 인용한 표에는 image `visible/visible`이라고 적혀 있으나 final recapture의 raw observation은 root `visible/visible`, `<img>` `clip/clip`이다. Final child plan/README는 `clip/clip`을 Chrome UA CSSOM으로만 한정하고 Spinon overflow acceptance를 주장하지 않는다.
- **당시 base 확인:** checkout과 local `origin/main`은 `a33b2459`지만 원격 main은 `040a5360be92f73e141797aa7dcd4ef00ab24942`였다. 후속 payload는 `040a5360`에 rebase됐다.

따라서 당시에는 기존 두 FAIL이 해결됐으나 #2/#15/#26 PARTIAL 세 개와 parent-sync #9 PARTIAL 하나가 남아 **중간 후보의 기존 30관점 결과는 PASS 26 / PARTIAL 4 / FAIL 0**이었다. 이는 아래 final follow-up 전 historical 결과다.

## Final candidate targeted recheck addendum · pass06 · 2026-10-26

최종 후보는 checkout `HEAD`, local `origin/main`, remote `main`이 모두 `040a5360be92f73e141797aa7dcd4ef00ab24942`다. `gh pr list --head plan/c11-4-image-auto-max-mixed-track-20261026`은 비어 있고 authored plan/fixture/evidence는 아직 local worktree payload다.

| 기존 행 | 최종 처분 | 재확인 |
|---:|---|---|
| #27 | **PASS — 해소 유지** | HTML/V8 positive item은 authored overflow가 없다. `<img>` `clip/clip`은 Chrome UA CSSOM 관측일 뿐이며 app computed value, projection, clip/scroll/paint acceptance가 없다. |
| #28 | **PASS — 해소 유지** | final plan/JSON에는 `Clip/Visible`, `Visible/Clip` 또는 overflow-specific adapter error control이 없다. 기존 fail-closed matrix는 root/item/source/grid/decode 등 범위 내 경계만 다룬다. |
| #2 | **PARTIAL 유지** | remote plan PR이 없고 authored payload가 untracked라 GitHub PR diff/CI state는 확인할 수 없다. |
| #15 | **PARTIAL 유지** | CSS Images dated source URL, byte count 및 hash는 남아 있지만 raw response preimage가 evidence directory에 없다. |
| #26 | **PARTIAL 유지** | metadata cleanup summary는 남아 있지만 CLI/process inventory 원문 log는 evidence directory에 없다. |
| #9 | **PASS 복원** | parent entry는 child/evidence/pass04/pass05/follow-up을 링크하고 pass05를 pre-fix historical result로 표시한다. overflow acceptance/control이 sizing slice 밖이라는 현재 scope와 C11.4 미완료/`72/213`도 child/parent/status와 일치한다. |
| #23 | **PASS 유지, 이전 관측 superseded** | final observation은 root `visible/visible`, image `<img>` `clip/clip`을 기록한다. Plan/README는 이를 Chrome CSSOM에 한정하고 Spinon overflow 동작으로 확대하지 않는다. |

따라서 **기존 PR diff-30 관점의 최종 후보 결과는 PASS 27 / PARTIAL 3 / FAIL 0**이다. PR 자체는 아직 존재하지 않으므로 이 값은 remote diff/CI verdict가 아니다. Pass06 fresh 20관점 report에서 발견한 README line 10의 언어 PARTIAL은 [targeted follow-up](./c11-4-image-auto-max-mixed-track-plan-review-pass06-follow-up-2026-10-26.md)에서 해소했다. 이 변경은 기존 30관점 점수와 무관하다. 본 follow-up은 review report만 수정했고 authored plan/evidence/product files, tests, browser, simulator, commit은 변경·실행하지 않았다.
