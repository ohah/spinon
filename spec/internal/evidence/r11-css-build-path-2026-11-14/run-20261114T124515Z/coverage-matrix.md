# R11 검사 범위표 · run-20261114T124515Z

이 표는 14행이며, R11의 기존 질문 8개와 실행·진단 후속 경계 6개를 담는다. 별도 verify.mjs 실행은 입력 사례 12개를 확인했다.

| R11 질문 | 판정 | 근거와 판정 범위 |
|---|---|---|
| Lightning CSS 고정 버전이 CSS를 AST visitor로 읽는가? | 확인됨 · current revision 재현 | package/lockfile 및 resolved module이 `1.33.0`이다. converter source는 `transform({ filename, code, visitor })`를 호출한다. source hash는 R10 commit `5ad70525…`, R14 evidence tree와 current HEAD에서 같다. |
| 변환기가 받는 문법 경계가 명시적인가? | 확인됨 · 제한 subset | 단일 class style rule과 `display:flex`, row/column direction, finite nonnegative px width/height/padding/gap, finite nonnegative flex-grow만 매핑한다. 중복 class rule, `!important`, 기타 property/selector/at-rule/unsupported values는 거부한다. 전체 세부 분기는 [`compile-css.mjs`](../../../../../spikes/style-layout/compile-css.mjs)에 있다. 이는 CSS의 넓은 부분집합이 아니다. |
| 성공 JSON의 version과 field가 무엇인가? | current revision 재현 | sample 입력의 출력은 JSON `version: 1`, `styles.screen`/`styles.card`와 명시된 값이다. 신규 출력 445 bytes SHA-256 `83c2f74e00d92a5789ce4cb405263fb0f875e8e64fe9fff228a4accff233ae5a`가 기존 `styles.json`과 정확히 같다. 상세 output은 [`cases/r10-sample/output.json`](./cases/r10-sample/output.json)이다. |
| 알 수 없는 입력에서 부분 성공이나 stale JSON이 남는가? | 시험한 경우 확인됨 · 일반 warning 경계 미확인 | current `verify.mjs`의 12개 경계가 통과했다. mixed `display:flex; color:red`는 exit 1, output 없음이다. R14의 보존 JSON은 전체 generated Tailwind CSS와 나머지 제한 control도 거부하고 JSON을 만들지 않았다고 기록한다. `transform()` warning 반환 metadata는 코드가 소비하지 않아 일반적인 warning 완전성은 미확인이다. |
| output 소비자가 format error를 거부하는가? | 기존 R10 근거 · 이번 run에서 재실행 안 함 | [`main.rs`](../../../../../spikes/style-layout/src/main.rs)는 `CompiledStyles.version == 1`을 요구하고 serde `deny_unknown_fields`로 알 수 없는 style field를 거부한다. R10 계획/테스트 근거는 제한 Taffy fixture 소비를 설명한다. 이번 실행은 Rust tests를 돌리지 않았다. |
| Tailwind generated CSS가 전체 지원되는가? | 아니오 · 기존 R14 역사 근거 | R14는 5,830-byte input 전체를 첫 `layer-statement`에서 거부하고 분리 `.flex`만 version 1 JSON으로 변환했다. `p-4`, color token, hover selector/media, responsive media, Preflight selector도 거부했다. raw result JSON SHA-256 `e34d8a121550ce679d48fde51de21ccf6ce65eb36ec488a0aaff81269b9f1f96`, generated CSS SHA-256 `c11cf6cff5c8e4b3053da8d328cda229884a6b59f9b4f3910d5b1a1f2023740b`. 이는 당시 historical run이다. |
| 과거 결과를 current source 결과와 구분했는가? | 확인됨 · revision 연결 | R10 converter inputs/outputs의 historical blob hash와 current hash가 같다. R14 source/evidence tree는 current의 ancestor commit `7ff04305…`와 동일한 tree다. R10 summary와 R14 README의 후속 편집 hash는 raw generated/output artifacts와 분리해 기록했다. 세부 내용은 [`historical-source-hashes.txt`](./historical-source-hashes.txt). |
| 입력 선언의 출력 누락과 parser warning까지 검증했는가? | sample exact match 확인됨 · warning은 미확인 | sample CSS의 expected `styles.json` 전체와 actual output이 byte-identical이다. negative/mixed input은 실패한다. converter가 Lightning CSS `transform()` 반환 metadata를 보존하지 않아 warning을 관찰했다고 주장하지 않는다. |

| 경고 metadata 완전성 | 미확인 | converter가 transform 반환값을 버린다. stdout/stderr가 비어 있어도 반환 warning을 배제하지 못한다. |
| Parser 오류와 AST visitor 거부의 진단 분리 | 미확인 | verify 요약은 전체 exit 0만 보존하고 개별 case 진단을 남기지 않았다. |
| 강제 종료·I/O 실패 중 partial output | 미확인 | 이번 negative case는 visitor rejection이며 interrupted write는 실행하지 않았다. |
| 실행 전 output path/symlink preflight | 재현 증거 불충분 | 계획에는 조건이 있으나 case metadata에 preflight 결과가 없다. |
| R14 생성 source lineage | 부분 확인 | generator package/lock/input/fixture/verify source와 generated CSS/JSON hash는 history/current HEAD에서 일치한다. 역사 실행의 per-control raw logs는 없다. |
| 실행 순서·임시 파일 cleanup | 후속 정리 | original snapshot은 temp path creation order를 입증하지 못한다. copied evidence hash 확인 후 task-owned temp는 삭제했으며 기록은 cleanup-follow-up에 있다. |

## 종합 판정

확인된 범위는 R10 sample의 versioned JSON byte-exact 재생성과 verify.mjs aggregate 성공이다. warning 완전성, 개별 CSS diagnostic, interrupted output, output preflight, R14 generation lineage는 미확인 또는 부분 확인이다. R11을 R10/R14와 구분하는 공식 완료 범위·판정 기준도 없다. 제품 CSS path, Tailwind support 또는 R11 완료로 확대하지 않으며 official checkbox와 완료 수는 유지한다.
