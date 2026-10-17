# C13.1 Tailnet 로드맵 항목 PR 변경 검토 30개

**검토일:** 2026-10-18
**대상:** 공개 로드맵의 PR #283 병합 기록, 내부 링크 인덱스의 검토 근거 추가
**기준:** 병합된 PR #283, 공식 `spec/STATUS.md`, C13.1 실행 근거 README, plan·내부 계약 원본

| # | 별도 실패 관점 | 판정과 근거 |
|---:|---|---|
| 01 | 변경 대상이 #283의 실제 병합 결과와 일치하는가 | 통과. `origin/main`의 최신 커밋은 #283 병합 SHA `d963ab234b8fd4e8a183e2a481eb8db0f52ebc39`다. |
| 02 | 로드맵 기록이 기존 #281 C13.1 기록을 덮어쓰거나 잘못 귀속하는가 | 통과. 새 문단을 #281 단락 뒤에 추가해 두 runtime slice의 링크와 설명을 분리했다. |
| 03 | 새 기록의 날짜가 근거 날짜와 다른가 | 통과. HTML 제목과 README 모두 2026-10-18을 사용한다. |
| 04 | 페이지의 한국어 문서 계약이나 문서 언어를 훼손하는가 | 통과. 기존 `lang="ko"`와 한국어 사용자 문구를 유지했다. |
| 05 | 정적 진행 문구가 공식 상태 대장처럼 보이는가 | 통과. 구현 상태를 완료로 표시하지 않고 기존 공식 카운터를 별도로 명시했다. |
| 06 | 완료 수를 현재값처럼 고정해 두는가 | 결함 발견. 당시 문구의 `61/197`은 최신 상태 대장과 어긋났고, Agent Browser는 `74/207`을 읽었다. 이번 정정 PR에서 고정 숫자를 제거한다. |
| 07 | C13.1의 미완료 상태를 누락하는가 | 통과. 새 문단에 C13.1 미완료를 명시한다. |
| 08 | C13 상위 범위를 완료로 확대 해석하는가 | 통과. C13.1/C13 완료를 뜻하지 않는다고 한정했다. |
| 09 | fixture 결과를 일반 CSS stacking·overflow 지원으로 확대하는가 | 통과. 고정 fixture 한정이며 일반 CSS 조합을 검증하지 않았다고 적었다. |
| 10 | frame 수가 evidence와 일치하는가 | 통과. 세 case에서 13/13 frame이라고 기록했고 README의 frame matrix와 대조했다. |
| 11 | Android hit 표본 수가 부풀려지는가 | 통과. 실행별 18/18 topmost hit identity로 제한했다. |
| 12 | iOS hit 표본 수가 부풀려지는가 | 통과. 실행별 18/18 topmost hit identity로 제한했다. |
| 13 | Android PixelCopy를 compositor 이후 화면이라고 오해시키는가 | 통과. Android 근거를 API 37 ARM64 AVD와 PixelCopy로 구체화했다. |
| 14 | iOS Metal drawable을 OS compositor 결과로 오해시키는가 | 통과. compositor 제출 전이라고 명시했다. |
| 15 | AVD를 물리 기기 결과로 오해시키는가 | 통과. AVD 명칭과 software renderer를 명시하고 실기기를 제외했다. |
| 16 | SwiftShader 결과를 hardware GPU 결과로 오해시키는가 | 통과. ANGLE/Vulkan/SwiftShader라고 적고 hardware GPU 검증이 아니라고 밝혔다. |
| 17 | Simulator 표본을 실제 iPhone 결과로 오해시키는가 | 통과. iPhone 17 Pro/iOS 26.2 Simulator라고 명시하고 실기기를 제외했다. |
| 18 | OS compositor 또는 scanout을 검증했다고 주장하는가 | 통과. 두 검증 범위를 명시적으로 제외했다. |
| 19 | WPT 적합성 결과로 오해시키는가 | 통과. WPT 검증이 아니라고 적었다. |
| 20 | DOM event dispatch를 검증한 것처럼 보이는가 | 통과. DOM event를 명시적으로 제외했다. |
| 21 | 표면 표본을 전체 surface 픽셀 비교로 오해시키는가 | 통과. 18개 고정 표본으로 제한해 서술했다. |
| 22 | revision negative control을 양 플랫폼 중 하나에만 귀속시키는가 | 통과. 양 플랫폼 모두 readback 전에 거부됐다고 썼다. |
| 23 | negative control이 PixelCopy/Metal readback 뒤에서 실패한 것으로 읽히는가 | 통과. surface readback 전 거부라고 명시했다. |
| 24 | 계획 링크의 preview 경로가 실제 문서와 다른가 | 통과. `plan/c13-1-stacking-descendant-clip-controls.md`의 public HTML 경로에 대응한다. |
| 25 | 내부 계약 링크가 다른 C13.1 계약을 가리키는가 | 통과. C13.1 overflow clip 계약 `0060`을 가리킨다. |
| 26 | 실행 근거 README 링크가 다른 slice를 가리키는가 | 통과. `c13-1-stacking-descendant-clip-runtime-2026-10-18/README.md`를 가리킨다. |
| 27 | Android 화면 링크가 존재하지 않거나 다른 run의 파일인가 | 통과. run 03의 보존된 `android-guard-run-03-screen.png`를 가리킨다. |
| 28 | iOS 화면 링크가 존재하지 않거나 다른 run의 파일인가 | 통과. run 03의 보존된 `ios-guard-run-03-screen.png`를 가리킨다. |
| 29 | PR 링크가 열려 있거나 다른 PR을 가리키는가 | 통과. #283은 이미 병합된 PR이며 링크 번호와 제목이 일치한다. |
| 30 | 내부 인덱스가 새 문서 검토 근거를 누락하는가 | 통과. `spec/internal/README.md`의 C13.1 항목에 이 30관점 검토를 추가한다. |

## 검토 범위

이번 변경은 기존 근거를 공개 로드맵에서 찾을 수 있도록 요약하고 연결하는 문서 변경이다. 새로운 runtime 검증이나 공식 완료 판정은 포함하지 않는다. 새 HTML 링크의 실제 미리보기 응답은 Tailnet 동기화 뒤 Agent Browser에서 별도로 확인한다.
