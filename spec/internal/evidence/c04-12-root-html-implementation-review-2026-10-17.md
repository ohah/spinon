# C04.12 루트 HTML 기반 구현 적대 검토

- **검토일:** 2026-10-17
- **범위:** HTML5 parser·HostDocument 초기 import·V8 document accessor·opt-in 앱 fixture 경계
- **구현 전 계획 검토와 분리:** 계획서의 20개 검토표는 재사용하지 않았다. 아래 항목은 현재 코드와 테스트·simulator 결과를 각각 다른 실패 경로로 다시 확인했다.
- **판정:** 이 제한된 문서 트리 기반은 검증했다. viewport/color-scheme 적용과 기본 앱 asset 경로가 없어서 C04.12 제품 기능의 완료 판정은 불가하다.

| # | 실패 관점 | 확인 결과 |
| --- | --- | --- |
| 1 | C ABI 입력 길이를 속여 parser byte 제한을 우회 | Rust parser가 UTF-8 byte 길이를 검사하고, C ABI는 포인터를 읽기 전에 1 MiB 초과를 거부한다. null 포인터와 최대 초과 길이 negative test 통과. |
| 2 | 비 UTF-8 입력을 lossy 변환해 DOM에 부분 반영 | C ABI가 `from_utf8` 실패에서 세션을 만들지 않는다. `0xff` negative test에서 예상 오류를 확인했다. |
| 3 | null source와 0이 아닌 허용 길이를 전달 | 포인터 검사가 `from_raw_parts`보다 앞선다. empty source도 parser의 document mode/quirks gate에 통과해야 하므로 자동 성공하지 않는다. |
| 4 | output buffer가 짧을 때 반환 포인터가 살아남거나 문자열이 잘림 | report writer가 capacity를 확인하고, 생성 이후 report 쓰기 실패면 session을 drop한다. fixture 출력은 2 KiB로 확보했다. |
| 5 | doctype이 없거나 잘못된 문서의 quirks mode를 정상 HTML로 게시 | `NoQuirks` 이외 모드는 startup 오류다. quirks negative test 통과. |
| 6 | 생략된 `<html>`, `<head>`, `<body>` 또는 잘못 중첩된 태그가 무시됨 | pinned HTML5 tree builder의 자동 삽입·recovery를 사용한다. parser fixture에서 생성 구조와 misnest recovery를 확인했다. |
| 7 | table foster parenting·reparent 과정에서 text가 table subtree에 잘못 남거나 사라짐 | 첫 검토에서 `append_based_on_parent_node`가 존재하는 parent에 append해 text를 table 뒤에 뒀다. html5ever TreeSink 계약에 맞춰 parent가 있으면 element 직전에 삽입하고, 없으면 이전 element의 마지막 child로 append하도록 수정했다. regression fixture에서 foster text가 table 앞에 위치하고 cell subtree가 유지되는 것을 확인했다. |
| 8 | 인접 텍스트 append/insert가 합쳐지며 주변 element 순서를 바꿈 | text node 병합과 sibling 삽입 경계를 포함해 `first`, `<b>`, `third` 순서를 검증했다. |
| 9 | HTML element namespace가 소실되어 SVG/MathML 구조가 혼동됨 | parser의 namespace URI와 local name을 HostDocument `CreateElement`까지 전달하고 root HTML namespace를 확인했다. |
| 10 | batch ABI가 표현하지 못하는 namespaced attribute가 조용히 손실됨 | namespace attribute는 전체 startup 오류로 fail closed한다. `xml:lang` negative test 통과. |
| 11 | 다수 attribute 입력이 계약된 상한보다 많이 import됨 | import tree 전체 attribute 상한 16,384개와 16,385개 negative test를 추가했다. source 1 MiB 상한도 독립 적용된다. |
| 12 | 깊거나 큰 문서가 HostDocument quota보다 많이 수용됨 | source 1 MiB, imported node 65,536, depth 256 budget을 명시하고 초과 source/depth 및 attribute test를 확인했다. |
| 13 | `<template>`의 별도 fragment child가 표시 tree에 섞임 | `<template>` 아래 `span`이 일반 HostDocument 자식이 아니며 다음 `main`만 visible subtree에 남는 테스트를 추가했다. DOM `template.content` API 지원을 뜻하지 않는다. |
| 14 | doctype/comment/PI가 unsupported render node로 남음 | 이 항목은 `Ignored` node로 표현하고 import 시 자식만 순회한다. 문서 계약은 doctype/comment/PI 보존을 약속하지 않는다. |
| 15 | root HTML의 `<script>`가 세션 시작 중 자동 실행됨 | 실제 V8 fixture가 script 실행 sentinel이 정의되지 않았음을 검사했고 Android/iOS Simulator에서 통과했다. |
| 16 | `documentElement/head/body`가 잘못된 parent에서 찾아지거나 getter마다 wrapper가 바뀜 | parser가 생성한 direct-child 관계와 실제 V8 fixture의 반복 getter identity를 확인했다. 첫 PR diff에서 `documentElement`를 `html` 이름에만 한정한 표준 불일치를 추가로 발견해 첫 element child 반환으로 수정하고, root 교체 후 getter와 `head`·`body`의 `null` 결과를 실제 V8 fixture에 추가했다. |
| 17 | 일부 HTML만 commit된 상태로 V8 worker가 외부에 노출됨 | root import 완료 후 초기 HostDocument snapshot을 등록하고 그 다음 V8 runtime을 만든다. 세션 초기화 실패는 startup 오류로 반환된다. |
| 18 | 기존 JavaScript-only `RuntimeSession::new()`의 초기 document가 달라짐 | 기존 생성 경로는 `HostDocumentBridge::new()`를 유지한다. root HTML은 별도 생성 진입점과 fixture에서만 opt-in한다. |
| 19 | fixture API/payload가 제품 기본 binary에 들어감 | host release staticlib에서 default에는 심볼·assertion string이 없고 opt-in feature에서 모두 존재하는 것을 대조했다. 이 결과는 최종 Android/iOS app bundle 전체의 symbol-size 분석은 아니다. |
| 20 | Android/iOS fixture flag가 서로 다르거나 simulator 화면이 실패해도 PASS로 보임 | Cargo feature, C define, Android BuildConfig, Swift compilation condition을 맞췄다. Android API 37 ARM64 AVD와 iPhone 17 Pro iOS 26.2 Simulator 양쪽에서 실제 V8 `PASS` 로그와 캡처를 확인했다. |

## 분리해 둔 제품 범위

이 검토는 fixture가 고정 입력을 읽는지 확인한다. 기본 앱의 `index.html` asset loading, viewport layout/page scale, OS scheme 변화, `prefers-color-scheme` cascade, meta/root CSS `color-scheme`에 따른 UA Canvas 색, HTML 화면의 paint/hit-test는 아직 구현·검증하지 않았다. 이 항목들은 C04.12 완료 기준에 남아 있으며 이번 PASS로 승격하지 않는다.
