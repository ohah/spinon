# C14.3 구현 PR 변경 적대 검토 20개 — 2026-10-16

검토 대상은 계획 PR #215 병합 이후의 C14.3 구현 diff와 그 동작·상태·실행 근거다. 계획 자체를 검토한 기존 표를 재사용하지 않았다. 각 항목은 PR diff에 걸린 독립 실패 경로이며, 검토 중 발견한 수정 사항은 별도 구현 검토와 실행 evidence에 다시 반영했다.

| # | 실패 관점 | diff에서 확인한 방어와 결과 |
|---:|---|---|
| 1 | 구현 내용을 상위 C14 전체 완료로 과장함 | 상태 대장은 C14.3 하위 항목만 갱신하고 C14.1·C14.2·C14 상위 항목은 미완료로 남긴다. |
| 2 | PR branch의 체크박스가 merge 전에 65개 기준을 임의 변경함 | C14.3 행을 실제 merge gate까지 미체크로 유지하고, main 반영 후 Agent Browser DOM으로 전체 수를 확인한다. |
| 3 | 내부 인터페이스 버전 상승이나 공개 CSS 지원 선언이 섞임 | 내부 계약은 `0.1.0` 그대로 두고, 구현은 제한 profile candidate로 서술한다. |
| 4 | 구현 전 oracle 입력·관찰·허용치가 사후에 바뀜 | Chrome 155 observation, reference JSON, 입력 PNG hash, viewport/DPR와 `0.05 CSS px` tolerance를 서로 연결했다. |
| 5 | Chrome reference 화면에서 사용한 PNG 링크가 끊김 | precomparison README의 상대경로를 `./chrome155.png`로 고치고 Agent Browser에서 image completion/natural size를 확인했다. |
| 6 | simulator 성공 문구가 실제 report와 화면에 근거하지 않음 | raw runtime logs, parsed JSON, screenshots를 Android와 iOS 각각 보관한다. 두 report 모두 14/14다. |
| 7 | 작은 containing block의 frame overrun을 누락하거나 clip 증거로 잘못 읽음 | report와 양쪽 화면에서 7px frame overrun을 명시하고 native image paint/clip은 미검증으로 적었다. |
| 8 | 실제 실기기·GPU 검증으로 표현을 부풀림 | AVD와 Simulator 모델·OS·ABI를 특정하고 physical device 및 image painter를 검사하지 않았다고 쓴다. |
| 9 | ADB 설치 실패를 런타임 성공으로 숨김 | Android install/start/logcat/screenshot 원본을 보존한다. 최초 정지 원인은 V8 경로 preflight였고 고정 V8 경로 뒤 ADB 명령은 성공했다. |
| 10 | Android/iOS 기본 빌드에 fixture code/data가 들어감 | default APK의 native library와 iOS executable에서 symbol/payload 부재를 확인하고 raw output을 남겼다. |
| 11 | positive control이 없는 negative-only artifact 주장 | opt-in Android `.so`와 iOS executable에서 동일 symbol 및 fixture 문자열이 존재함을 확인했다. |
| 12 | feature flag가 Rust만 켜고 native/Swift 호출부는 끄거나 그 반대임 | Cargo feature, C header, build shell, Gradle/Xcode, JNI/Objective-C++, Java/Swift compile condition의 opt-in/out을 비교했다. |
| 13 | 앱 fixture가 기본 resource loader/network에 의존함 | live V8 document에 고정 `spinon.invalid` source를 쓰고 tracked PNG를 decode해 intrinsic snapshot으로 넘긴다. |
| 14 | report 입력이 달라진 문서·snapshot을 같은 source로 취급함 | document generation/revision/render revision, intrinsic source/revision match를 report에 기록하고 앱 gate에서 검사한다. |
| 15 | Grid 테스트가 intrinsic track sizing 범위를 몰래 넓힘 | 14-case oracle은 definite fixed-track item만 담고 status/spec/README에 제외 범위를 반복해서 명시한다. |
| 16 | `fit-content()` 함수나 비-image keyword가 fallback sizing으로 성공 처리됨 | negative tests는 node identity를 가진 `UnsupportedIntrinsicSizing`을 요구하며 `auto`/0 대체를 허용하지 않는다. |
| 17 | 앱 case 증가로 Java/Swift report buffer chunk나 display가 잘림 | 실제 2-chunk Android, 9-chunk iOS raw logs를 파싱해 14개 JSON case와 UI summary를 대조했다. |
| 18 | C11.2.1 등 인접 profile 지원 범위가 바뀜 | `align-items:start`는 Template Grid profile에 한정한다. C11.2.1 regression test를 포함한 workspace suite를 다시 통과했다. |
| 19 | 문서의 완료 판정과 실제 구현/API/error contract가 어긋남 | versioned internal layout engine contract와 spec index를 수정하고 status row에는 candidate·미지원 범위를 분리했다. |
| 20 | PR 본문에 작업 ID·검증·남은 경계·동작 화면이 누락됨 | PR #216에 상태 ID·명세·증거·남은 범위와 두 Simulator 화면을 넣었다. Agent Browser에서 두 raw 이미지 URL이 각각 `complete=true`와 원본 크기 1080×2400, 1206×2622로 로드됨을 확인했다. Tailnet roadmap는 merge 후 hash-verified sync로 갱신한다. |

PR merge 직전에 `gh pr checks`, mergeability, base SHA와 Tailnet served hashes를 다시 확인한다. unresolved P1/P2는 없다.
