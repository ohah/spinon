# C11.2.1 모바일 runtime 행렬 계획 검토 · 2026-10-12

**검토 대상:** [모바일 10-scene runtime 행렬 계획](../../../plan/c11-2-1-mobile-runtime-matrix.md)  
**기준 자료:** C11.2.1 inventory, HTML fixture, pinned Chrome 154 reference, 현재 Android/iOS C11.2.1 실행 경로  
**판정:** 아래 20개 실패 경로를 계획에 반영한 뒤 구현에 착수한다. 이 검토는 실행 결과나 기능 완료를 뜻하지 않는다.

| # | 공격 관점 | 발견 위험 | 계획에 반영한 방어·구현 시 확인 |
|---:|---|---|---|
| 1 | C11.1과 C11.2.1 scene 인수가 서로 바뀜 | 두 경로가 비슷한 intent와 log prefix를 사용한다. 잘못 라우팅하면 C11.1 결과를 placement 결과로 오인한다. | 별도 C11.2.1 scene ID allowlist와 고유 log prefix를 사용하고 scene ID 누락을 거부한다. |
| 2 | 임의 scene ID 또는 문자열이 native 경계에 전달됨 | C ABI에 자유로운 JavaScript/CSS를 넘기면 fixture runner가 임의 코드 실행기가 된다. | native에는 길이 제한된 scene ID만 전달하고, Rust에서 고정 10개 scene의 포함 source를 선택한다. 알 수 없는 ID·NUL·빈 ID는 거부한다. |
| 3 | inventory, HTML, Chrome 기준 버전이 서로 다름 | 예전 fixture와 새 실행을 합쳐도 scene 수만 맞으면 잘못 통과할 수 있다. | generator가 세 입력의 SHA-256을 고정하고, HTML scene ID·순서·root·node preorder를 inventory 및 DPR 1 reference와 대조한다. `--check`는 생성물 drift를 실패시킨다. |
| 4 | 중복 ID·중복 node·누락 node가 합계만 맞춤 | 같은 node를 두 번 세거나 다른 scene의 node를 재사용할 수 있다. | case별 ID uniqueness, 전역 fixture ID uniqueness, parent preorder, node 집합 equality를 검사한다. |
| 5 | HTML fixture의 `display:none` wrapper나 scene CSS가 runtime을 바꿈 | 웹 fixture의 active 상태를 그대로 따라가면 장면이 숨겨지거나 wrapper가 지원 profile 밖 배치를 바꾼다. | 검증된 scene의 node 구조와 inline declarations만 추출한다. 제한된 paint class 색상은 허용 palette에서만 inline style로 합성하며 임의 stylesheet cascade는 실행하지 않는다. |
| 6 | element parent 관계를 바꾸거나 wrapper를 빠뜨림 | flat tree로 만들면 nested Grid 자동 배치와 중첩 explicit 배치가 사라진다. | parent ID를 보존하고 wrapper를 별도 고정 node로 만든다. HTML/reference parent graph와 preorder가 다르면 생성 실패다. |
| 7 | CSS/ID 문자열이 JavaScript source를 탈출 | quote, backslash, 줄바꿈이 있는 fixture 값이 source injection이나 다른 node 생성을 일으킬 수 있다. | 모든 문자열 literal은 JSON string escaping으로 직렬화하고, 입력 selector/property/value를 고정 fixture grammar에 제한한다. 생성 source를 고정 최대 크기와 syntax 검증으로 검사한다. |
| 8 | 과도한 fixture source가 앱 메모리·C ABI buffer를 넘김 | 최대 case 크기와 결과 크기 불일치가 buffer truncation을 만들 수 있다. | scene별 source 64 KiB 상한, output capacity 오류, 실제 node/frame 수 상한을 두고 truncated UTF-8/status를 성공으로 해석하지 않는다. |
| 9 | 네 거부 scene이 다른 이유로 실패해 기대 거부로 오인 | parser error, timeout, feature disabled가 unsupported CSS 진단처럼 보일 수 있다. | 세 placement 거부는 code·NodeId·property를 고정한다. named-line scene은 computed `grid-template-columns` 변환 단계에서 `cascade_failed`, owner/property `None` 및 상세 property를 따로 고정한다. crash, disabled marker, timeout, 빈 report는 거부 통과가 아니라 runner 실패다. |
| 10 | 잘못된 node가 기대한 node보다 먼저 실패 | tree preorder나 declaration order 변화가 진단 소유자를 바꿀 수 있다. | layout 거부는 정확한 NodeId→fixture ID mapping·code·property를 비교한다. cascade 거부는 structured owner가 `None`임을 확인하고 detail의 property·CSS value를 고정한다. 각 scene의 source hash가 바뀌면 oracle 재생성을 요구한다. |
| 11 | 거부 전에 partial layout/frame/render snapshot이 남음 | 오류가 난 scene도 이전/부분 결과를 내면 화면에 성공처럼 보일 수 있다. | 새 process/host에서 거부 case를 실행하고 성공 frame·draw·present marker가 없어야 한다. failure report의 diagnostic 이외에 partial result를 성공으로 집계하지 않는다. |
| 12 | 이전 scene의 화면이나 revision이 현재 scene의 증거로 재사용 | 같은 app process 내 장면 교체는 이전 frame·surface generation·style cache를 남길 수 있다. | scene마다 새 앱 process와 새 HostDocument를 만든다. 이전 frame은 새로운 scene 성공을 대신할 수 없게 run ID와 PID를 연결한다. |
| 13 | wrapper와 fixture node의 NodeId offset 오류 | wrapper 때문에 모든 expected frame 번호가 한 칸 밀리거나 root가 빠질 수 있다. | wrapper frame은 fixture frame과 분리 검증하고, inventory preorder 순으로 fixture root부터 node ID·rect를 대응한다. wrapper는 `(0,0,360,800)` 고정값을 별도로 확인한다. |
| 14 | DPR·surface pixel 좌표와 CSS geometry 단위 혼용 | Chrome DPR 1 reference와 device pixel frame을 직접 비교하면 잘못된 크기 변환을 숨긴다. | geometry는 CSS px만 비교하고 viewport는 360×800 CSS px로 고정한다. surface screenshot은 시각 근거에만 사용하며 pixel parity로 주장하지 않는다. |
| 15 | 제출 marker를 실제 draw/present 완료로 과장 | host eval 성공이나 queued draw만으로 surface 제출이 보장되지 않는다. | layout-ready와 WGPU draw/present marker를 별도로 요구하고, marker의 box count·scene/run/PID가 일치해야 한다. OS 화면 표시 완료·프레임 지연은 이 행렬의 주장 밖이다. |
| 16 | 겹침 scene의 source paint order가 geometry만으로 빠짐 | 두 겹친 item frame은 같으므로 source-order 오류가 frame 비교에서 드러나지 않는다. | `overlap-source-order`의 paint snapshot 순서와 WGPU scene item order를 별도로 기록·비교한다. 화면 캡처만으로 순서를 판정하지 않는다. |
| 17 | 중첩 Grid 장면이 outer 결과만 검증 | `nested-placement-modes`에서 안쪽 automatic/explicit 배치 오류가 외부 box 수만으로 감춰진다. | 7개 fixture node 각각의 frame과 parent graph를 비교하고 auto child 두 개와 explicit child 두 개의 위치를 따로 assertion한다. |
| 18 | iOS launch argument가 다른 probe로 흡수되거나 run ID가 누락 | 기존 boolean argument는 scene별 선택을 표현하지 못하며 simulator unified log에 오래된 결과가 남는다. | `--spinon-c1121-grid-scene=<fixed-id>`와 run ID를 분리 파싱하고, 실행 PID·scene ID·run ID 3개가 모두 맞는 marker만 수집한다. |
| 19 | Android intent/log 수집이 이전 logcat 또는 다른 PID를 읽음 | global logcat clear는 사용자 log를 파괴하고, stale PID output은 false pass를 만든다. | 새 process PID·run ID·scene ID를 모두 일치시켜 필터링한다. `logcat -c`를 쓰지 않으며 PID가 없거나 재사용 판정이 모호하면 실패한다. |
| 20 | Simulator·WPT·screenshot 결과를 제품 적합성으로 확대 | AVD software GPU, Simulator screenshot, Chrome에서 통과한 WPT는 Spinon/WPT 제품 지원의 증거가 아니다. | Android AVD와 iOS Simulator를 별도 기록한다. Chrome WPT 후보는 oracle 결과로만 표기하고, screenshots는 실행 여부 확인에만 쓴다. 실기기·전체 WPT·전체 CSS Grid 완료는 별도 gate로 둔다. |

## 계획 보정

- 실패 case는 `status=error` 하나만 보지 않는다. placement 거부는 code·node·property를, named-line cascade 거부는 code·owner 부재·상세 property를 각각 비교한다.
- 실패 case에는 `drawn`/`presented` 성공 marker가 없어야 하고 runner timeout·crash·fixture 비활성은 실패로 분리한다.
- 성공 case의 wrapper frame과 fixture frame을 나누고, overlap paint order와 nested placement를 geometry와 별도 assertion한다.
- 각 실행 결과는 `run_id + scene_id + process_id`로 귀속한다. logcat 전체 비우기는 금지한다.
- matrix generator의 문자열 escaping·source 상한·고정 scene allowlist를 자동 검사한다.

## 재검토 결론

20개 공격 관점의 계획상 방어를 명시했다. 최초 Android runtime 실행에서 named-line scene은 placement property가 아니라 Grid track template cascade 단계에서 거부됨을 확인해 위 diagnostic contract를 바로잡았다. 이 결과를 기능 통과나 지원으로 간주하지 않는다. 나머지 scene의 고정 ID와 오류 소유권은 실행 로그로 계속 확인한다.
