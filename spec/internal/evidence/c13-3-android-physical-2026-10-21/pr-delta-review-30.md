# C13.3 Android 실기기 근거 PR 변경 관점 검토 30개

이번 PR diff는 Android 제품 코드나 지원 계약을 바꾸지 않고, 실기기 gesture fixture 실행 근거와 상태·로드맵 기록을 추가한다. 기존 구현·계획 검토와 다른 관점으로 원본 캡처, 주장 경계, 문서 간 연결, PR 전달성을 다시 확인한다.

1. **상태 대장이 C13.3을 완료 처리하는가?** — 새 실행 근거만 추가하고 C13.3 미완료 문구를 유지한다.
2. **공식 완료 집계가 바뀌는가?** — `74/207`을 그대로 유지한다. 이 단일 실행을 별도 완료 항목으로 세지 않는다.
3. **새 문서가 public API 지원을 암시하는가?** — opt-in debug fixture 경로임을 밝히고 DOM event나 일반 HTML/CSS 지원으로 확장하지 않는다.
4. **플랫폼 범위를 Android 전체로 일반화하는가?** — SM-S731N 단일 기기·Android 16/API 36로 한정한다.
5. **테스트 대상이 물리 기기인지 근거가 있는가?** — 고정 serial, ADB USB device 행, 모델 정보와 기기 실행 logcat을 함께 보관한다. 연결 emulator 둘은 입력에 사용하지 않는다.
6. **실행한 APK가 local artifact와 같은가?** — local/installed SHA-256 일치를 README와 `environment.json`에 연결한다.
7. **APK가 release 제품 빌드처럼 표현되는가?** — opt-in debug feature 조합이라고 적고 release APK 동작은 검증하지 않았다고 한정한다.
8. **물리 GPU가 아닌 renderer를 hardware 결과로 과장하는가?** — Vulkan backend와 Samsung Xclipse 940 renderer log를 명시한다. surface capture를 optical output으로 부르지 않는다.
9. **실기기 입력이 사람 손가락 입력으로 표현되는가?** — ADB `input swipe`가 만든 synthetic MotionEvent라고 README·summary·로드맵에 구분한다.
10. **input source가 app callback에서 확인되지 않았는데 기록에 들어가는가?** — source bits/tool type, down/pan/up 원본 touch log를 evidence folder에 둔다.
11. **성공 설명이 최종 offset 대신 예상값을 적는가?** — `summary.json`의 최종 run에 맞춰 outer Y `88.42639 CSS px`, inner `(86,106)`, root `(0,0)`으로 통일했다.
12. **run별 이동량을 고정 보장처럼 표현하는가?** — 이 값은 2026-10-21 단일 실행의 관측값이며 지원 계약이 아니다.
13. **negative control의 5px가 touch slop 아래인지 입증하는가?** — 5 physical px와 기기 native slop 23px을 함께 적고 모든 move가 거부됐다.
14. **negative control 결과가 다른 실행 수치와 섞이는가?** — 현재 `summary.json`의 initial/final offsets와 before/after screenshot hash가 revision·offset·pixel 불변을 보인다.
15. **pixel 수가 README·로드맵·상태 대장에서 어긋나는가?** — positive 87,204 inside·0 outside와 below-slop 0·0을 같은 최종 run 기준으로 맞췄다.
16. **capture 영역이 화면 밖 fixture 부분을 포함한 것처럼 보이는가?** — visible surface 범위와 scrollport를 구분하고, 화면 밖 fixture clipping을 별도 단락에 기록한다.
17. **400 CSS px fixture가 기기 폭에 모두 보였다고 읽히는가?** — available 384 CSS px, SurfaceView 좌우 22/23 physical px clipping을 로드맵과 근거 README에 적었다.
18. **화면 밖 경계에 걸친 입력이 성공을 만들었는가?** — gesture point는 display 안 좌표 `(400,1363)→(400,1110)`이고 parser는 signed surface origin을 보존한다.
19. **스크린 캡처 비교가 PixelCopy나 scanout으로 오인되는가?** — ADB full-screen screenshot sample로 표시하고 PixelCopy·optical scanout을 주장하지 않는다.
20. **positive 변경 pixel이 scrollport 경계를 넘었는가?** — root scrollport 안 85,514 변경, 바깥 123,525 비교 pixel 중 0 변경을 기록한다.
21. **화면 결과가 PR에서 확인 가능한가?** — PR 본문에 before/after 원본 PNG 두 장을 side-by-side로 넣을 수 있고, README에도 상대 image link가 있다.
22. **근거 link가 잘못된 route/파일명을 가리키는가?** — README, summary, screenshot, review 문서와 public roadmap HTML 경로가 실제 evidence path를 가리킨다.
23. **roadmap note가 명세 source를 덮거나 배포를 암시하는가?** — 공개 roadmap source의 부분 검증 note만 추가한다. 지원 계약은 STATUS/spec에 남아 있고 site 배포는 하지 않는다.
24. **상태 대장에 실행 한계가 빠지는가?** — ADB 합성 입력, 단일 기기, debug fixture, clipping, no raw touch/optical scanout을 포함한다.
25. **재현 지시가 이전 device/branch에 의존해 불명확한가?** — `mise` build, fixed serial install, evidence runner 순서와 APK path를 README에 둔다.
26. **검증기 성공 기준이 이번 변경에서 재현 가능한가?** — model/API/display/APK hash, 시작 상태, scroll owner, queue/draw, below-slop, screenshot boundary와 failure marker assertions가 script에 있다.
27. **실행 시각이나 결과가 마지막 실행과 어긋나는가?** — README input times는 `summary.json`의 최종 run `20:19:11.004Z`/`20:19:15.307Z`에 맞췄다. 앞선 실행의 stale timestamps는 수정했다.
28. **해시 manifest가 수정 뒤 낡거나 자신을 포함하는가?** — merge 전 evidence·두 review 문서 확정 후 다시 만들고 evidence directory에서 `shasum -a 256 -c`로 확인한다. manifest 자신은 제외한다.
29. **PR 제목·라벨·양식이 프로젝트 규칙을 따르는가?** — `docs(android): ...` 제목, 기존 `documentation` 및 `area: android` labels, 상태/API 적용 여부/근거/범위를 채운다.
30. **검사하지 않은 CI 통과나 배포 완료를 쓰는가?** — 별도 프로젝트 테스트를 주장하지 않고 PR 생성 후 required checks와 merge state를 확인한다. 병합은 Rebase and merge만 사용하며 docs site는 배포하지 않는다.

검증 결과와 플랫폼 한계는 [실기기 evidence](README.md)에 있다. PR 본문에는 이 변경에서 확인한 결과와 before/after 화면을 연결한다.
