# C11.4 later fit-content row-span 구현/runtime 실패 관점 검토 20개

검토 대상은 기본 parsed Root HTML `RootHtmlFitContentAutoRowsV1` admission, Layout frame normalization, Style projection, 신규 RuntimeSession matrix와 rejection/recovery, Android API 37 ARM64 AVD 및 iPhone 17 Pro/iOS 26.2 Simulator smoke다. 이는 선행 계획의 20관점 검토와 별개이며, plan review를 재사용하지 않았다.

| # | 실패 관점 | 확인한 방어·근거 | 판정 |
|---:|---|---|---|
| 1 | CSS의 1-based position과 Rust의 0-based index가 어긋나 position 2가 position 3으로 이동 | Runtime fixture는 `fit_position - 1`에 삽입하고 typed computed order를 각 scene에서 비교한다. 두 position 모두 960 scene이다. | 통과 |
| 2 | `fit-content()` 추가 중 tail member가 빠져 track 수나 keyword multiplicity가 달라짐 | Layout·Style helper 모두 list 길이 6과 `min-content×2`, `max-content×2`, `auto×1`을 exact 검사한다. 1,920 scene 구조 검증이 같은 multiset을 확인한다. | 통과 |
| 3 | 기존 position 1 지원이 새 helper 변경으로 사라짐 | 기존 position-1 960-scene row-span과 810-scene repeated-text 회귀를 다시 실행했다. | 통과 |
| 4 | 새 helper가 position 4–6까지 허용 | track index 3–5의 input을 RuntimeSession에서 atomic rejection하는 focused case를 추가했다. | 통과 |
| 5 | 새 position의 9-item auto-placement가 span profile과 섞임 | Position 2·3 + legacy nine-item auto-placement 거부 regression을 추가했다. 처음 발견한 guard가 position-1 legacy profile까지 막던 회귀는 candidate 구분으로 수정했고 기존 30-order 검사도 재통과했다. | 통과 |
| 6 | 4-member 및 기존 3·5-item profile의 개수 조건이 바뀜 | 새 six-member candidate와 기존 list helper를 분리하고 기존 focused Layout/Style 및 RuntimeSession regressions를 실행했다. | 통과 |
| 7 | duplicate/missing fit-content, 예상 밖 track type 또는 잘못된 cap이 통과 | Layout·Style predicates는 fit track를 하나만 허용한다. 신규 RuntimeSession duplicate-fit rejection과 existing cap/parser rejection tests를 확인했다. | 통과 |
| 8 | percentage cap이 viewport 또는 intrinsic row basis를 사용 | profile의 definite 220px content block basis와 Chrome corpus의 20%·0%·120% 관계를 RuntimeSession matrix에서 비교했다. | 통과 |
| 9 | computed track list가 authored order 대신 정규화된 순서로 projection됨 | 각 scene의 computed typed list를 기대 track list와 순서까지 비교하고 30개 tail order를 모두 돈다. | 통과 |
| 10 | hidden/whitespace child가 span topology의 직접 item 수를 바꿈 | Layout은 hidden subtree를 제외해 topology를 검사하고 Style은 parsed visible children을 exact item list로 수집한다. matrix positive 입력은 세/네 개 topology로 한정한다. | 통과 |
| 11 | span topology 이외의 authored placement가 성공 profile에 섞임 | 신규 validator는 빈 explicit cell 뒤에 row 2/span 2, row 3/span 2를 각각 한 번 요구하고, 네 item 경우에만 row 2/span 3 하나를 추가 허용한다. | 통과 |
| 12 | auto column 생성 또는 item이 두 번째 column으로 흘러 row 결과를 오염 | Grid는 단일 40px column이고 각 item은 column 1/2다. Runtime matrix는 x/width 및 placement를 exact 비교한다. | 통과 |
| 13 | named line, template area, auto-repeat 또는 alternate flow가 새 경로로 들어옴 | 두 profile validator가 빈 line names, template-area/auto-repeat 부재와 row flow를 검사한다. | 통과 |
| 14 | 지원하지 않는 alignment가 style/layout 중 한쪽에서만 허용 | `start`와 `stretch`만 matrix에 포함한다. 기존 projection/Layout allowlist 및 style/runtime 회귀를 확인했다. | 통과 |
| 15 | `stretch`가 `fit-content`·min/max-content intrinsic track까지 늘림 | 독립 Chrome 비교에서 non-auto used tracks 불변, auto-only stretch 관계를 검사한다. Trace-enabled RuntimeSession matrix도 1,920 scene 통과했다. | 통과 |
| 16 | Grid owner가 없거나 둘 이상인 scene에서 다른 Grid의 trace를 사용 | Style projection은 한 visible owner와 handle을 요구하고 Layout normalizer는 node ID로 Grid trace를 결합한다. 기존 owner/revision rejection을 재실행했다. | 통과 |
| 17 | trace 누락·중복·row index 불일치를 frame 보정에 사용 | trace-feature positive matrix는 row trace 하나, explicit 1 + positive implicit 3, noncollapsed rows와 실제 internal indices를 검사한다. trace atomicity regression도 실행했다. | 통과 |
| 18 | 1/64px frame 보정이 span height나 child payload 위치를 분리 | 기존 span normalizer의 final row trace 기반 보정을 유지한다. 모든 semantic item/payload frame을 Chrome 관측과 비교해 최대 허용 0.01px 안인지 확인했다. | 통과 |
| 19 | 거부 입력 후 partial frame·revision 공개 또는 session이 회복 불가 | Positions 4–6 및 nine-item invalid shape에서 completed output이 없고 revision이 변하지 않는지 검사한 뒤 같은 session valid recovery를 실행했다. | 통과 |
| 20 | Simulator screenshot을 native pixel parity나 실기기 증거로 과장 | AVD/iOS 네 조합은 동일 HTML hash, `layout=ready boxes=10`, draw/present 및 화면만 확인했다. README에 PixelCopy/Metal readback·hardware GPU가 아님을 명시했다. | 통과 |

## 결과

새 1,920-scene 기본 RuntimeSession matrix와 trace-enabled matrix, 기존 position-1 row-span·repeated-text 회귀, invalid input atomicity/recovery, Android/iOS default-route visual smoke를 근거로 20개의 별도 실패 경로를 검토했다. 이 후보에서 발견한 nine-item guard 회귀는 수정 후 해당 regressions를 재실행해 닫았다. C11.4 상위, 공개 CSS 지원 또는 전체 Grid conformance는 완료로 처리하지 않는다.
