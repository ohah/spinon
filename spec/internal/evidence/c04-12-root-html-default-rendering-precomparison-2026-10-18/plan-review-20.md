# C04.12 기본 root HTML 렌더링 계획 · 독립 실패 관점 20개

구현 전에 계획의 실제 위험 경로를 서로 다른 20개 관점으로 검토했다. 이 기록은 코드 구현 review나 PR diff review를 대신하지 않는다.

| # | 적대적 검토 관점 | 계획에 둔 실패 방지 기준 |
| --- | --- | --- |
| 1 | Rust `RuntimeGpuHost`에 생성자를 추가해도 앱은 계속 기존 `new()`만 호출할 수 있다. | Android/iOS의 일반 launch path까지 새 root HTML host를 호출해야 완료로 판정한다. |
| 2 | root HTML host가 source를 다시 parse해 V8 document와 GPU document가 갈라질 수 있다. | actor-owned parsed HostDocument 하나를 cascade·layout·paint가 함께 사용한다. |
| 3 | FFI source 포인터가 해제된 뒤 비동기 parser가 읽거나 빈/과대 길이가 overflow할 수 있다. | 입력 포인터·길이·빈 입력을 검증하고 parsed document가 호출자 buffer 수명에 의존하지 않게 한다. |
| 4 | 화면 resize 뒤 이전 layout 또는 이전 media style을 가진 scene이 게시될 수 있다. | document/style/environment revision, viewport, page-scale과 font engine token을 한 결과에 묶고 stale snapshot을 거부한다. |
| 5 | `<meta name=color-scheme>`, CSS `color-scheme`, OS scheme의 선택 순서가 기존 C04.12와 달라질 수 있다. | 기존 C04.12 precedence를 유지하고 light/dark 각각의 실제 body·Canvas 결과를 비교한다. |
| 6 | Parley의 `system` feature가 mobile 대상에서 비활성화되어 빈 registry가 될 수 있다. | 잠금된 Fontique Android XML/CoreText backend를 Android/iOS target build와 simulator 실행으로 확인한다. |
| 7 | `system-ui, sans-serif`에서 첫 family가 없을 때 순서나 generic fallback이 CSS와 다를 수 있다. | family list 순서 및 generic family resolution을 보존하고 resolved family를 보고서에 기록한다. |
| 8 | OS font set이 engine 구성 뒤 바뀌어 layout cache가 다른 face를 참조할 수 있다. | engine token 동안 registry를 immutable하게 두며 reload가 추가되면 registry revision을 교체한다. |
| 9 | CoreText/system font lookup이 actor의 background thread에서 안전하지 않을 수 있다. | 실제 session worker thread에서 초기화와 반복 shaping을 확인하고 호출 가능한 thread 계약을 기록한다. |
| 10 | text node가 element computed style을 올바르게 상속하지 않고 UA/browser default로 그려질 수 있다. | Stylo snapshot의 text parent 및 inline ancestor를 사용해 가족·크기·색을 run별로 투영한다. |
| 11 | `font-weight:700` 또는 italic이 fake style로 합성되어 Chrome과 달라질 수 있다. | 선택된 face/weight/style를 기록하고 지원 face가 없으면 조용히 합성하지 않고 거부한다. |
| 12 | `line-height:normal`을 `font-size × 상수`로 근사해 baseline과 줄 높이가 어긋날 수 있다. | resolved face의 실제 ascent/descent/line metrics를 사용하고 명시적 line-height 확장은 범위 밖으로 둔다. |
| 13 | Taffy measure와 final shape가 다른 available width를 사용해 줄바꿈이 달라질 수 있다. | 같은 C15 layout snapshot의 measure·glyph·line box·paint를 쓰고 source width/revision을 검증한다. |
| 14 | Unicode 공백, newline, collapsible whitespace 처리로 빈 줄이나 문자 손실이 생길 수 있다. | C15 white-space/word-break 고정 acceptance fixtures와 source-order 결과를 회귀 기준으로 유지한다. |
| 15 | inline과 Block 자식이 섞인 곳에서 anonymous paragraph가 분리·합쳐져 source order가 바뀔 수 있다. | C15.2b anonymous block flow와 mixed-baseline fixture를 root HTML path에서 재사용한다. |
| 16 | `display:none` subtree의 text가 layout·paint 또는 hit-test에 남을 수 있다. | 같은 computed display snapshot에서 text 제외를 하고 hidden control을 Chrome 및 runtime fixture에 둔다. |
| 17 | UTF-16 source offsets와 UTF-8 shaping spans가 emoji/combining text에서 다를 수 있다. | C15 source mapping을 보존하고 surrogate pair/combining regressions를 default runtime adapter까지 실행한다. |
| 18 | inline run 색상 전환에서 glyph가 빠지거나 paint order가 원래 순서와 달라질 수 있다. | text source별 computed color와 source ranges를 보존하고 common scene order를 확인한다. |
| 19 | owner `overflow`, scroll ancestor, DPR/page-scale 중 하나가 text에 중복 적용되거나 누락될 수 있다. | 기존 owner clip·scroll transform·page-scale 계약을 동일 scene에 연결하고 경계 바깥 표본을 확인한다. |
| 20 | cascade/layout/raster 중간 실패가 box만 그린 부분 scene으로 나타날 수 있다. | failure는 전체 scene publication을 막고, Android PixelCopy/iOS Metal readback 성공·오류를 각각 확인한다. |

검토 중 발견한 계획 보완은 표의 1–12, 18–20번에 반영했다. 아직 구현이 없어 런타임 결함이 수정되었다고 주장하지 않는다.
