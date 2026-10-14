# C13.1 full viewport hit identity 계획 실패 관점 검토 · 2026-10-14

계획 대상은 기존 13개 valid C13.1 fixture row의 integer hit identity를 target frame 전체에서 viewport 전체로 넓히는 host Rust verification slice다. 이 검토는 browser oracle capture와 implementation edits 전에 실행 모델·identity projection·negative controls를 확인했다.

| # | 공격 관점 | 판정과 근거 |
|---:|---|---|
| 1 | 전체 viewport 수가 viewport 한 줄이나 target bounds로 축소될 수 있다 | PASS — 좌표 범위는 `x=0..1279`, `y=0..1099`로 고정했고 행당 1,408,000점이다. |
| 2 | 13개 fixture row 대신 invalid/float 행을 섞어 fail-closed 결과와 성공 scene을 혼동할 수 있다 | PASS — 성공 profile 13개를 명시하고 invalid margin/float 3행을 제외하며 원인은 별도 분류했다. |
| 3 | Chrome scroll position이 0이 아니어서 좌표계가 Rust viewport와 어긋날 수 있다 | PASS — capture 전 `scrollTo(0,0)` 및 `scrollX/scrollY == 0`을 확인한다. |
| 4 | DPR scaling이 CSS 좌표와 device pixel을 혼합할 수 있다 | PASS — DPR 1 고정 CSS integer query를 쓰고 device pixel sample은 제외한다. |
| 5 | 브라우저나 fixture가 달라진 observation을 기준 hash 없이 고정할 수 있다 | PASS — Chrome revision, viewport, fixture SHA-256과 기존 oracle 연결을 고정했다. |
| 6 | 전체 page의 다른 row target이 현재 case의 target으로 잘못 label될 수 있다 | PASS — current case section과 target ancestor identity를 scoped lookup하며 다른 row/page element는 root bucket에 넣는다. |
| 7 | 현재 case의 target descendant가 별도 tag/class이면 unknown hit로 빠지거나 parent box로 분류될 수 있다 | PASS — target과 모든 descendant를 target identity로 접고 미지의 DOM class는 current case owner에 따라 제한한다. |
| 8 | nested clip inner가 clip parent와 같은 NodeId로 접혀 nested ancestry hit가 사라질 수 있다 | PASS — current row의 `.nested-inner`는 target보다 앞서 별도 `inner` bucket으로 보존한다. |
| 9 | positioned escape target의 DOM parent와 containing block 차이를 놓칠 수 있다 | PASS — `positioned-escape`를 유효 행으로 유지하고 frame 밖 target 표본을 exact 비교한다. |
| 10 | viewport 바깥 좌표를 섞어 기존 4점 viewport-null control을 중복 주장할 수 있다 | PASS — 이 계획은 0..width-1, 0..height-1 내부 grid만 scan하며 바깥 점은 다루지 않는다. |
| 11 | 우측/하단 exclusive 경계를 inclusive로 처리할 수 있다 | PASS — 마지막 정수 표본은 1279와 1099이며 바깥 경계 1280/1100은 scan에서 제외한다. |
| 12 | RLE row에 gap/overlap이 있어 표본 수만 맞을 수 있다 | PASS — validator와 Rust test가 첫 x, 연속 run, 마지막 x, y 연속성과 폭을 각각 검사한다. |
| 13 | 13개 행 총합 계산이 틀려 일부 행을 빼먹을 수 있다 | PASS — expected count는 `1280 × 1100 × 13 = 18,304,000`으로 고정한다. |
| 14 | 큰 JSON 생성이 메모리/출력 한계를 넘어 부분 observation을 완전본으로 쓸 수 있다 | PASS — 픽셀별 JSON 대신 연속 identity를 행별 RLE로 저장하고 포인트 수와 case count를 검증한다. |
| 15 | Rust scene root가 Chrome viewport보다 작아 fallback identity 비교가 무효가 될 수 있다 | PASS — 모든 비교 scene의 root element와 viewport height를 1280×1100으로 맞춘다. |
| 16 | 원래 target frame 안 187,320점을 새 full grid로 대체해 기존 검증이 회귀할 수 있다 | PASS — 이전 target-frame observation과 검사는 유지하고 전체 grid 관측을 추가한다. |
| 17 | unrelated DOM/page element를 실제 node identity 지원으로 과장할 수 있다 | PASS — 이 slice는 현재 row owner identity에만 관심이 있고 다른 fixture element는 synthetic viewport root로 추상화함을 명시했다. |
| 18 | hit identity scan을 paint surface 또는 DOM event test로 과장할 수 있다 | PASS — 계획에서 host `hit_test_css_point`만 주장하며 PixelCopy/Metal, dispatch와 WPT를 제외했다. |
| 19 | helper 설정 변경이 layout/clip geometry를 바꾸면서 hit scan 차이를 숨길 수 있다 | PASS — root height만 633에서 1100으로 맞추고 absolute child geometry와 CSS clip inputs를 고정한다. |
| 20 | 큰 표본 수를 C13.1/C13 완료 근거로 과장할 수 있다 | PASS — STATUS와 계획은 C13.1 및 C13 미완료를 유지하고 나머지 surface/event/WPT gate를 명시한다. |

20개 계획 실패 관점을 검토했다. 명시적 수정은 필요하지 않았고 unresolved implementation issues는 아직 capture/runtime test를 수행하지 않아 판단 대상으로 삼지 않았다. browser oracle capture와 구현 검토는 다음 단계다.
