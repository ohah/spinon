# C11.1 Root HTML anonymous Grid text 계획 검토 · 20개 독립 실패 관점

검토 대상은 [제한 구현 계획](./c11-root-html-grid-anonymous-text.md), 고정 fixture와 Chrome 156 사전 비교다. 이 검토는 구현 검증이 아니다. 아래 항목은 계획에서 정한 구현·검증 경계의 적절성을 점검한다.

| # | 실패 관점 | 계획의 대응과 판정 |
|---:|---|---|
| 1 | 오래된 Grid 초안이나 Chrome 동작을 규범 기준으로 삼아 anonymous item 생성 규칙을 놓치는가 | 고정된 W3C 2025 CRD §6을 규범 기준으로 두고 Chrome 156을 독립 관측값으로 한정했다. PASS. |
| 2 | 공백-only text가 항상 사라진다고 가정해 white-space pre의 보존 공백까지 제거하는가 | whitespace-only 성공 대조는 computed white-space normal에 한정하고 보존 공백은 거부 범위로 둔다. PASS. |
| 3 | HostDocument가 인접 텍스트를 여러 NodeId로 나눠 같은 text sequence를 잃는가 | 같은 anonymous run으로 모으되 모든 원본 NodeId provenance를 유지하도록 한다. 구체 grouping 규칙은 구현 tests에서 고정한다. PASS. |
| 4 | block child 앞뒤 텍스트를 하나의 item으로 합쳐 source-order placement가 달라지는가 | text와 element 혼합, element separator, 복수 run은 이 slice에서 fail-closed다. PASS. |
| 5 | 단일-item 첫 cell의 결과를 전체 anonymous item auto-placement 지원으로 확대하는가 | 하나의 run·기본 row flow만 성공시킨다. general auto-placement 주장을 제외한다. PASS. |
| 6 | fixed tracks 합계와 root content box가 달라 stretch 기대 frame이 틀리는가 | root 120×70, columns [40,80], row [70], gap 0 및 zero edges를 고정하고 합계 기반 독립 frame을 기록한다. PASS. |
| 7 | white-space normal과 pre에서 텍스트 item 존재 여부를 같은 규칙으로 처리하는가 | normal whitespace-only control만 빈 item으로 판정하며 preserved values는 지원 밖으로 둔다. PASS. |
| 8 | anonymous item은 unstyleable인데 별도 element의 style을 찾아 적용하는가 | computed inherited values는 Grid owner의 같은 style snapshot에서만 얻도록 계획한다. PASS. |
| 9 | root color·font가 paragraph paint에 전달되지 않아 layout frame은 맞지만 글자가 안 보이는가 | white-space normal, monospace 16px/20px, color #e5484d와 기본 alignment의 정확한 inherited tuple을 계획과 precomparison fixture에 고정했다. paragraph-to-renderer 전달은 구현/runtime 검증 대상으로 남긴다. PASS. |
| 10 | text node가 아닌 synthetic NodeId를 만들어 DOM identity·event mapping을 오염시키는가 | 원래 text NodeId를 stable anchor로 사용하고 synthetic DOM mutation을 금지한다. PASS. |
| 11 | auto-placement를 위해 HostDocument child vector를 재정렬해 accessibility/source order를 바꾸는가 | source tree와 child order를 보존하도록 명시한다. PASS. |
| 12 | Chrome의 glyph advance를 모든 플랫폼의 측정값으로 강제해 정상 fallback 차이를 오류로 오판하는가 | Chrome Range width는 관측값이고 모바일 geometry/paint gate와 분리한다. PASS. |
| 13 | text가 40px track을 넘거나 여러 line으로 감기는데 단일 Range 기준을 성공 처리하는가 | 한 줄 기준을 고정하고 line wrapping 및 font variation은 제한 범위에서 거부한다. PASS. |
| 14 | root/style/document revision 불일치가 기존 text paragraph를 재사용하게 하는가 | text anchor와 layout item을 같은 document/style/layout revision에 결합하고 stale input을 실패로 둔다. PASS. |
| 15 | 잘못된 UTF-16 또는 중복 NodeId가 일부 text만 그려지는 성공으로 바뀌는가 | encoding·source uniqueness 오류와 partial paragraph 수집을 publication 전에 거부하도록 한다. PASS. |
| 16 | 새 C11.1 route를 만들 때 C11.4 fixture gate를 무조건 켜 일반 실행에 test payload가 새는가 | C11.4 gate를 단순 활성화하지 않고 default/opt-in archive 경계를 별도 검사한다. PASS. |
| 17 | whitespace-only run에서 layout item은 없지만 renderer가 빈 box나 stale draw를 제출하는가 | item 0·Range 0·explicit tracks 유지와 성공/거부 publication을 runtime에서 따로 검사한다. PASS. |
| 18 | 실패 후 직전 successful scene이 새 실패 입력의 결과로 남는가 | failure atomicity, partial draw 부재와 기존 scene/cache boundary를 실행 계획에 넣었다. PASS. |
| 19 | Android AVD 또는 iOS Simulator 캡처가 실기기·hardware GPU·compositor 픽셀 검증으로 과장되는가 | AVD/Simulator 환경, draw 단계, 표면 readback 여부와 주장 한계를 분리해 기록하도록 했다. PASS. |
| 20 | 단일 text scene을 C11.1·C11.4·C11 전체나 WPT/CSS 공개 지원 완료로 표시하는가 | 계획·STATUS·roadmap 단계 모두 상위 checkbox 및 공식 72/213을 유지하도록 명시했다. PASS. |

### 검토 결과와 선행 조건

계획은 적대적 검토 뒤에도 제한된 지원 범위를 유지한다. 구현 시작 전 exact computed text tuple을 precomparison fixture와 대조해 9번 실패 관점의 구현 선행 조건을 닫는다. 구현 중 review에서 plan과 다른 실패 관점 20개를 새로 만들며, 이 계획 표를 재사용하지 않는다.
