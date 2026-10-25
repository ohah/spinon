# C11.4 계획·상태 동기화 PR diff 검토 30개

검토일: 2026-10-25
검토 대상: `plan/c11-4-grid-track-sizing.md`, `plan/c11-4-negative-implicit-track-sizing.md`, `plan/c11-4-orthogonal-row-intrinsic-contribution.md`, `spec/public/roadmap.html`의 현재 diff
기준: `origin/main`의 `spec/STATUS.md` 273, 276, 307–308, 367행, 공개 roadmap 179–180행, PR #623, #627, #379, #385 및 연결된 실행 근거
방법: 문서·GitHub PR metadata·실행 evidence를 읽기 전용으로 대조했다. 테스트와 빌드는 실행하지 않았다.

판정은 현재 문구의 사실 정확성과 상태 동기화를 평가한다. `PARTIAL`은 핵심 주장과 연결 증거가 맞지만 요약에 정확한 구분이나 한계가 빠진 경우다. `FAIL`은 완료 상태와 같은 문서의 실행 지침이 충돌하는 경우다.

| # | 독립 실패 관점 | 판정 | 근거 |
|---:|---|:---:|---|
| 1 | PR #623 링크가 대상 기능을 가리키며 merged 상태인가 | PASS | 상위 계획 418행과 공개 roadmap 179행의 링크는 `feat(c11.4): cyclic percentage와 auto-repeat sizing`; GitHub state `MERGED`. |
| 2 | PR #623 병합 시각이 기록과 일치하는가 | PASS | 계획 418행과 roadmap 179행의 `2026-10-25T04:35:52Z`가 PR metadata와 일치한다. |
| 3 | PR #623 merge SHA가 정확한가 | PASS | 두 문서의 `56daefe96e8143959bc99a89c7a8c80e7f1cc4ba`가 GitHub mergeCommit 및 STATUS 367행과 일치한다. |
| 4 | PR #623 구현 source SHA가 merge SHA와 혼동되지 않았는가 | PASS | 계획 418행과 roadmap 179행은 source `c5b8ef75804e39d36a37aa61b82b1f028115fefd`와 merge SHA를 따로 적고, 실행 README 5행도 source revision을 확인한다. |
| 5 | cyclic target의 intrinsic pass 반복 수가 맞는가 | PASS | 계획 418행과 roadmap 179행의 1회 반복/basis 없음 설명은 계획 및 runtime README 16행의 intrinsic pass와 일치한다. |
| 6 | cyclic target의 final pass basis와 used tracks가 맞는가 | PASS | `24px`, 2회, `[12,12]px`가 두 요약·runtime README 16행·STATUS 367행과 일치한다. |
| 7 | definite control의 독립 기대값이 추적 가능한가 | PASS | 두 요약과 연결 계획 18행/runtime README 17행에 40px control, basis 40px, 2회, `[20,20]px`가 기록돼 있다. |
| 8 | PR #623의 scene 범위를 일반 CSS 지원으로 넓혀 말하는가 | PASS | 계획 418행은 exact two-scene opt-in fixture에 한정하고 일반 CSS admission을 제외한다. roadmap 179행도 exact profile로 범위를 고정한다. |
| 9 | PR #623 focused 검사 결과를 workspace 전체 결과로 과장하는가 | PASS | `runtime/FFI focused tests`로 범위를 한정했으며 workspace 전체 통과를 주장하지 않는다. |
| 10 | PR #623 Android/iOS 실행 환경이 대상과 일치하는가 | PASS | 계획 418행·roadmap 179행의 Android API 37 AVD와 iPhone 17/iOS 26.2 Simulator가 STATUS 367행 및 실행 README와 맞는다. |
| 11 | PR #623 PixelCopy/Metal 표본 수가 요약에 보이는가 | PASS | 두 요약에 target/control scene별 PixelCopy/Metal 표본 `3/3` exact가 있고 Android README 80–81행·iOS 95–96행과 일치한다. |
| 12 | PR #623 AVD renderer 한계를 보존했는가 | PASS | Android AVD가 software renderer라는 점을 두 요약에 명시한다. |
| 13 | PR #623 iOS pixel 표본의 compositor 경계를 보존했는가 | PASS | iOS Metal drawable을 OS compositor 제출 전에 읽었다고 두 요약에 적었다. |
| 14 | PR #623 static archive 결과를 APK/IPA 전체 제외로 확대하는가 | PASS | 상위 계획은 release APK/IPA exclusion을 주장하지 않는다. 공개 roadmap은 archive 결과를 archive marker/symbol로 한정한다. |
| 15 | 공식 C11.4 완료 수가 PR #623으로 잘못 증가하는가 | PASS | 계획 422행·roadmap 179행과 STATUS 367행은 C11.4 미완료 및 `72/213` 유지로 일치한다. |
| 16 | PR #627 링크와 merge 상태가 일치하는가 | PASS | 상위 계획 420행과 orthogonal 계획 상태가 PR #627을 연결하고 GitHub state는 `MERGED`다. |
| 17 | PR #627 병합 시각이 기록과 일치하는가 | PASS | 두 계획에 기록된 `2026-10-25T09:08:33Z`가 GitHub metadata와 일치한다. |
| 18 | PR #627 merge SHA가 정확한가 | PASS | 두 계획의 `f14184a7364fcfa40859efb8ee52962de23f4487`가 GitHub mergeCommit 및 현재 `origin/main`과 일치한다. |
| 19 | orthogonal slice scene 수와 주요 입력 종류가 맞는가 | PASS | 네 `vertical-rl` direct text/natural PNG 장면이 orthogonal 계획의 scope, STATUS 276행 및 runtime README 3, 11–16행과 일치한다. |
| 20 | min/max row 기대값과 순서 대조가 정확한가 | PASS | orthogonal 계획의 scene 표와 상위 계획 420행에 `[50]`/`[80]px`, reversed source order, contribution provenance 및 `Columns → Rows` trace가 기록돼 있다. |
| 21 | #611/#620 회귀 장면 합계가 정확한가 | PASS | 기존 회귀 8개와 orthogonal 4개를 합쳐 12개라는 계획 420행의 구성이 STATUS 276행 및 runtime README 3행과 일치한다. |
| 22 | 각 플랫폼의 통과 수가 합산으로 누락을 감추지 않는가 | PASS | 계획 420행은 Android와 iOS에서 각각 `12/12`라고 분리해 적는다. |
| 23 | Android 실행 대상과 제한이 정확한가 | PASS | API 37 ARM64 16 KB AVD 표기가 STATUS 276행 및 runtime README 20행과 맞고 software renderer 제한도 명시한다. |
| 24 | iOS 실행 대상 및 readback 시점이 정확한가 | PASS | iPhone 17 Pro/iOS 26.2 Simulator와 compositor 제출 전 drawable 제한이 계획 420행·runtime README 28, 49행과 맞는다. |
| 25 | 실제 text/image pixel painting까지 검증했다고 오해할 여지가 있는가 | PASS | 상위 계획 420행과 orthogonal 계획 상태가 runtime image/text/glyph telemetry 0이라 실제 Ahem/PNG pixel painting을 검증하지 않았다고 명시한다. Runtime README 48행도 일치한다. |
| 26 | PR #627 artifact 결과를 최종 APK/IPA 전체 제외로 확대하는가 | PASS | 두 계획은 확인한 것이 static FFI archive뿐이며 final release APK/IPA의 fixture ABI/payload exclusion을 입증하지 않는다고 명시한다. |
| 27 | PR #627의 GitHub check 상태를 CI green으로 잘못 표현하는가 | PASS | GitHub `statusCheckRollup`은 비어 있고 문서에는 CI 통과 주장이 없다. |
| 28 | 두 slice의 공용 dispatcher·trace·host adapter가 병렬 편집된 것으로 기록되는가 | PASS | 계획 422행은 공유 파일을 순차 통합했다고 적으며 orthogonal 계획의 공유 파일 소유 기록과 일치한다. |
| 29 | PR #379/#385의 merge·scene·한계 정보가 상태표와 일치하는가 | PASS | 음수 implicit 계획 상태가 #379 `2026-10-20T07:41:31Z`/`99a2d408385a03591088a44005cb7b35aaf69cb3`, #385 `2026-10-20T10:13:19Z`/`078f5708a5c13d956150b311d392490cc4faa311`을 적는다. 106/106, 18 추가 case 및 124/124, overflow 표본과 남은 한계는 완료 판정 및 STATUS 307–308행과 일치한다. |
| 30 | 완료 상태와 음수 implicit 계획의 구현 절차가 서로 충돌하는가 | PASS | 절차 heading이 `PR #379 당시 구현 및 검증 순서`로 바뀌었고 바로 아래에 당시 계획이며 현재 체크리스트가 아니라고 명시한다. 완료 상태와의 모순이 해소됐다. |

## 후속 조치

- 세 계획 문서의 현재 diff를 다시 대조했다. #379/#385 병합 metadata와 음수 implicit 구현 절차의 역사 표기, #623 target/control 표본 수, #627의 실제 pixel painting 및 final app bundle artifact 한계가 상태표·실행 근거와 일치한다.
- 30개 관점 모두 `PASS`이며 후속 수정 요청은 없다.
- 세 계획 문서는 직접 수정하지 않았다. 테스트·커밋·PR은 하지 않았다.
