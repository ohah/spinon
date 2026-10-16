# C14.1 · live V8 문서의 PNG 자연 크기 측정 계획

## 목표

기존 C14.1 앱 fixture가 직접 만든 synthetic `HostDocument`를 사용하던 경로를 실제 `RuntimeSession`의 V8 DOM 변경으로 생성된 `HostDocumentSnapshot`으로 바꾼다. 고정 PNG 요청을 snapshot의 HTML `img[src]`에서 만들고, 기존 `spinon-image` decoder·`ImageResourceSnapshot`·C14.1 CSS px layout snapshot 경로에 전달한다.

이번 단계는 **V8에서 변경된 문서 → actor가 소유한 문서 snapshot → 고정 PNG decoder → intrinsic size map → Stylo/Taffy adapter** 연결 증거다. 플랫폼 앱은 기존 report-only opt-in 경로를 재사용한다. 제품 resource loader, 자동 fetch/decode, 비동기 replacement scheduling, renderer image paint와 화면 pixel은 범위 밖이다. 내부 계약 버전 `0.1.0`, C14.1, C14, U05, X16 상태는 미완료로 유지한다.

## 비교 모델 · fixture · 사전 판정

- **독립 oracle:** Chrome `155.0.8059.40` 사전 비교와 고정된 [C14 PNG reference](../tests/fixtures/resources/c14-1-png-basic-reference.json). PNG는 [c14-1-png-basic.png](../tests/fixtures/resources/c14-1-png-basic.png), SHA-256 `5d8c9509a0fec2288f02611a8ccc22a23a90033d1a96b894b439525398bce4a2`다. 예상 natural size는 `24×16 CSS px`, `width:auto;height:auto`의 image frame은 `(0,0) 24×16 CSS px`다.
- **V8 재현 입력:** [c14-1-live-v8-document.js](../tests/fixtures/css/c14/c14-1-live-v8-document.js)가 `RuntimeSession::eval`로 `div#root`와 단일 HTML `img#image`를 만들고 고정 URL `https://spinon.invalid/c14-1-png-basic.png`를 연결한다. CSS는 기존 fixture에서 추출한 고정 author stylesheet이며 JS DOM이 `<style>` parsing을 제공한다고 가정하지 않는다.
- **환경:** Rust fixture와 Android API 37 ARM64 AVD, iPhone 17 Pro / iOS 26.2 Simulator에서 같은 opt-in C ABI route를 실행한다. 기기 scale과 AVD density를 CSS px에 곱하지 않는다. 물리 기기는 선택하거나 조회하지 않는다.
- **관찰값:** V8 eval status/report, actor snapshot의 generation·document revision·render-tree revision, root와 연결된 HTML image 개수, image `NodeId`와 현재 `src`, request/resource/layout source identity, intrinsic revision, decoded natural size, 결과 frame과 단위, 앱 route 및 실행 환경을 기록한다.
- **통과 조건:** eval이 `status=0`이고 snapshot에 HTML root 하나와 연결된 HTML image 하나가 있어야 한다. 현재 `src`가 fixture URL과 정확히 같아야 하며 요청 source, image resource snapshot source, style-layout source의 generation/document/render-tree revision이 일치해야 한다. PNG byte length와 SHA-256이 reference에 고정한 값과 일치해야 한다. decoder와 측정 map은 `24×16 CSS px`, 결과 frame은 `(0,0) 24×16 CSS px`이고 각 frame 필드의 Chrome 차이는 `0.5 CSS px` 이하여야 한다. report에서 `document_origin=live-v8`와 내부 measurement revision을 확인한다.
- **명시적 실패:** V8 예외, actor 폐기/종료/큐 포화, root 누락·복수, 연결 끊긴 image, HTML namespace가 아닌 image, image 누락·복수, 빈/다른 `src`, corrupt PNG, stale source/generation, 중복 image NodeId, unsupported CSS profile, layout frame 누락, report buffer 부족은 성공으로 기록하지 않는다.
- **해석 한계:** 이 비교는 실제 V8 host binding이 만든 문서 snapshot을 기존 측정 adapter에 공급하는 내부 fixture 결과다. 앱에서 URL을 fetch했다거나 네트워크·cache·async stale admission·GPU/Metal image paint 또는 공개 CSS/API 지원을 검증하지 않는다.

## 설계와 격리

1. `spinon-runtime`에 `c14-1-live-v8-fixture-test` feature를 추가한다. 해당 feature가 켜졌을 때만 `RuntimeSession`이 owner actor에 문서 snapshot 요청을 넣는 fixture 전용 메서드를 제공한다. V8 actor의 `HostDocumentBridge`를 다른 스레드에서 직접 참조하지 않는다.
2. snapshot 요청은 실행 actor가 처리하고 불변 `HostDocumentSnapshot`을 응답한다. 닫힌/stopped worker, 큐 포화, 5초 응답 timeout, 응답 채널 종료는 오류다. timeout 뒤 늦게 실행된 읽기 명령은 문서를 변경하지 않으며 수신자가 없는 응답을 버린다. 호출 정의는 actor 처리 시점에 최신 commit된 snapshot을 돌려주는 것으로 한정한다. 이 테스트 메서드는 공개 JS/C ABI가 아니다.
3. `spinon-ffi`의 기존 `c14-1-app-fixture-test`가 runtime feature를 함께 활성화한다. 기본 feature-off Android/iOS 산출물에는 fixture 호출·JS/PNG payload·snapshot 요청 코드가 없어야 한다.
4. FFI fixture는 V8 실행·snapshot 검증을 먼저 수행하고, snapshot tree를 순회해 고유 root와 연결된 고유 HTML `img` handle을 찾는다. 해당 `src`가 fixture URL과 일치하지 않으면 고정 PNG bytes를 디코드하지 않는다.
5. 현재 snapshot에서 `ImageRequest`를 만든 뒤 고정 PNG를 decode한다. `ImageResourceSnapshot::new`가 현재 source와 `src`를 다시 확인하게 두고, CSS cascade/layout은 현재 snapshot과 같은 source stamp의 immutable measurement snapshot을 받는다. async network loader나 source remap은 추가하지 않는다.

