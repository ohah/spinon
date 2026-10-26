# C11.4 image auto/max mixed-track PR #643 독립 변경 검토 30

## 검토 범위와 판정

- 검토한 GitHub PR: [#643](https://github.com/ohah/spinon/pull/643)
- 검토 대상 base: `040a5360be92f73e141797aa7dcd4ef00ab24942`
- 검토 대상 GitHub head: `cac4e33950aa55e8bb8e7ec1f81bf790316ba7ac`
- PR 상태: open, mergeable. `gh pr checks`는 보고된 check가 없었다. CI 통과로 해석하지 않았다.
- 별도 source-bound 앱 검증은 현재 head `cac4e339`에서 수행한 Android API 37 ARM64 AVD / iPhone 17 Pro iOS 26.2 Simulator의 `final05`까지 대조했다. manifest는 [source-hashes.json](../spec/internal/evidence/c11-4-image-auto-max-mixed-track-runtime-2026-10-26/runs/c114-image-auto-max-20261026T020007Z-cac4e339-final05/source-hashes.json)이다. 소스, APK, iOS 실행 파일 및 screenshot SHA-256을 실제 파일과 다시 비교했고, 두 화면을 직접 열어 상태 overlay를 확인했다. 이 실행 자료와 대표 이미지는 아직 GitHub에 push되지 않은 local candidate다.
- 판정: **24 PASS / 6 PARTIAL / 0 FAIL**. 구현 코드에서 확인된 차단 결함은 없다. final05 자료를 commit/push하고 PR 본문 및 screenshot URL을 그 증거에 맞게 갱신한 뒤 새 URL의 Agent Browser 확인을 끝내기 전에는 PR evidence gate가 닫히지 않는다.

## 30개 서로 다른 실패 경로·불변 조건

| # | 확인한 실패 경로·불변 조건 | 근거와 관찰 | 판정 |
| ---: | --- | --- | --- |
| 1 | 제한 fixture 한 건 때문에 C11.4 전체나 공식 완료 수가 잘못 완료 처리되는가 | [STATUS](../spec/STATUS.md)와 계획은 C11.4 상위 및 Stage A/B를 미완료로 두고 `72/213`을 유지한다. | PASS |
| 2 | opt-in scene이 일반 author CSS admission이나 공개 API 지원으로 표현되는가 | [작업 계획](./c11-4-image-auto-max-mixed-track.md)과 PR 범위는 고정 fixture slice로 제한하며 공개 API 변경이 없다고 구분한다. | PASS |
| 3 | C11.4 FFI scene registry에 새 장면이 빠지거나 다른 scene을 덮는가 | [FFI fixture registry](../crates/spinon-ffi/src/runtime_gpu/fixtures/c11_4.rs)는 고유 `image-auto-max-mixed-tracks` ID로 등록하고 등록 테스트가 source를 확인한다. | PASS |
| 4 | runtime case가 FFI scene과 root/item ID 또는 geometry를 다르게 선언하는가 | [runtime case matrix](../crates/spinon-runtime/src/session/ua_cascade/runtime_layout/fixtures/c11_4_image/cases.rs)의 100×40 root, 지정 ID, 17px 이웃, 5px gap이 FFI scene과 일치한다. | PASS |
| 5 | Android/iOS allowlist 또는 예상 root/item frame에서 scene ID가 누락·오타나는가 | 플랫폼 matcher의 `image-auto-max-mixed-tracks` 분기와 root 100×40/item 24×16 기대값이 일치하고 final05 양쪽 runtime summary가 통과했다. | PASS |
| 6 | HTML oracle 또는 고정 PNG asset이 바뀌어 24×16 기대값이 다른 입력에 붙는가 | [HTML fixture](../tests/fixtures/css/c11/grid-image-auto-max-mixed-track-c114-precomparison.html)와 [precomparison](../spec/internal/evidence/c11-4-image-auto-max-mixed-track-precomparison-2026-10-26/README.md)은 입력 및 PNG hash를 기록한다. PNG는 97 bytes, natural 24×16이다. | PASS |
| 7 | `minmax(auto,max-content)`에서 replaced image의 automatic minimum을 잘못 계산하는가 | 독립 CSS Grid 2025 CRD 계산은 definite inline axis, span 1, scrollable overflow 아님, 지정 크기 없음의 조건에서 content-based minimum 24px를 도출한다. Chrome 관측은 계산을 대체하지 않는다. | PASS |
| 8 | max-content maximum이 batch 뒤에도 무한 또는 0으로 남는가 | 고정 Taffy 0.15.0 source의 growth-limit update는 `IntrinsicItemBatch` trace 기록 전에 실행된다. 이후 열 24px/행 16px가 맞다. | PASS |
| 9 | row axis가 auto height 대신 viewport 여유분으로 늘어나는가 | natural image height 16px 및 `align-content:start`로 row 16px를 기대하고 양 플랫폼 trace/frame은 같은 결과를 낸다. | PASS |
| 10 | fixed neighbor, gutter 또는 남는 definite free space를 잘못 더해 final frame이 달라지는가 | 열 `[24,17]`, gap 5, trailing free space 54, root 100×40, item `(0,0,24,16)`의 독립 산술과 runtime frame이 일치한다. | PASS |
| 11 | Chrome CSSOM 값이나 Spinon candidate 출력이 규범 기대값의 근거로 순환 사용되는가 | precomparison README/expectation JSON은 규범 계산과 Chrome 관측을 구분하며, candidate는 acceptance 수치의 oracle로 쓰지 않는다. | PASS |
| 12 | Taffy trace 표현이 고정 dependency와 다르게 해석되는가 | vendored [`track_sizing.rs`](../vendor/taffy/src/compute/grid/track_sizing.rs)와 pinned `taffy = 0.15.0`을 대조했다. | PASS |
| 13 | CSS track 개수와 raw trace entry를 혼동해 boundary sentinel/gutter를 누락하는가 | [기대 trace JSON](../spec/internal/evidence/c11-4-image-auto-max-mixed-track-precomparison-2026-10-26/spec-derived-expectations.json)은 두 CSS columns의 5 slots와 한 row의 3 slots를 고정한다. 양 끝 sentinel과 실제 gutter도 따로 비교한다. | PASS |
| 14 | 각 track stage에서 초기 base와 intrinsic contribution 이후 값이 틀리는가 | [runtime evidence](../spec/internal/evidence/c11-4-image-auto-max-mixed-track-runtime-2026-10-26/README.md) 및 final05 runtime logs는 6-stage 전체 slot의 base/growth, gutter/collapsed/infinitely-growable flags를 보여준다. | PASS |
| 15 | 초기 JSON `null`을 내부 `+∞`의 직접 관측이라고 과장하는가 | 계획과 runtime README는 serializer가 모든 non-finite 값을 null로 보내므로 `Initialize` null이 증명하는 범위를 projection으로 제한한다. | PASS |
| 16 | Minimum, MinContent, MaxContent contribution의 item/span/value 또는 automatic minimum provenance가 빠지는가 | Android/iOS matcher는 두 축의 세 contribution, item 2, span 1, 값 24/16, `AutomaticMinimum` flags와 non-minimum null source를 검사한다. | PASS |
| 17 | 다른 HTML root ID 또는 비-HTML root가 새 adapter profile로 들어오는가 | `prepare()`는 root allowlist와 HTML `div` identity를 확인하고, fail-closed tests는 unknown root와 non-HTML namespace를 거부한다. | PASS |
| 18 | extra/nested item, 잘못된 child ID 또는 고정 PNG가 아닌 source가 검사 전 decode로 넘어가는가 | topology, direct child, item identity, leaf, `src` 검사가 computed style 및 decode보다 앞선다. wrong item/source 테스트가 각 gate를 확인한다. | PASS |
| 19 | root의 track list·gutter·box geometry 변이가 고정 scene으로 오인되는가 | validator는 typed track sequence, row/column list, dimensions, border, margins/padding, gaps, alignment 및 repeat/areas/line-name 조건을 제한한다. 변이 테스트가 핵심 경계를 고정한다. | PASS |
| 20 | direct validator rejection 후 runtime 전체가 frame/trace/resource를 만들지 않는다는 주장을 과도하게 하는가 | `prepare()` width 변이는 error와 `C11_4ImageInput` 미반환까지 확인하지만 downstream frame/trace/resource 부재는 직접 관측하지 않았다. README도 그 한계를 명시한다. | PARTIAL |
| 21 | decode 실패, 잘못된 natural dimensions, stale revision, foreign generation 또는 다른 profile이 이 scene에서 허용되는가 | 계획은 관련 adapter error 경계를 적고 shared decoder/snapshot 테스트를 연결하지만, 새 scene의 그 adapter 분기는 직접 실행하지 않았다. 고정 compile-time PNG positive path만 이번 run 범위다. | PARTIAL |
| 22 | fixture ABI/payload가 기본 product FFI archive로 새어 나오는가 | artifact scanner의 `c04-runtime-gpu` negative control은 source `e4570a04`에서 scene marker 0/5, fixture ABI 0/2를 기록한다. 이후 변경은 test/platform/docs이며 Rust product scene source는 바뀌지 않았다. | PASS |
| 23 | positive control이 고장나도 negative archive 검사만으로 통과 처리되는가 | `c04-runtime-gpu-fixture-test` positive control은 marker 5/5와 정의 ABI 2/2를 확인한다. | PASS |
| 24 | static archive 결과를 APK/IPA 전체 fixture exclusion으로 확대 해석하는가 | [artifact README](../spec/internal/evidence/c11-4-image-auto-max-mixed-track-runtime-2026-10-26/feature-boundary/README.md)는 검사 범위를 `spinon-ffi` static archive로 제한하고 shared PNG payload를 새 scene 단독 산출물로 귀속하지 않는다. | PASS |
| 25 | Android JSON parser의 coercion, 누락/null, NaN/Infinity가 정상 trace처럼 통과하는가 | 새 matcher는 Number/Boolean/String 원형, finite numeric, 정확한 integer, 필수 key와 null을 fail-closed 확인한다. final04 정상 runtime trace도 통과했다. 다만 malformed-field mutation을 실제 주입하는 별도 테스트는 없다. | PARTIAL |
| 26 | iOS Foundation `NSNumber`가 Boolean을 숫자로 통과시키거나 finite/integer 비교를 우회하는가 | `CoreFoundation` type ID로 `CFBoolean`과 number를 분리하고 finite/exact integer 검사를 한다. final04 정상 runtime trace는 통과했지만 malformed JSON mutation은 직접 시험하지 않았다. | PARTIAL |
| 27 | 최신 strict matcher와 실행 artifact/source revision이 서로 다른 빌드에서 섞이는가 | final05 [source manifest](../spec/internal/evidence/c11-4-image-auto-max-mixed-track-runtime-2026-10-26/runs/c114-image-auto-max-20261026T020007Z-cac4e339-final05/source-hashes.json)는 정확한 source head `cac4e339`, Java/Swift/test/expectation hashes, V8 revision, APK 및 iOS executable hashes를 연결한다. 원본 파일과 app/screenshot hashes를 독립 비교했고 양쪽 앱 run이 모두 통과했다. | PASS |
| 28 | simulator screenshot을 PNG raster paint/pixel equality 또는 hardware/compositor 이후 출력으로 오해하는가 | final05 screenshot을 직접 열어 status overlay와 scene을 확인했다. 두 runtime log 모두 `runtime_image_count=0`, `image_draw_count=0`; 계획·README·PR alt text는 layout/presentation 범위로 제한한다. 빨간 사각형은 img의 CSS background다. | PASS |
| 29 | PR screenshot 링크가 현재 PR 본문에서 실제 로드되지 않거나 Agent Browser를 남기는가 | 기존 [Agent Browser evidence](../spec/internal/evidence/c11-4-image-auto-max-mixed-track-runtime-2026-10-26/pr-image-link-check-2026-10-26.json)는 immutable `e4570a04` URL 두 개가 로드됐고 close 후 session inactive/pid null/process list empty임을 증명한다. 최신 final05 대표 이미지의 immutable URL은 아직 PR에 올라가 있지 않아 Agent Browser에서 확인되지 않았다. | PARTIAL |
| 30 | 현재 remote PR metadata, local evidence, CI 상태가 서로 다른데 merge-ready라고 표시되는가 | remote head는 `cac4e339`지만 remote 본문은 Android rerun pending 문구, iOS `final03` 및 e457 screenshot URL을 유지한다. final05 README/screenshots/source hashes는 로컬 candidate다. `gh pr checks`에 보고된 check도 없다. 새 evidence를 push하고 본문·URL을 갱신한 뒤 Agent Browser 재확인이 필요하다. | PARTIAL |

## 후속 및 차단 범위

- **차단:** final05 source/app/hash/evidence를 PR에 push하고, PR 본문 Android/iOS 결과와 screenshot 링크를 같은 immutable evidence commit으로 갱신한 뒤 Agent Browser에서 새 두 URL을 확인하고 세션·전용 프로세스 종료 증거를 갱신해야 한다. 현재 remote PR 본문은 그 갱신을 반영하지 않는다.
- **비차단 한계:** malformed JSON mutation test, `prepare()` 이후의 downstream atomicity 관측, 새 adapter의 decode/dimension/stale/profile rejection 실행은 범위의 PARTIAL이다. 이 제한은 계획/evidence에 적혀 있으며 현재 positive fixture 경로에서 재현된 결함은 아니다.
- GitHub check rollup은 비어 있으므로 CI 성공을 주장하지 않는다. PR은 open/mergeable로 확인했으며 merge는 수행하지 않았다.
- PR diff 전체에 포함된 raw build log 일부에는 원본 stdout의 trailing whitespace/마지막 blank line이 있다. 이는 `git diff --check` 경고의 출처이며 runtime evidence는 변경하지 않았다.
