# C11.4 four-fit row-span 구현 PR diff 검토: 30개 실패 관점

**PR:** [#1357](https://github.com/ohah/spinon/pull/1357)

**검토한 head:** `0ef4c4dbac927d20caf0bf587b6babd45c220a48`

**base `main`:** `88c000dabd1a4402faa83b9696e252e2d9b2eb4e`
**검토 전 diff identity:** `gh pr diff 1357`와 `git diff origin/main...HEAD`가 text patch 기준 byte-for-byte 동일했다. SHA-256 `bae1cb09668c97cbff88934954f419981ac477e54a06c3191c222f88296e7f1e`, 2,626 lines, 41 files. 경로 목록도 원격과 local이 같았다. 이 해시는 이 review 문서를 추가하기 전의 동작·문서·evidence diff다. Review 문서와 링크를 포함해 푸시한 뒤 전체 diff를 다시 대조한다.

| # | 적대 검증 관점 | PR diff 확인과 결과 |
| ---: | --- | --- |
| 1 | PR이 예상 branch/base와 다른 상태를 가리킴 | #1357은 `codex/c11-4-repeated-text-four-fit-auto-min-20261111` → `main`; head/base SHA가 위 값과 일치한다. |
| 2 | GitHub PR의 patch가 로컬 branch와 다르거나 누락됨 | 원격·로컬 patch의 SHA-256과 byte 비교가 같고 파일 경로 41개도 일치한다. |
| 3 | 대량의 무관한 snapshot·artifact가 PR에 섞임 | PR은 관련 Rust 코드·테스트, 계획/상태/계약/roadmap, 지정된 task evidence만 포함한다. 사전부터 있던 untracked artifact는 들어가지 않았다. |
| 4 | 작업 종류·영역 라벨 누락 | `enhancement`, `area:testing`, `area:layout`이 PR에 붙었다. |
| 5 | 제한 slice를 공식 C11.4/C11 완료로 승격 | `spec/STATUS.md`의 해당 공식 checkboxes는 미완료이고 `78/214`가 유지된다. |
| 6 | 내부 contract 숫자나 public surface를 부수적으로 변경 | 내부 숫자 버전은 `0.1.0`; diff에 public API·FFI·ABI 변경이 없다. |
| 7 | 잘못된 list 길이가 네-fit predicate에 들어옴 | Style/layout 양쪽이 정확히 6 members를 요구하고 style test가 length 5를 거부한다. |
| 8 | 네 fit 외의 tail multiset이 통과 | Exact tail은 `min-content ×1`, `auto ×1`; auto-placement의 `max-content` tail과 분리되고 잘못된 tail cases가 양 계층에서 거부된다. |
| 9 | 중복 cap이 누락 cap을 대신함 | Seen-cap 배열은 네 등록 cap을 한 번씩 요구한다. Style/layout tests와 runtime mutation이 duplicate/missing cap을 거부한다. |
| 10 | `20%`가 같은 숫자의 px 또는 임의 percentage로 해석됨 | Percentage branch는 typed `Percentage(0.2)`만 인정하고 computed type/order를 matrix의 각 scene에서 검사한다. |
| 11 | 미등록·NaN·계산식 cap이 admission을 우회 | Style/layout tests는 non-finite/calc typed values를 거부하고 runtime rejection은 미등록 cap과 unresolved percentage `calc()`를 거부한다. |
| 12 | 위치 조합 일부가 빠지거나 고정 순서만 허용 | 네 fit positions의 15개 quadruple, 24 cap orders, 두 tail orders를 projection/layout에서 각각 720 cases로 확인하고 runtime matrix에 포함한다. |
| 13 | Style projection과 layout layer가 서로 다른 허용 집합을 사용 | 두 계층은 같은 exact typed caps·tail·길이를 검증하며 각 layer unit tests가 720 combinations를 실행한다. |
| 14 | fit placement 검사에서 6 positive implicit row 제한이 약화 | Position gate는 positions 1–6의 strict order와 positive implicit row count 6을 요구한다. Runtime mutation은 span이 seventh row를 요구할 때 실패한다. |
| 15 | 지원 예외가 다른 Grid owner·topology·placement로 확장 | 기존 parsed Root HTML owner, 고정 sparse row-flow/span families와 `visible` overflow 경로를 재사용한다. `dense`, 추가 row, 다른 alignment와 scrollable item은 거부한다. |
| 16 | 빈 explicit control 또는 mixed minimum이 candidate로 승인 | 첫 control은 `min-height:0`; 모든 span text item은 같은 zero/auto profile이어야 한다. Mixed media revision의 rejection test가 이를 확인한다. |
| 17 | non-eligible item contribution을 automatic minimum으로 계산 | Trace test는 자동 최소 eligibility metadata, source span, content-based line floor, final spanned row sum을 item마다 검사한다. |
| 18 | baseline과 candidate 집계가 서로 섞이거나 입력 쌍이 불완전 | 11,520 scenes는 5,760 paired keys다. Matrix는 모든 pair의 minimum mode와 변하지 않아야 하는 scene fields를 검사한다. |
| 19 | DOM source-order 역전에서 frame이 잘못된 item에 대응 | Stable item IDs로 frame map을 대조하고 forward/reverse row sizes와 geometry pair도 확인한다. 관측 geometry delta는 0 CSS px다. |
| 20 | line boxes는 다르지만 item/frame 총계만 같아 통과 | 각 scene에서 item frame, payload frame, glyph line count, span row sum을 Chrome observation에 직접 비교한다. |
| 21 | track trace가 stale revision 또는 다른 Grid owner에 붙음 | Trace test는 completed cascade key, Grid node, item inputs의 revision/owner와 typed authored `auto_rows`를 맞춘다. |
| 22 | candidate rounding이 content floor를 버리거나 fit cap을 초과 | Candidate 25,920 eligible occurrence의 floor violation은 0이며 trace의 final track base와 minimum contribution을 비교한다. |
| 23 | baseline rounding도 candidate 정책으로 바뀜 | Baseline matrix의 3,648 under-floor occurrence를 별도 보존하고 nearest/fractional auto residual controls를 고정했다. |
| 24 | stretch 분기에서 auto row 이외의 값이 변하거나 post-stretch fraction을 읽음 | 5,760 `start|stretch` pairs에서 non-auto row가 고정되고 auto row만 늘 수 있다. fractional-autotrack 정책은 `pre_alignment_sizes`에서 판정한다. |
| 25 | 지원 밖 cap·flow·overflow·basis 입력이 조용히 수용 | Duplicate/missing/unsupported cap, fifth fit, unresolved calc, dense row flow, scrollable child, unsupported alignment, indefinite percentage basis가 rejection cases다. |
| 26 | 실패 revision이 부분 layout/frame을 남김 | Rejection assertions require `Failed`와 `completed=None`; stale render output을 성공값으로 취급하지 않는다. |
| 27 | rejection 뒤 session이 복구 불가하거나 이전 key를 재사용 | Dark media mutation rejection 뒤 light revision이 same RuntimeSession에서 다른 key와 기대 frame으로 복구한다. |
| 28 | rounding 변경이 기존 auto-placement four-fit을 깨뜨림 | Chrome 157 four-fit auto-placement 5,760-scene RuntimeSession regression 전체가 통과했고 projection/layout/trace/rejection focused cases도 통과했다. |
| 29 | simulator screenshot을 feature geometry나 pixel parity로 과장 | Build flags에서 C11.4 app fixture는 꺼져 있다. PR은 Android SwiftShader/iOS 기본 Root HTML smoke라고 설명하고, feature geometry는 RuntimeSession matrix로 구분한다. PixelCopy/Metal readback·실기기·WPT는 미검증이다. |
| 30 | PR 본문 screenshot 링크가 깨졌거나 상태 증거가 부정확 | Agent Browser에서 네 이미지 모두 `complete=true`; iOS 각각 1206×2622, Android 각각 1080×2400이다. PR check rollup은 `no checks reported`; CI green으로 기록하지 않는다. roadmap preview는 HTTP 200이며 `78/214`와 four-fit 진행 중 카드를 표시한다. |

**결과:** 검토 관점에서 새 diff 결함을 찾지 못했다. 이 기록과 README 링크를 포함해 branch를 갱신한 후 원격/local patch·path list를 재비교한다. 현재 PR에는 GitHub check가 보고되지 않았으며 이 결과는 CI 성공이나 C11.4/C11 완료를 뜻하지 않는다.
