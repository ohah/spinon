# C14.1 live V8 이미지 측정 구현 검토 · 20개 실패 관점

**구현:** `plan/c14-1-live-v8-image-measurement.md`
**검토 대상:** actor snapshot 요청, C14.1 app fixture, 기본/opt-in artifact, Android/iOS Simulator 실행
**검토 결과:** P1/P2 미해결 없음. 분석만으로 확인한 경계와 실제 실행한 경계를 표에 구분했다.

| # | 독립 실패 관점 | 검토 결과 |
|---:|---|---|
| 1 | 일반 빌드에서 C14 전용 runtime 요청 API가 노출되는가 | `c14-1-live-v8-fixture-test` cfg와 FFI feature 연결을 확인했다. Android/iOS 기본 최종 artifact에서 fixture C ABI와 actor snapshot symbol이 없고 opt-in positive control에는 있다. 통과. |
| 2 | actor 외부 thread가 mutable `HostDocumentBridge`를 직접 읽는가 | snapshot 요청만 actor queue에 제출하고 actor match arm에서 immutable snapshot을 복사한다. 두 Simulator에서 실제 문서 결과를 받았다. 통과. |
| 3 | eval 완료 전에 snapshot 명령이 앞서 이전 문서를 읽는가 | eval response 이후 snapshot을 enqueue한다. 두 플랫폼의 report가 script 뒤 document revision 8과 render-tree revision 2를 기록했다. 통과. |
| 4 | shutdown과 snapshot 제출의 경쟁에서 닫힌 session이 성공하는가 | `submission` lock 아래 `closing`을 확인한다. 전용 runtime test에서 shutdown 뒤 호출 거부 1/1을 확인했다. 통과. |
| 5 | queue 포화가 빈 문서 또는 성공 report로 변환되는가 | `EnqueueError::Full`을 오류로 반환해 fixture가 fail-closed 된다. 실제 full-queue 주입은 하지 않았으며 분기 검토만 했다. 통과. |
| 6 | queue 종료 후 대기 중인 snapshot 명령이 caller를 무한 대기시키는가 | scheduler drain이 snapshot reply에 명시적 오류를 보낸다. 종료/대기 interleaving의 별도 주입은 하지 않았으며 코드 경계를 확인했다. 통과. |
| 7 | actor가 snapshot 응답을 보내기 전 종료하면 channel 오류를 숨기는가 | `Disconnected`가 명시적 `Err`로 변환된다. 별도 worker-panic 주입은 하지 않았으며 분기 검토만 했다. 통과. |
| 8 | actor 정체가 caller를 무한 대기시키거나 timeout 뒤 문서를 바꾸는가 | 응답 대기는 5초로 제한한다. 늦게 처리돼도 해당 명령은 snapshot 복사만 하며 DOM 변경을 하지 않는다. timeout 주입은 하지 않았으며 구조로 확인했다. 통과. |
| 9 | worker 소유 문서 데이터가 snapshot 응답 뒤에도 mutable alias로 남는가 | `HostDocument::snapshot()`이 owned snapshot을 반환한다. FFI는 actor bridge가 아니라 snapshot만 request/layout에 공급한다. 통과. |
| 10 | root가 없거나 여러 개일 때 첫 root를 임의 선택하는가 | live snapshot root가 정확히 한 개여야 진행한다. 양 플랫폼의 양성 report는 동일한 단일 root를 기록했다. 통과. |
| 11 | 대상이 빠졌거나 비-HTML/detached/wrong-tag element여도 image로 처리하는가 | 단일 child, HTML namespace, `img` tag, id와 connected 상태를 확인한다. 양 플랫폼의 report는 하나의 연결된 `img`를 관찰했다. 별도 구조 mutant는 실행하지 않았고 fail-closed 분기를 확인했다. 통과. |
| 12 | 다른 요소의 src 또는 비어 있는 src에 고정 PNG를 붙이는가 | `ImageRequest::from_document` 뒤 URL을 고정값과 exact match한다. `.invalid` URL을 network fetch하지 않고 고정 bytes에만 decode한다. 통과. |
| 13 | 기준 PNG 파일이 같은 크기의 다른 payload로 바뀌어도 oracle에 통과하는가 | reference의 fixture id, filename, byte length와 SHA-256을 decoder 전에 비교한다. digest 교체 unit negative control이 2/2 FFI 테스트에서 거부됐다. 통과. |
| 14 | request/resource/layout source가 서로 다른 문서 generation·revision을 섞는가 | request와 resource 및 intrinsic snapshot source identity를 비교한다. 양 Simulator report에서 요청/resource/layout source 일치가 참이다. 통과. |
| 15 | `src` 변경 뒤 이전 decode completion이 새 문서에서 받아들여지는가 | 실제 V8에서 `src`를 바꾸고 actor snapshot revision 8→9를 읽었다. 이전 completion은 양쪽에서 정확히 `SourceChanged`로 거부됐다. 통과. |
| 16 | stale negative control이 다른 오류를 `SourceChanged` 성공으로 잘못 세는가 | NodeId가 동일한 image인지와 오류 variant가 `SourceChanged`인지 함께 검사한다. 양쪽 app report의 `stale_source_rejected=true`와 해당 variant를 확인했다. 통과. |
| 17 | intrinsic frame을 root 또는 다른 node에서 조회하는가 | frame key는 검증된 image `NodeId`다. Android/iOS report 모두 image NodeId 2의 `(0,0) 24×16` frame을 기록했다. 통과. |
| 18 | simulator DPR/AVD density를 자연 CSS px에 다시 곱하거나 단위를 섞는가 | Chrome oracle·layout report가 CSS px를 사용하고 viewport scale을 1로 고정한다. 두 화면과 log에서 natural size `24×16 CSS px`, frame `(0,0) 24×16 CSS px`가 일치한다. 통과. |
| 19 | FFI output buffer·문자열 변환 또는 실패 status가 성공으로 보이는가 | Android JNI·iOS C ABI 경로가 JSON status를 화면과 log로 전달한다. 양쪽 opt-in route는 `status=ok`; 기본 route는 명시적 disabled log와 화면을 냈다. 별도 작은 buffer 주입은 하지 않았고 기존 함수의 bounded writer 경계를 유지했다. 통과. |
| 20 | 측정 report를 제품 loader, image paint 또는 overflow 경계 증거로 과장하는가 | 앱 화면은 report text이며 image를 paint하지 않는다. docs와 evidence에서 network loader·async replacement·image pixels·overflow clip을 제외하고 C14.1 미완료를 유지한다. 통과. |

## 실행 경계

- `mise exec -- cargo test --locked --workspace --quiet`: 866 passed, 4 ignored, 0 failed.
- `spinon-runtime` C14 actor snapshot/closed-session focused test: 1/1 passed.
- `spinon-ffi` C14 PNG reference positive/negative tests: 2/2 passed.
- Workspace Clippy, C14 opt-in Clippy, rustfmt와 `git diff --check`: 통과.
- Android API 37 ARM64 AVD와 iPhone 17 Pro/iOS 26.2 Simulator: opt-in report 성공, 기본 route disabled.
- 기본 artifact exclusion과 opt-in positive control: [실행 근거](c14-1-live-v8-image-measurement-2026-10-16/README.md).

초기 host FFI E2E 시도는 해당 test harness에 Android/iOS V8 provider가 없어 isolate 생성에서 실패했다. 이를 앱 통과로 세지 않았고, E2E 증거는 양쪽 Simulator 앱 경로에서 얻었다. 이 실패 원인과 재실행 범위도 evidence README에 보존한다.
