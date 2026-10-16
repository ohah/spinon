# C14.2 live V8 simulator 계획 PR 변경 적대 검토 20개 — 2026-10-16

검토 대상은 C14.2 후속 계획 PR diff, 공식 상태/내부 인덱스 링크, C14.3 병합 뒤 남아 있던 상태 문구 수정이다. 계획 실패 관점 표와 다른 PR 통합 경계이며, 구현 검토를 대신하지 않는다.

| # | 독립 PR 실패 관점 | 확인 결과 |
|---:|---|---|
| 1 | 후속 검증 계획을 제품 image loader나 C14.2 계산기 재구현으로 확대한다 | plan은 live V8 app fixture만 추가하고 network/cache/replacement/paint/WPT를 명시적으로 제외한다. **통과** |
| 2 | PR #156이 아직 미병합인 것처럼 C14.2 근거가 기록된다 | 최신 main의 PR #156 merge SHA/날짜를 확인해 오래된 "merge 직전 확인" 문구를 제거했다. **통과** |
| 3 | C14.2 완료 checkbox를 simulator 검증 전 미리 켠다 | STATUS의 C14.2는 미완료로 유지하고 새 29-case gate를 남은 조건으로 기록했다. **통과** |
| 4 | C14.1 또는 상위 C14, U05, S02 제품 경계가 함께 완료된 것처럼 보인다 | 후속 plan과 STATUS 모두 이 상태 ID를 별도 미완료로 유지한다. **통과** |
| 5 | 새 plan이 기존 C14.2 oracle과 연결되지 않는다 | 기존 fixture, inventory, Chrome observations 및 PNG hash를 직접 링크한다. **통과** |
| 6 | 27개 Block inventory 전체가 지원 가능한 PNG case로 세어 분모가 부풀려진다 | 양성 분모를 21 PNG + Flex 8개로 고정하고 SVG/누락/깨진 입력을 제외한다. **통과** |
| 7 | Block과 Flex scene을 한 문서에 합쳐 Chrome 좌표를 이동시킨다 | 서로 다른 live V8 문서 두 개로 구성해 각각의 원본 scene 좌표계를 유지한다. **통과** |
| 8 | Block observation에 없는 containing-block frame을 비교 가능한 값처럼 요구한다 | Block은 image full frame, Flex는 observed parent와 image full frame을 비교하도록 구분했다. **통과** |
| 9 | app route의 main branch가 source main과 다른 fixture를 참조한다 | run 시 case ID/order/src를 고정 oracle/inventory에 대조하도록 요구한다. **통과** |
| 10 | fixture가 synthetic HostDocument 경로를 사용해 live V8 검증이라 과장된다 | 양 문서의 root/img tree를 RuntimeSession::eval에서 생성하도록 계획했다. **통과** |
| 11 | V8 snapshot feature가 C14.2 feature와 결합되지 않아 default artifact에 들어간다 | C14.2 Cargo feature가 C14.1 live-V8 capability만 재사용하고 자체 FFI/payload는 opt-in으로 둔다. **통과** |
| 12 | 한 문서의 generation/revision을 다른 문서에 재사용해 섞인 resource snapshot을 만든다 | 두 문서 각각 독립 IntrinsicSizeRevision/resource snapshot과 stamp를 유지하도록 요구한다. **통과** |
| 13 | simulator scale이 CSS px oracle에 적용된다 | DPR 1 oracle의 CSS px 결과를 비교하고 Android/iOS scale 곱을 금지한다. **통과** |
| 14 | Chrome max-width subpixel 동작을 반올림해 회귀가 사라진다 | 원시 oracle `19.984375×13.328125`를 report에 보존하고 반올림하지 않는다. 명시적 0.05 CSS px tolerance 안에서 20px도 허용될 수 있음을 밝혀 oracle 값을 20px로 덮어쓰는 동작과 구분했다. **통과** |
| 15 | PR에 screens이 있더라도 report 내부가 29개를 검증하지 않는다 | 29/29 및 개별 frame/source/match 필드를 성공 조건으로 고정하고 raw report를 보존한다. **통과** |
| 16 | 화면 report가 잘려도 성공 상태가 표시된다 | case 누락·frame 누락·FFI capacity truncation은 fail-closed 조건이다. **통과** |
| 17 | Android emulator를 실기기 검증으로 표현한다 | Android API 37 ARM64 AVD와 iOS 26.2 Simulator를 명시하고 물리 기기 연결/조회 금지를 적었다. **통과** |
| 18 | default artifact exclusion이 static library 검사만으로 release bundle 전체 주장이 된다 | 검사는 APK/native library와 iOS linked app 범위로 제한해 해석하도록 적었다. **통과** |
| 19 | C14.3 main merge 뒤 65→66 확인과 pending merge 문구가 충돌한다 | PR #216 merge 및 Agent Browser 66/198 확인으로 STATUS/index/plan/evidence의 현재 문구를 정리했다. **통과** |
| 20 | 이 PR이 기존 리뷰와 계획 검토를 같은 것으로 세어 20회를 충족했다고 주장한다 | 계획의 20개 실패 관점과 PR diff의 이 별도 20개 관점을 분리한다. **통과** |

**판정:** PR의 링크·범위·상태 전이를 검토했다. 미해결 P1/P2가 없다. 이 계획 PR은 구현 PR의 검토나 29-case 실행 결과를 대신하지 않는다.
