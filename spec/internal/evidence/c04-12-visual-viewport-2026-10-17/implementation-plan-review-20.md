# C04.12 visual viewport 계획 적대 검토

계획 문서를 구현하기 전에 서로 다른 실패 경계를 대조했다. 발견 사항은 계획의 범위·수식·판정 기준에 반영했다.

| # | 공격 관점 | 검토와 계획상 처리 |
| ---: | --- | --- |
| 1 | layout viewport와 visual viewport를 같은 크기로 계속 취급 | 계획에서 두 값을 별도 필드와 별도 공식으로 정의했다. |
| 2 | media query가 visual viewport를 읽는 것으로 잘못 연결 | width/height media 입력은 layout 크기를 사용한다고 고정했다. |
| 3 | 고정 width가 화면보다 넓은 경우 layout height 계산 오류 | Chrome `500×1082` 관찰을 사전 기준에 추가하고 aspect ratio 공식으로 수정한다. |
| 4 | scale 2에서 visual height를 layout height로 사용 | Chrome layout `844`, visual `422`를 별도 fixture 기준으로 넣었다. |
| 5 | scale 0.5가 width/height 두 축에서 확장되는지 확인 누락 | Chrome `780×1688` layout 및 visual 값을 비교한다. |
| 6 | 고정 width 500 scale 1에서 화면 밖 box가 viewport와 함께 확장 | layout은 500×1082, visual은 390×844로 분리한다. renderer transform 자체는 범위 밖이라고 명시한다. |
| 7 | CSS layout 계산에 DPR을 곱해 넣는 회귀 | screen CSS는 surface/DPR로만 계산하고 DPR 필드를 유지한다. |
| 8 | page scale을 DPR에 흡수해 double-scale | page scale과 DPR을 별도 반환하고 곱한 값을 저장하지 않는다. |
| 9 | width 기본값, device-width, 폭 확장 규칙 변경 | metadata allowlist와 width 수식을 유지하고 기존 tests를 회귀 기준으로 둔다. |
| 10 | 폭/높이 aspect ratio에서 physical surface와 CSS screen 혼합 | layout height는 dimensionless surface ratio를 resolved CSS layout width에 적용한다. |
| 11 | 0 또는 NaN surface dimensions로 나눗셈 | 기존 surface validation 다음에만 계산하고 산술 결과 유효성을 다시 검사한다. |
| 12 | 극단 aspect ratio에서 visual/layout height overflow | layout 및 visual 결과 모두 finite·positive 검사 후 cascade에 게시한다. |
| 13 | 너무 큰 resolved width에 대한 기존 상한 우회 | 기존 최대 layout width 검증을 그대로 적용한다. |
| 14 | 환경 값이 바뀌어도 cascade revision/cache가 재사용 | layout width/height가 기존 viewport key에 남아 height 변화는 환경 revision으로 재계산된다. |
| 15 | color-scheme 또는 pointer media 입력이 viewport 수정 중 사라짐 | 기존 media environment 복사를 유지하고 관련 테스트를 회귀 대상으로 둔다. |
| 16 | FFI의 RootHtmlEnvironment와 Canvas nested copy 간 값 불일치 | 두 생성 경로 모두 같은 Rust snapshot의 visual fields를 쓴다. |
| 17 | C/Rust struct field order와 padding 불일치 | C header와 Rust `repr(C)`를 같은 순서로 고치고 native simulator build에서 ABI 접근을 확인한다. |
| 18 | iOS bridge만 새 fields를 출력하고 Android는 누락 | 양쪽 report format과 simulator assertion을 같은 fixture 값으로 확인한다. |
| 19 | Android density 또는 iOS point scale을 DPR로 혼동 | 양 플랫폼 FFI 입력은 현재 physical surface px와 실제 device scale을 사용한다. simulator log에서 두 입력을 보존한다. |
| 20 | native rotation 후 이전 aspect ratio 재사용 | surface 재제출 때마다 입력 surface 비율로 계산한다. rotation 이벤트 구독은 범위 밖이라 문서화한다. |
| 21 | screenshot의 단순 bbox를 app rendering 근거로 확대 | Chrome oracle 캡처와 app simulator 캡처를 별도 기록하고 서로 대체하지 않는다. |
| 22 | headless emulation의 `innerWidth`를 layout 진실로 오인 | `documentElement.clientWidth/clientHeight` 및 `matchMedia`를 판정값으로 쓴다. |
| 23 | no-meta oracle session의 동기화가 어긋나 잘못된 bbox를 저장 | 불일치한 session의 no-meta pixel 결과를 제외하고 기존 metrics reference만 유지한다. |
| 24 | `visualViewport` offset/scroll이 초기 size 공식에 묵시적으로 포함 | 초기 origin `(0,0)`만 명시하며 pinch/scroll offset은 명시적으로 제외한다. |
| 25 | CSS pixel fractional layout height 반올림 차이 | 계산을 `f32`로 보존하고 oracle 수치 허용치를 0.001 CSS px로 둔다. |
| 26 | Safari 또는 모든 Chrome 버전으로 Chrome 155 관찰 확대 | 고정 빌드와 iPhone 12 emulation 한정으로 기록한다. |
| 27 | desktop meta viewport 동작까지 함께 바꾸는 부작용 | 모바일 root HTML profile만을 대상으로 하며 desktop behavior를 규정하지 않는다. |
| 28 | renderer transform이 아직 없어 사용자에게 완성으로 오해될 위험 | 계획과 C04.12 상태 문구에 renderer·HTML body paint 미지원 경계를 남긴다. |
| 29 | native touch point가 page scale로 자동 변환된다고 오해 | surface-to-CSS input mapping은 별도 계약이며 이 단계에서는 바꾸지 않는다. |
| 30 | text raster가 pageScale에 맞춰 고해상도로 다시 생성된다고 확대 해석 | text rerasterization과 pixel equivalence를 범위 밖으로 둔다. |

## 검토 결과

검토 중 layout height가 `screen_height/page_scale`로 충분하다는 기존 가정이 fixed width 500 및 scale 2 Chrome 관찰과 모순되는 점을 발견했다. 계획을 layout aspect ratio와 visual viewport 분리로 고쳤다. no-meta Agent Browser session은 emulation 상태에 따라 live JS와 screenshot이 불일치하므로 해당 pixel oracle를 제외했다. 위 변경이 계획에 반영되어 구현 기준으로 사용할 수 있다.