## 구현 전 실패 관점 검토

구현을 시작하기 전에 이 계획에 대해 서로 다른 20개 failure perspective를 [계획 검토 기록](../spec/internal/evidence/c14-1-live-v8-plan-review-2026-10-16.md)에 남긴다. 계획에서 발견한 P1/P2 결함은 먼저 반영한다.

## 실행 순서

1. 이 계획과 20개 계획 검토를 확정하고, 비교 모델 링크를 [0001 적합성 문서](../spec/0001-conformance.md)에 추가한다.
2. 고정 V8 JavaScript fixture를 추가한다.
3. actor-owned snapshot 요청을 fixture feature로만 연결하고 queue/full/closed/disconnect 경로를 오류로 전달한다.
4. 기존 C14.1 app adapter를 V8 snapshot 사용으로 전환하고 document/source/revision identity를 report에 포함한다.
5. 실행 fixture의 positive case와 stale-source, disconnected/non-image, invalid route/report 경계를 검토한다. 기존 synthetic-only 성공을 live V8 증거로 재사용하지 않는다.
6. 코드 작성 후 계획과 다른 20개 구현 실패 관점 검토를 수행해 발견 결함을 고치고, 수정된 관점을 다시 확인한다.
7. feature-off/on artifact 경계를 검사하고 Android API 37 ARM64 AVD 및 iPhone 17 Pro / iOS 26.2 Simulator 앱에서 각각 실행한다. 각 플랫폼의 최신 앱 build, report log, 화면 capture를 개별 보존한다. simulator 한쪽의 결과로 다른 쪽을 대신하지 않는다.
8. PR diff에 대해 [필수 프로젝트 규칙](../docs/project-rules.md)에 따라 별도의 30개 적대 관점 검토를 기록하고, mergeability와 필수 checks를 갱신 확인한다. PR은 `main`을 대상으로 만들고 Rebase and merge를 사용한다.
9. `spec/STATUS.md`에 이번 증거가 연결하는 경계를 기록하되 C14.1 완료 체크는 유지하지 않는다. 병합 뒤 Tailnet 정적 미리보기와 Agent Browser 결과를 확인한다.

## 완료 판정과 제외 범위

- V8 JavaScript가 만든 HostDocument snapshot에서 request source, PNG natural size와 C14.1 frame을 Chrome oracle 기준으로 확인하고 Android/iOS Simulator 각각의 opt-in 앱 경로에서 같은 report contract를 관찰한다.
- 기본 artifact에서 fixture symbol·fixture payload가 빠지고 opt-in artifact에서 positive control이 확인된다.
- 구현 적대 검토는 서로 다른 20개, PR 적대 검토는 서로 다른 30개 failure perspective를 남기고 P1/P2 미해결을 두지 않는다.
- 성공해도 C14.1 product integration, URL loading, async decode/replacement, stale scheduling, display/paint, pixel parity, 실제 기기, C14/U05/X16 완료를 주장하지 않는다.

## 2026-10-16 실행 상태

- live V8 document → actor snapshot → fixed PNG decode → intrinsic snapshot → Stylo/Taffy 측정을 Android API 37 ARM64 AVD와 iPhone 17 Pro/iOS 26.2 Simulator에서 각각 성공시켰다. src mutation stale negative control도 양쪽에서 `SourceChanged`로 거부됐다.
- 기본 Android/iOS 앱 artifact의 C14.1 native symbol과 고유 JS/PNG payload 부재, opt-in positive control 및 기본 route disabled를 확인했다.
- workspace tests, focused feature tests, workspace/feature Clippy, rustfmt와 diff check를 통과시켰다. 원본 로그·화면·hash는 [실행 근거](../spec/internal/evidence/c14-1-live-v8-image-measurement-2026-10-16/README.md), 별도 20개 구현 검토는 [구현 실패 관점 검토](../spec/internal/evidence/c14-1-live-v8-implementation-review-20-2026-10-16.md)에 있다.
- 별도 PR 변경 검토 30개를 마쳤고, `intrinsic_source_matches` 보고 필드 불일치를 분리해 수정한 뒤 feature test·Clippy를 다시 통과시켰다([PR 변경 실패 관점 검토](../spec/internal/evidence/c14-1-live-v8-pr-delta-review-30-2026-10-16.md)). PR checks는 보고되지 않았고 기본 branch protection의 필수 status check와 승인 수는 0이다. Rebase and merge 및 병합 뒤 Tailnet preview hash 확인은 남아 있다. 이 실행 결과로 C14.1 또는 C14를 완료 표시하지 않는다.
