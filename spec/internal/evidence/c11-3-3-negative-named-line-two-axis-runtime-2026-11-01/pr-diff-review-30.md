# C11.3.3 two-axis negative named-line pair PR diff review · 30개 독립 관점

검토 대상은 PR #970의 source, tests, mobile fixture routing, captured evidence, candidate contract와 plan 문서다. 최초 검토에서 발견한 auto-size fallback은 `0954c9cef855237510563a7ad3585e190249ee49`에서 닫았다. 아래 판정은 그 수정과 수정본 양 플랫폼 재실행을 반영한다.

| # | 실패 관점 | 확인 및 최종 판정 |
|---:|---|---|
| 1 | 새 placement mode가 기존 one-axis 기능을 덮어쓰는가 | 별도 `TemplateGridNegativeNamedLinePairAutoImplicitTracksBothAxes` variant로 분리했다. 기존 one-axis Android/iOS regression은 각각 8/8 exact다. PASS. |
| 2 | 양 축 중 하나만 implicit pair를 만들 때 실수로 dual-axis mode를 고르는가 | 두 축 usage flag를 모두 요구한다. style projection test는 one-axis와 both-axis mode를 구분한다. PASS. |
| 3 | start/end named ident가 한 축 안에서 서로 다른가 | 기존 resolver의 same-name pair predicate 뒤에만 usage를 허용한다. mismatch/missing occurrence tests가 남아 있다. PASS. |
| 4 | row와 column의 custom-ident가 서로 다른가 | cross-axis predicate가 네 authored ident를 비교하며 대소문자를 보존한다. 불일치 input은 dual-axis gate를 통과하지 않는다. PASS. |
| 5 | auto-sized item이 generic signed mode로 빠져 잘못 수용되는가 | 최초 PR review에서 재현했다. pair shape를 item size와 독립해 mode에 묶고 layout에서 `UnsupportedGridItemSize`로 거부하도록 고쳤다. focused style regression과 수정본 Android/iOS run이 통과했다. PASS. |
| 6 | `grid-template-areas`와 no-area named-line sizing이 섞이는가 | projection gate와 layout validator 모두 area template 조합을 차단한다. focused error assertion이 `grid-template-areas`를 확인한다. PASS. |
| 7 | hidden 또는 non-participating child가 단일 item 제한을 우회하는가 | layout validator는 hidden state를 반영한 participating direct-child 수로 검사한다. 두 participating child test는 거부된다. PASS. |
| 8 | 자식이 없거나 item input이 누락돼 조용히 빈 grid가 되는가 | zero-child와 missing-item shape가 `InvalidGridPlacement`로 닫히며 frame 성공을 반환하지 않는다. PASS. |
| 9 | 양 축 endpoint 중 하나가 positive, mixed, auto, span 또는 definite line인가 | 전용 shape validator는 두 축 모두 negative integer `Line` 쌍이어야 한다. row mixed/positive와 one-axis mode controls가 거부된다. PASS. |
| 10 | 음수 occurrence가 explicit grid extent 안에 머무는가 | validator는 각 축 track count에서 계산한 first leading implicit line보다 더 앞선 pair만 허용한다. boundary와 range tests를 확인했다. PASS. |
| 11 | equal endpoints가 zero-sized span 또는 division 오류를 만드는가 | equal-endpoint fixture는 유효한 one-track span 및 expected track sizes를 만든다. Android/iOS pixel sample이 Chrome과 exact다. PASS. |
| 12 | 두 끝점 역순에서 열·행 정규화가 서로 영향을 주는가 | both-reversed, columns-only reversed, rows-only reversed를 별도 scene으로 실행한다. frame과 표면 sample이 모두 일치한다. PASS. |
| 13 | `grid-auto-columns` pattern phase가 rows 계산을 이동시키는가 | 5/7px column pattern과 3/4px row pattern의 최종 arrays를 축별로 고정하고 비교한다. PASS. |
| 14 | `auto` contribution이 explicit track size와 혼합돼 잘못 분배되는가 | 독립 기대값 Columns `[6,6,6,0,20]`, Rows `[4,4,4,0,15]` 및 item `(0,0,18,12)`를 Rust와 Chrome 기준에서 비교했다. PASS. |
| 15 | fixed pattern track이 충분하지 않을 때 phase가 끝난 뒤 반복되는가 | 기대 arrays `[5,7,5,7,20]` 및 `[3,4,3,4,15]`가 row/column 방향에서 각각 검증됐다. PASS. |
| 16 | item이 선택한 fixed-px implicit area보다 클 때 sizing이 item 폭으로 바뀌는가 | 18×12 item과 17×10 fixed area를 별도 overflow scene으로 둔다. area 크기와 item frame, Chrome 표본을 독립 비교한다. PASS. |
| 17 | `grid-auto-*`의 `auto` 자체를 track list 항목으로 넣을 수 있는가 | track-list `Auto`는 `UnsupportedGridAutoColumns/Rows`로 닫는 focused layout controls가 있다. PASS. |
| 18 | negative, NaN 또는 infinite breadth가 track arithmetic에 들어가는가 | common track validator가 non-finite를 거부한다. 양 축 fixture는 infinite row breadth를 검사하고 기존 negative-length controls도 유지한다. PASS. |
| 19 | implicit expansion 한도가 넘을 때 부분 frame이 publish되는가 | 10,001개 요청은 `GridTrackLimitExceeded`와 10,000 ceiling을 확인한다. bounded failure만 반환한다. PASS. |
| 20 | normalized container와 renderer가 새 mode를 누락하는가 | normalization과 dense-column route에서 signed implicit track normalizer로 전달한다. 13 frame runtime과 PixelCopy/Metal samples가 end-to-end 연결을 확인한다. PASS. |
| 21 | Android host가 다른 fixture scene ID를 두-axis route로 오인하는가 | `MainActivity`와 GPU demo의 exact allowlist에 새 ID만 추가했다. 미등록 scene mutation은 runtime draw 전에 거부됐다. PASS. |
| 22 | Android 표본 좌표가 item 바깥이나 다른 frame을 가리키는가 | 여섯 sample의 CSS 좌표와 blue RGBA를 Chrome 관측에 고정했다. 잘못된 좌표 mutation은 PixelCopy readback 전에 거부됐다. PASS. |
| 23 | iOS host가 Android와 다른 ID 또는 sample 좌표를 사용해 결과를 숨기는가 | Swift allowlist와 sample tuple이 같은 여섯 case를 사용한다. iOS mutation log는 readback 전 거부를 기록한다. PASS. |
| 24 | iOS surface 비교가 compositor 이후 픽셀인 것처럼 과장되는가 | 근거와 contract는 결과를 Metal drawable, compositor 제출 전으로 한정한다. PASS. |
| 25 | Android software renderer 결과를 실기기/hardware GPU 증거로 확대 해석하는가 | 실행 근거는 API 37 AVD/SwiftShader라고 명시하고 실기기·hardware GPU를 제외한다. PASS. |
| 26 | test-only scene payload가 기본 FFI archive에 노출되는가 | 기본 macOS archive에서 세 marker 0개, opt-in fixture positive control에서 3개를 확인했다. 문서는 이를 APK/IPA 전체 검사라고 부르지 않는다. PASS. |
| 27 | fixture scene source가 브라우저/앱 runtime에서 13개 예상 node와 다르게 만들어지는가 | source contract test는 six case IDs, track patterns, named placements와 color marker를 검사한다. simulator result는 13 frames를 보고한다. PASS. |
| 28 | negative coordinate/unknown scene guard가 실제 capture script에서 잘못된 run ID를 허용하는가 | script는 fixture schema로 전용 mode를 선택하고 80자 한도 내 suffix를 만들어 negative mutation을 독립 실행한다. Android/iOS run logs는 각각 거부 시점을 기록한다. PASS. |
| 29 | app hash 또는 fixture/oracle hash가 다른 run에 붙는가 | source revision JSON은 commit `0954c9c`, fixture hash, Android app hash와 iOS app hash를 각각 exact run ID에 연결한다. 두 결과 모두 `matched`다. PASS. |
| 30 | 제한 candidate가 WPT/full CSS Grid 지원 또는 공식 완료로 문서화되는가 | plan, contract, STATUS와 PR 본문은 C11.3.3/C11을 미완료로 두고 official `72/213`을 유지한다. 범위에서 일반 CSS admission, WPT와 전체 Grid를 제외한다. PASS. |

GitHub PR #970의 check rollup은 현재 비어 있다. 따라서 hosted CI 통과로 기록하지 않는다. PR은 review 종료 시점에 open이며 merge되지 않았다.
