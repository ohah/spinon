# C11.1 모바일 runtime 10-scene 계획 검토 · 2026-10-12

**대상:** [10-scene 교차 실행 계획](../../../plan/c11-1-mobile-runtime-matrix.md) · [C11 계획](../../../plan/c11-css-grid.md) · [0059 내부 계약](../0059-c11-grid-layout.md)

## 독립 실패 관점 20개

| 회차 | 공격 질문 | 계획에서 요구하는 방어·판정 |
| ---: | --- | --- |
| 1 | reference·inventory와 HTML scene 목록이 서로 다른데 일부만 실행해 성공 처리하는가 | 세 입력의 scene ID 집합·node preorder·parent 관계를 비교하고 하나라도 다르면 matrix 생성 실패. |
| 2 | HTML 변환기가 알 수 없는 tag/attribute를 무시해 다른 tree를 생성하는가 | 문서 workspace에 잠긴 `parse5` `7.3.0`을 root devDependency로 명시한다. 대상 fixture subset 외 tag/attribute/class/style와 parser error를 명시 오류로 처리하고 생략 fallback 금지. 이는 mobile runtime dependency가 아니다. |
| 3 | class palette 추출이 실패해 background만 빠지거나 임의 기본색을 쓰는가 | 허용 class와 순수 background-color 규칙을 고정하고 중복·누락·추가 property를 거부. |
| 4 | class 규칙의 paint-only 변환이 layout computed style을 바꾸는가 | 변환 대상은 검증된 `background-color`뿐이며 원래 inline layout declaration hash와 출력 보존을 검사. |
| 5 | 스타일 시트에서 class 선언 순서/중복 선언이 변환기 정규식 밖에서 다른 cascade를 만드는가 | palette 입력의 전체 일치 소비를 요구하고 동일 selector 중복/지원 밖 declaration은 build 실패. |
| 6 | `scene-root` wrapper를 빼면서 margin collapse나 containing block을 바꾸는가 | wrapper를 제외하지 않는다. 원본 absolute declaration은 현재 Grid runtime profile이 cascade 오류로 거부하므로 360×800 fixture adapter wrapper를 사용하고 각 wrapper/data frame을 Chrome과 비교한다. adapter geometry가 전 10 scene에서 맞기 전에는 이 변환을 인정하지 않으며, 이는 일반 CSS Flex/absolute 지원 근거가 아니다. 실기기에서 wrapper 생략 시 `grid-under-block` 부모 x가 10px에서 0px로 달라지는 것을 관측했다. |
| 7 | runtime viewport와 Chrome 기준 viewport 차이가 percentage 또는 `fr` 결과를 바꾸는가 | definite Grid container 축과 media environment를 비교하고 scene별 frame을 oracle 대조; 필요한 환경 값은 명시 설정. |
| 8 | inventory node order와 실제 JS append order가 달라져 NodeId만 우연히 맞는가 | runtime source의 preorder/parent metadata를 출력하고 실제 frame 순서·개수를 case별로 검증. |
| 9 | empty/single item이 없는 node를 생략해 빈 layout을 성공으로 오인하는가 | scene마다 expected node ID 집합·정확한 수를 검사하고 empty scene도 root frame 필수. |
| 10 | 중첩 Grid 안 Flex, Flex 안 Grid를 단일 Grid dispatch로 처리하는가 | 10 scene 중 mixed parent/child를 별도 확인하고 child algorithm의 display별 frame을 비교. |
| 11 | row auto-placement의 다섯 번째 item이 implicit row를 몰래 생성하거나 overflow를 숨기는가 | explicit 2×3 capacity를 유지하고 다섯 item frame을 전부 oracle과 비교; implicit track 수를 결과에 기록. |
| 12 | fixed/percentage/`fr` track 단위가 다른 scene을 놓쳐 단일 fixed scene pass를 matrix pass로 승격하는가 | matrix가 10개 고정 ID 모두를 요구하고 각 case의 computed track property 및 node frame을 비교. |
| 13 | 호출자가 임의 scene 문자열을 통해 임의 JavaScript/fixture 경로를 실행하는가 | Java/Swift allowlist 후 정수 index만 FFI에 전달; FFI도 범위검사 후 static embedded source만 선택. |
| 14 | FFI index가 음수·최대값 초과·잘린 ABI 값일 때 다른 scene으로 wrap되는가 | unsigned ABI 폭과 변환 전후 검증을 고정하고 모든 out-of-range negative control이 host/eval 실패. |
| 15 | 한 HostDocument에서 scene들을 순차 평가해 이전 node/cache/revision이 다음 scene에 남는가 | scene마다 새 앱 process와 host를 사용하고 결과에 process ID, HostRoot, run ID를 기록. |
| 16 | 앱이 시작되지 않았거나 renderer가 준비되지 않았는데 이전 log의 marker로 pass하는가 | 시작 전 force-stop/log reset, 새 run ID·PID·host marker·draw marker를 요구하고 stale marker를 negative control로 거부. |
| 17 | Android/iOS 인자 이름 또는 platform branch가 달라 scene mapping이 어긋나는가 | 같은 canonical scene ID allowlist와 generated index map을 두 platform source에서 비교하는 테스트를 둔다. |
| 18 | Android surface 크기·density 변화가 CSS coordinate를 pixel coordinate처럼 비교하게 하는가 | geometry는 CSS px frame만 비교하고 surface screenshot은 보조 시각 근거로 분리; density를 metadata로 저장. |
| 19 | 한 플랫폼 10/10 결과를 다른 플랫폼 통과로 합치거나 geometry 결과를 pixel exact로 과장하는가 | comparator와 README를 platform별로 분리하고 Android PixelCopy canary 외 pixel pass를 주장하지 않는다. |
| 20 | 모든 자체 fixture가 통과해 C11 전체·WPT·성능까지 완료된 것처럼 로드맵을 체크하는가 | 계획·STATUS·roadmap에 완료 범위를 10-scene runtime geometry로 제한하고 WPT/Metal pixel/전체 Grid 한계를 남긴다. |

## 검토 판정

초기 계획에서 세 가지 모호성을 발견해 수정했다. 첫째, “HTML subset”만 적으면 변환 실패 시 조용한 생략을 막지 못하므로 알 수 없는 tag·attribute·style·class를 거부하도록 계약에 넣었다. 둘째, 정규식 HTML parser는 중첩/attribute 오류를 놓칠 수 있어 이미 lockfile에 있는 `parse5`를 test tooling의 직접 devDependency로만 사용하기로 했다. 셋째, runtime argument가 arbitrary JavaScript나 ID로 이어질 위험이 있어 native 경계에는 allowlist 검증 후 정수 index만 전달하고 source는 static embedding에서 선택하도록 했다. 이 방어가 실제 generator/FFI 및 Android/iOS 실행기 테스트에 반영되기 전에는 구현 완료 판정을 하지 않는다.

나머지 검토 항목은 각각 fixture identity, cascade, layout, resource bounds, freshness, platform parity, evidence scope를 독립적으로 다뤘다. 계획 실행 중 Android 실기기 행렬이 wrapper 생략에 따른 root margin 차이를 찾았고, 원본 absolute style이 Grid profile 진단에 의해 거부되는 점까지 확인해 제한된 adapter 경계를 명시했다. 구현 뒤 새 코드와 raw evidence를 대상으로 별도 20개 검토가 필요하다.
