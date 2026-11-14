# C11.4 one-fit row-span mixed minimum · PR diff 실패 관점 30개

- PR: [#1381](https://github.com/ohah/spinon/pull/1381)
- 검토 대상 diff: `main` base `929c4475517ec9a981deccffafa3af62c84238c1` → head `f2571fab80af074c6ca884c47e6ee4bab1444f90`
- 검토 방식: 변경 Rust 코드·테스트·명세·로드맵·실행 로그·이미지를 파일별로 대조했다. Agent Browser에서 PR 페이지와 이미지 본문도 확인했다.

| # | 실패 관점 | 대조·판정 |
| ---: | --- | --- |
| 1 | 새 gate가 0-fit 또는 two-fit row-span까지 허용함 | 새 style/layout predicate는 six-member span list의 fit-content 수가 정확히 하나일 때만 mixed mask를 허용한다. 기존 two-fit 3,840-scene과 auto-placement 회귀가 별도다. |
| 2 | six-member track list 길이·tail multiset이 달라짐 | 기존 min-content/max-content/auto tail 및 one-fit cap/order profile을 유지한다. Chrome batch denominator 900/900이다. |
| 3 | positions 1–3과 4–6의 implicit row count가 혼합됨 | 세-row topology는 positions 1–3, 여섯-row topology는 positions 4–6만 허용한다. projection boundary test와 두 uniform endpoint matrices가 이를 확인한다. |
| 4 | 허용 cap 순서가 track index를 잘못 해석함 | list마다 computed `auto_rows`를 Chrome track order와 비교하고 trace owner에서 position·cap을 다시 대조한다. |
| 5 | forward/reverse placement가 같은 span topology로 오인됨 | exact placement multiset matcher와 4 topology variant를 사용한다. scene index와 semantic IDs를 보존한다. |
| 6 | mask bit가 DOM 순서가 되어 reversal 후 auto 대상을 바꿈 | observation의 `maskItemIds`를 기준으로 computed minimum을 item ID별 검증한다. source-order pairs가 matrix에 포함된다. |
| 7 | explicit control의 empty box가 `auto`를 받음 | projection/layout은 첫 child의 computed/layout minimum `0px`을 요구하고 RuntimeSession에서 control frame과 style을 확인한다. |
| 8 | span child의 `min-height:1px`, `min-content` 등이 우연히 허용됨 | minimum mask validator는 `0px|auto`만 허용한다. nonzero/intrinsic minimum 및 control-auto negatives가 rejection evidence에 있다. |
| 9 | 숨김 child 또는 추가 Grid owner가 profile을 넓힘 | owner cardinality와 visible direct child/profile 검사를 유지한다. media-hidden variant는 선택된 한 topology만 layout에 들어가고 owner/revision trace를 확인한다. |
| 10 | auto minimum이 item별이 아니라 Grid 전체에 적용됨 | computed style와 row trace contribution을 scene의 item ID별 비교한다. eligible occurrence 169,344개에서 source·값·span·auto-track 조건을 확인한다. |
| 11 | scrollable overflow인 item에 content-based auto minimum을 강제함 | fixed visible-overflow contract 밖 입력은 fail-closed다. matrix fixture에는 clipping/scroll 스타일을 추가하지 않았다. |
| 12 | flexible max track 포함 span에서 eligibility가 잘못됨 | replay의 `crosses_flexible_track`를 거부하고 trace에서 `spans_flexible_max_track:false`를 요구한다. |
| 13 | fit-content cap이 content-based minimum을 위쪽에서 잘라냄 | Chrome line floor, RuntimeSession text line count, contribution value와 row/item/payload geometry를 같이 비교한다. |
| 14 | span의 covered row range가 한 칸 밀림 | line 2 기준 implicit-row offset을 사용하고 각 item frame height를 covered row 합과 대조한다. 모든 120,960 scene이 통과했다. |
| 15 | nested payload 높이/line wrapping이 row sizing 결과를 숨김 | text payload는 고정 monospace grammar이며 actual glyph baseline의 고유 line 수, payload frame과 content floor를 Chrome 관측과 비교한다. |
| 16 | Grid content edge 밖 paint를 지원 완료로 잘못 셈 | 현재 fixed oracle에서 overflow는 0이다. runtime frame은 oracle과 0.01px 이내로 묶고 overflow fixture를 만들었다고 주장하지 않는다. |
| 17 | owner ID는 같지만 completed layout과 cascade가 다른 revision임 | completed layout key와 cascade key를 비교하고 owner-scoped 최신 row pass를 찾는다. |
| 18 | 누락·중복·reordered trace가 stale 결과를 재사용함 | trace integrity rejection과 same-session recovery를 실행했다. missing/out-of-range grid index는 panic 대신 typed unsupported error를 반환한다. |
| 19 | sizing replay가 중간 intrinsic contribution phase를 건너뜀 | minimum, content-based minimum, max-content minimum, intrinsic maximum, max-content maximum phase 순서로 적용하고 각 stage base size를 trace와 비교한다. |
| 20 | growth-limit을 base-size로 다시 더해 1/64px total이 달라짐 | mixed one-fit은 intrinsic base replay를 우선하고 LayoutUnit target sum 및 final Taffy state를 모두 통과할 때만 채택한다. residual scene 110과 전체 matrix가 통과했다. |
| 21 | fallback residual allocator가 임의 row에 값을 배분함 | replay가 target sum을 만족하면 replay를 유지한다. fallback은 같은 total과 final trace를 만족할 때만 허용한다. |
| 22 | stretch가 non-auto row를 바꾸거나 tail이 남은 residual을 흡수하지 못함 | start/stretch pair와 singleton auto-tail residual을 분리 검사한다. non-auto row와 final trace 불일치면 결과를 거부한다. |
| 23 | explicit track나 border/padding을 implicit row offset에 포함함 | 지원 Grid는 고정 40px column·10px explicit row, zero edges/gaps다. frame normalization은 grid border를 content frame에 반영한다. |
| 24 | 한 scene의 computed track/minimum만 검사하고 다른 revision frame을 결합함 | batch 시작 layout-ready 확인 뒤 모든 scene에서 새 environment revision을 기다리고 같은 cascade key/frame/trace를 요구한다. |
| 25 | matrix가 중복 list를 세거나 tail group을 누락함 | record를 track-list batch로 묶고 900 unique list, metadata scene 수 120,960, 예상 batch 크기를 끝에서 재확인한다. |
| 26 | 3-row matrix가 48 scenes, 6-row matrix가 192 scenes가 되는 이유가 누락됨 | topology별 item-mask 폭과 두 alignment 수를 명시적으로 검사한다. 전체 denominator 및 unique-list denominator가 모두 일치한다. |
| 27 | timeout을 제품 실패나 성공으로 잘못 분류함 | 5초 `Pending` 종료와 scene 99449 단독 1.78초 재현을 보존했다. per-scene wait 30초에서 전체 matrix가 실제로 끝난 로그가 있다. |
| 28 | Android/iOS 앱 이미지가 다른 source revision이나 잘못된 route에서 옴 | APK 및 iOS bundle의 HTML SHA-256이 fixture `a944…7676`과 일치하고 C04.12 Root HTML route launch args를 기록했다. 최초 잘못된 Android route/CSS와 iOS launch 실패 자료는 성공 증거와 분리 보존했다. |
| 29 | simulator 캡처가 PixelCopy, Metal readback, 실기기 또는 hardware GPU 결과로 과장됨 | 실행 문서는 이를 light/dark visual smoke로 한정한다. Android는 Vulkan llvmpipe, iOS는 simulator이며 surface parity 주장은 없다. 시작 시 superseded `status=-12`와 이후 성공 request도 기록했다. |
| 30 | PR 본문 그림/상태·라벨·검토·CI가 실제 원격 PR과 다름 | Agent Browser PR 페이지에서 labels `enhancement`, `area:layout`, `area:testing` 및 네 이미지가 로드되는 것을 확인했다. natural size는 Android 각 1080×2400, iOS 각 1206×2622다. PR은 현재 draft이고 Checks 0개이므로 CI green으로 말하지 않는다. `STATUS.md` 78/214와 C11.4/C11 unchecked 상태는 유지한다. |

## 발견 후 정리

- Layout unit test에서 invalid trace index panic을 발견해 fail-closed 오류로 수정했다. Layout 관련 56/56 및 post-split focused test 1/1을 통과했다.
- Projection five-fit expectation test가 기존 shared helper 동작과 불일치해 제품 코드는 유지하고 assertion을 수정했다. 관련 tests 5/5를 재실행했다.
- Full matrix의 한 장면이 초기 5초 wait에 `Pending`을 반환했지만 같은 scene 단독은 통과했다. wait를 30초로 조정하고 full 120,960-scene matrix를 끝까지 통과했다.
- Raw build/emulator/test logs의 명령 출력 끝 공백은 원본 로그에 남아 있다. Rust/source/docs 변경의 whitespace 검사는 통과하며 로그 파일 내용을 정규화하지 않았다.

**판정:** PR diff에서 지원 범위 확대, trace/revision 혼합, fixture provenance 과장 또는 unverified pixel/CI claim은 발견하지 않았다. 30개 관점에서 확인한 bounded contract를 유지한 상태로 draft를 ready로 전환하고, GitHub checks/required checks를 별도로 확인한 뒤 merge할 수 있다.
