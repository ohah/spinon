# C11.4 상태·로드맵 문서 PR diff 검토 · 30개 관점

PR #680 후보 diff의 공식 상태, 계획 상태, 공개 로드맵, 병합 후 검토 근거를 각기 다른 실패 경로로 대조했다. C11 완료 처리, 수치 변경, 범위 과장, 잘못된 병합 정보나 경로 오류를 발견하지 못했다.

| # | 실패 관점 | 검토 결과 |
| ---: | --- | --- |
| 1 | C11 상위 체크가 부분 결과로 완료 처리된다. | 상위 체크는 미완료다. |
| 2 | C11.4 체크가 완료 처리된다. | C11.4는 미완료로 유지된다. |
| 3 | 공식 완료 집계가 임의로 증가한다. | 변경 전후 모두 72/213이다. |
| 4 | 미완료 수나 집계 기준을 바꿔 완료 수를 부풀린다. | 집계 기준과 checklist 항목은 수정하지 않았다. |
| 5 | 계획이 구현 전 상태로 남아 실제 결과와 어긋난다. | 계획은 PR #679 병합과 6/6 matrix 결과를 기록한다. |
| 6 | 계획 상태가 전체 Stage A 완료로 확대된다. | direct-text 제한 slice라고 한정한다. |
| 7 | PR 번호가 다른 C11.4 PR로 바뀐다. | 세 문서는 구현 PR #679를 일관되게 지목한다. |
| 8 | Rebase and merge 시각이 로컬 시각 또는 잘못된 시간대로 기록된다. | 병합 메타데이터의 UTC 시각 `2026-10-27T03:24:44Z`를 사용한다. |
| 9 | merge SHA가 테스트 source SHA와 혼동된다. | merge SHA를 명시하며 README가 테스트된 PR head와 tree 일치를 별도로 기록한다. |
| 10 | check 상태를 성공으로 과장한다. | 빈 rollup과 미설정 required checks를 있는 그대로 표현한다. |
| 11 | Android 결과를 실기기 검사로 표현한다. | Android API 37 ARM64 AVD라고 특정한다. |
| 12 | Android software 환경이 hardware GPU 검증으로 표현된다. | hardware GPU는 미검증으로 남는다. |
| 13 | iOS Simulator 결과가 물리 iPhone에서 수행된 것처럼 보인다. | iPhone 17 Pro / iOS 26.2 Simulator라고 특정한다. |
| 14 | 두 플랫폼의 scene 수가 서로 다르게 기록된다. | Android와 iOS 각각 6/6이다. |
| 15 | normal 대조군이 신규 다섯 scene에 섞여 개수가 모호하다. | 기존 normal을 포함한 총 여섯 장면임이 드러난다. |
| 16 | 장면 범위가 원 evidence와 다르다. | nowrap, break-all, content variation, whitespace, reorder 및 normal 대조군이 기록된다. |
| 17 | 입력이나 revision 검사를 생략했는데 전체 런타임을 검증했다고 쓴다. | 계획·상태는 확인된 input, source/style/revision, trace와 frame 항목을 명시한다. |
| 18 | draw/present 로그를 실제 글리프 출력 근거로 쓴다. | 화면은 layout box/surface이며 글리프 수가 0임을 밝힌다. |
| 19 | runtime screenshot을 pixel equality 근거로 표현한다. | 픽셀 동등성은 주장하지 않는다. |
| 20 | opt-in fixture 결과가 일반 CSS admission으로 확대된다. | 일반 CSS admission과 WPT는 미검증으로 남는다. |
| 21 | 정적 archive 검사를 APK/IPA 전체 경계로 확대한다. | FFI archive 범위에 한정하고 APK/IPA exclusion은 미검증이다. |
| 22 | 미지원 일반 HTML이나 arbitrary text 경로가 지원된다고 암시한다. | 좁은 고정 fixture 경로만 기술한다. |
| 23 | STATUS의 링크가 계획 문서의 잘못된 파일명을 가리킨다. | plan 경로와 matrix 문서명을 확인했다. |
| 24 | STATUS가 실행 README 또는 review 파일을 누락된 이름으로 가리킨다. | linked evidence 파일들이 후보 tree에 존재한다. |
| 25 | Android 화면 링크가 pass01 실패 캡처를 가리킨다. | pass02 Android screenshot을 선택했다. |
| 26 | iOS 화면 링크가 pass01 또는 다른 UDID 실행을 가리킨다. | pass02 iOS screenshot 경로가 매트릭스 README와 일치한다. |
| 27 | 공개 로드맵 route의 `/spinon/` prefix 또는 페이지 alias가 깨진다. | plan, evidence, STATUS route가 기존 Tailnet 경로 규칙을 따른다. |
| 28 | 공개 로드맵과 STATUS의 완료 상태가 서로 다르다. | 두 문서 모두 C11/C11.4 미완료, 72/213으로 맞는다. |
| 29 | PR #679 이미지 확인을 배포된 Tailnet 검증으로 오해한다. | GitHub PR 이미지 로드 확인과 Tailnet 게시 확인을 분리한다. |
| 30 | 미리보기 변경을 현재 PR에서 이미 완료했다고 주장한다. | Tailnet 동기화와 Agent Browser 검증을 후속 단계로 남긴다. |

검토 근거는 후보 diff, `spec/STATUS.md`, `spec/public/roadmap.html`, direct-text 계획 및 pass02 실행 README다. 이 검토는 문서 산출물의 정확성을 대상으로 하며 Tailnet 서버 제공 상태나 전체 문서 사이트 build 성공을 대신하지 않는다.
