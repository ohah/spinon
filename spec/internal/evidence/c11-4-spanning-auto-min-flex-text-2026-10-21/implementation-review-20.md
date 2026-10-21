# 구현/runtime 적대적 검토 20개

대상은 고정 C11.4 text fixture의 FFI source, runtime adapter, layout 결과, Android 실기기와 iOS Simulator 로그다. 기존 계획 검토를 재사용하지 않고 구현 변경의 서로 다른 실패 경로를 확인했다.

| # | 공격 관점 | 확인과 판정 |
|---:|---|---|
| 1 | 임의 scene ID가 고정 코드를 실행시키는가 | FFI adapter는 `spanning-auto-min-flex-tracks`만 반환하고 unknown ID는 `None`; 고정 allowlist 단위 테스트 통과. |
| 2 | root ID만 맞춘 다른 요소가 Grid로 오인되는가 | runtime에서 `div`, 고정 root ID, `RuntimeBlockFlexGridTrackSizingV1`을 요구하고 불일치는 오류로 끝난다. |
| 3 | item 하나를 더 넣어도 기존 장면으로 처리되는가 | root의 직접 자식 수가 정확히 1이 아니면 거부한다. |
| 4 | nested element나 텍스트 없는 item이 통과하는가 | item의 직접 자식 하나와 정확한 text node를 요구한다. nested node, 복수 text, element child는 고정 입력과 다르면 거부한다. |
| 5 | 길이·문자열을 바꿔도 200px intrinsic을 재사용하는가 | 실제 text를 `X` 20개와 비교한 뒤만 측정 입력을 만든다. 다른 content는 거부한다. |
| 6 | 시스템 font/fallback 차이가 기기마다 값을 바꾸는가 | system font 없는 `TextLayoutEngine`에 저장소 Ahem을 등록하고 paragraph를 Ahem 10px로 고정한다. |
| 7 | 줄바꿈·word break가 min-content를 200에서 줄이는가 | `nowrap`, `word-break:normal`, `overflow-wrap:normal`, collapse 및 LTR을 모두 확인하며 Ahem 고정 테스트가 200/200을 검사한다. |
| 8 | line box 높이·font metric 변화가 item frame에 숨는가 | font-size와 line-height 10px, normal style·weight를 검증한다. Rust runtime test는 item height 10px을 단언한다. |
| 9 | grid 폭·높이가 viewport나 기기 크기에 따라 달라지는가 | fixture source와 adapter가 100×30 CSS px의 고정 Grid dimensions를 사용한다. 실기기 해상도는 CSS scene 크기로 대체하지 않는다. |
| 10 | gap·track 수·track sizing function이 바뀌어도 같은 기대를 적용하는가 | 두 개의 `minmax(0,1fr)` column, 4px gap, 1개 auto row를 typed style에서 검증한다. |
| 11 | track 총 너비 산술이 grid 폭을 넘거나 gap을 중복하는가 | 독립 Chrome oracle과 Rust 최종 stage가 각각 `[48,48]`을 기대한다. `48 + 4 + 48 = 100`이다. |
| 12 | 200px min-content가 item/grid 폭을 강제로 키우는가 | text intrinsic min/max 200과 item frame 100×10을 각각 검사한다. track도 48/48이다. |
| 13 | row axis 결과가 column 성공에 가려지는가 | Chrome oracle은 row 10px이며, 앱 frame은 item height 10px이다. 이 slice는 row sizing 일반 행렬이라고 주장하지 않는다. |
| 14 | item placement가 한 track만 span하거나 implicit column을 추가하는가 | computed placement는 column line 1에서 span 2, row는 auto로 고정한다. 두 final track만 검사한다. |
| 15 | margin/padding/border가 contribution에 끼어드는가 | root/item spacing zero 및 border default를 runtime에서 검사하고 source와 단위 fixture가 동일하게 고정한다. |
| 16 | 방향/쓰기 모드가 바뀌어 x/y 해석이 달라지는가 | item LTR·horizontal-tb를 확인하며 root/item frame x와 크기를 별도로 단언한다. root의 일반 writing-mode 행렬은 지원 범위 밖이다. |
| 17 | unsupported profile에서 조용히 기본 layout으로 빠지는가 | 새 adapter의 profile mismatch와 geometry mismatch는 `RuntimeLayoutFailure`를 반환한다. 성공 결과로 위장하는 fallback은 없다. |
| 18 | 앱 source, Rust 검사, Android/iOS 기대값이 서로 다른 장면을 가리키는가 | ID와 CSS 입력을 대조하고, 실기기와 Simulator 결과에서 root/item frame 및 48/48 tracks를 확인했다. |
| 19 | 기기 로그의 성공이 stale APK에서 나왔는가 | 첫 실행은 old APK와 allowlist 오류를 기록했다. 새 APK를 명시 빌드하고 local/install SHA-256 일치 후 다시 성공했다. |
| 20 | present 신호를 glyph/overflow pixel 정확성으로 과장하는가 | Android screenshot은 box scene이고 runtime log는 draw/present만 증명한다. 글리프 rasterization, 200px 텍스트 픽셀 overflow와 compositor-after 결과는 미검증으로 문서화했다. |

## 결론

검토에서 수정이 필요한 track 계산 오류는 발견하지 않았다. 실기기 첫 캡처의 stale APK는 최신 fixture APK를 빌드·설치해 재실행했고 hash 증거를 남겼다. 별도 surface pixel 또는 glyph paint gate가 없는 한계는 성공 범위에서 제외했다.
