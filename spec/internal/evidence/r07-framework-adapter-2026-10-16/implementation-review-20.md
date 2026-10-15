# R07 Vue·Svelte 어댑터 구현·실행 적대 검토 20회

대상: `spikes/framework-adapters/`, 사전 oracle, 캡처된 실행 trace 및 두 개의 기존 Rust ownership 경계 테스트. 계획 검토와 PR delta 검토를 재사용하지 않는다.

| # | 실패 관점 | 확인 결과 및 조치 |
|---:|---|---|
| 1 | 기대 tree가 adapter 출력에서 생성됐는가 | oracle은 실행 전에 별도 JSON으로 고정했고 네 단계의 정확한 구조를 양쪽 결과와 비교했다. |
| 2 | Vue 실제 custom renderer 대신 Vue DOM 경로를 사용했는가 | `@vue/runtime-core` `createRenderer`의 host callbacks로 실행했다. |
| 3 | Svelte SSR만 테스트해 native host update를 건너뛰었는가 | `generate: "client"`로 compiler output을 실행했고 DOM-shaped facade를 통과했다. |
| 4 | Node의 server conditional export가 성공 근거로 섞였는가 | 첫 실행에서 `lifecycle_function_unavailable`를 재현했다. 테스트 스크립트를 browser condition으로 고정해 해결했다. |
| 5 | Svelte module이 spike 밖 임시 경로에서 dependency를 찾지 못하는가 | OS temp 경로 실패 후 spike 내부 임시 디렉터리로 옮겼고 unmount 뒤 삭제함을 확인했다. |
| 6 | 두 번의 click이 callback 누적으로 2회 이상 증가하는가 | Vue listener 교체, Svelte delegation 각각 0→1→2를 oracle과 대조했다. |
| 7 | Svelte root와 document listener 경합으로 callback이 중복되는가 | 각각의 root/document 경로가 synthetic dispatch 1회당 한 번만 업데이트했고 unmount 뒤 listener 수 0이다. |
| 8 | unmount가 HostRoot에 노드를 남기는가 | 두 adapter 모두 unmount projection이 `children: []`다. |
| 9 | detached 노드가 계속 사용자 입력을 받는가 | harness는 detached target을 `DetachedTarget`으로 거부한다. 실제 OS input/제품 dispatch는 주장하지 않는다. |
| 10 | detached framework closure가 회수됐다고 과장하는가 | closure 수명과 GC는 미확정으로 README 한계에 기록했다. |
| 11 | Svelte static markup parser가 다른 템플릿 node를 조용히 잃는가 | fixture는 parse5로 parse하며 element/text만 materialize한다. 예상 밖 template node는 오류를 낸다. 추가 문법은 검증하지 않았다. |
| 12 | fragment/comment bookkeeping이 제품 tree에 노출되는가 | 내부 comment와 anchor는 projection에서 제외한다. fixture에는 fragment/list 시나리오가 없고 비목표로 유지한다. |
| 13 | 알 수 없는 속성이 정상 지원처럼 통과하는가 | 문자열 `aria-label` 이외의 prop/attribute는 오류; 오류 전후 속성 map과 operation 길이가 같다. |
| 14 | 이름/namespace 오류가 tree를 부분 수정하는가 | `svg:path` 이름은 `InvalidNodeName`; 변이 전 operation 길이가 보존된다. |
| 15 | OwnerId가 빠지거나 다른 owner로 바뀌는가 | 17개 operation 각각에 framework별 token이 있다. 교차 owner insert는 변이 없이 `OwnerConflict`; 같은 조건의 Rust core tests도 두 건 통과했다. JS recorder는 Rust ABI가 아니다. |
| 16 | text/속성/mount operation이 core surface와 동떨어졌는가 | 사용된 구조 operation을 Rust `DocumentOperation` 정의와 대조했다. event 계측 항목은 별도 이름으로 구분했다. |
| 17 | 계측 이름이 React 전용 API와 혼동되는가 | `SyntheticEventDispatch` 표기를 발견해 `HarnessClickDispatch`로 변경했다. HostDocument core 및 spike에서 React 전용 type/dependency 검색 결과는 없다. |
| 18 | lock이 없거나 설치 시 다른 framework가 선택되는가 | 정확한 버전과 integrity가 들어 있는 lockfile로 `bun install --frozen-lockfile` 성공을 확인했다. |
| 19 | 전역 DOM shim이나 생성물이 다음 실행에 누출되는가 | 이전 global descriptor를 복원하고 임시 compiler 폴더를 지운다. 반복 실행 뒤 임시 폴더가 남지 않았다. |
| 20 | 조사 결과가 제품 호환·모바일 화면·R03 완료로 확대되는가 | 결과는 host 가능성에 한정했다. FFI, CSS/GPU, 접근성, Simulator, React 동등성, R03은 명시적 미완료다. |

## 판정

20개 서로 다른 실패 관점을 실제 코드·실행 기록·계약 정의에 대조했다. 발견한 하니스 경로·조건 export·오해 가능한 계측 이름·파일 책임 크기를 수정했다. 남은 한계는 기능 성공으로 채우지 않고 scope에 고정했다.
