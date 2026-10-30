# C11.4 Grid text baseline PR diff review (30 checks)

PR diff review의 대상은 last-baseline 계획 후보를 구현/runtime 후보로 확장한 변경이다. 모든 검사는 동일한 final PR head에 대해 재실행하고, 승인된 범위와 미검증 경계를 구분한다.

1. **공식 상위 C11 checkbox 유지** — PASS. 이 제한 fixture로 C11 완료를 주장하지 않는다.
2. **공식 집계 유지** — PASS. 전체 로드맵 공식 수 `72/213`을 변경하지 않는다.
3. **C11.4 parent 상태 유지** — PASS. 제한 slice 하나로 C11.4 intrinsic track sizing 전체를 완료 처리하지 않는다.
4. **계획 상태와 코드 상태 일치** — PASS. 계획 문서가 Chrome 사전 비교만 끝난 “계획 후보”로 남지 않도록 implementation/runtime 결과를 링크한다.
5. **독립 oracle 분리** — PASS. CSS Grid 규범 계산과 Chrome 156 관측을 별도 파일로 고정하고 구현 결과를 oracle의 입력으로 쓰지 않는다.
6. **내부 계약 숫자** — PASS. 공개 API나 계약 `0059` 숫자 버전을 바꾸지 않는다.
7. **fixture module 경로** — PASS. C11.4 FFI fixture는 `runtime_gpu/fixtures/c11_4/`에 둔다.
8. **fixture feature gate** — PASS. product default 경로로 새 scene route를 노출하지 않는다.
9. **지원 admission 경계** — PASS. 일반 CSS first/last baseline 값은 ordinary projection에서 계속 거부한다.
10. **Grid/Flex 경계** — PASS. 새 sentinel은 Grid adapter에서만 만들어지고 Flex last-baseline 지원 주장을 추가하지 않는다.
11. **writing-mode 경계** — PASS. 고정 profile이 `horizontal-tb` 외 입력을 거부한다.
12. **box-edge 경계** — PASS. 직접 text leaf의 margin/border/padding 제약을 유지한다.
13. **font 안정성** — PASS. 저장소 Ahem asset과 고정 font revision을 사용한다.
14. **font/width 범위** — PASS. 임의 font와 폭을 일반 지원한다고 서술하지 않는다.
15. **scene 개수 일치** — PASS. 기존 first regression 4개와 신규 last/control 5개, 총 9개가 catalog·capture·native expectations에 들어간다.
16. **inherited align-items** — PASS. parent `last baseline` scene이 검증 대상이다.
17. **align-self override** — PASS. 개별 `align-self:last baseline` 장면을 따로 확인한다.
18. **source-order reversal** — PASS. 뒤집힌 scene은 source order와 placement geometry를 독립적으로 검증한다.
19. **first-baseline control** — PASS. 기존 one-line geometry와 first metric을 재검증한다.
20. **start control** — PASS. baseline grouping이 없는 multiline control을 포함한다.
21. **mutation rejection** — PASS. malformed profile mutation은 partial layout 대신 fail-closed runtime 오류를 낸다.
22. **trace validation** — PASS. scene당 Columns/Rows stages와 text line baselines, frames를 확인하고 누락된 trace chunk를 거부한다.
23. **Android device identity** — PASS. manifest는 API 37 ARM64 QEMU AVD를 식별한다.
24. **Android install binding** — PASS. 설치 APK와 빌드 APK SHA-256이 같다.
25. **Android surface sampling** — PASS. 9 scene × 3 PixelCopy 표본, 27/27 exact다. renderer는 SwiftShader다.
26. **iOS device identity** — PASS. manifest는 iPhone 17 Pro / iOS 26.2 Simulator를 식별한다.
27. **iOS install binding** — PASS. 설치 executable과 빌드 executable SHA-256이 같다.
28. **iOS surface sampling** — PASS. 9 scene × 3 Metal drawable 표본, 27/27 exact다. compositor 제출 전 표본이다.
29. **초기 Android 실패 기록** — PASS. reversed-scene validator indexing 오류와 수정 후 전체 재실행을 README에 함께 기록한다.
30. **검증/PR/배포 경계** — PENDING PR. 최종 형식·diff·archive-boundary 검사와 simulator evidence는 통과했다. PR checks/state/merge SHA 및 STATUS가 게시 roadmap과 일치하는지 최종 head에서 확인한 뒤 이 항목을 갱신한다.

29개 항목은 source review와 현재 실행 근거에 대한 검토다. 30번은 PR 생성·병합과 roadmap 동기화까지 마친 뒤 닫는다.
