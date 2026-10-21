# C11.4 anonymous Grid border overflow PR diff 실패 관점 검토 20개

이 검토는 구현 정확성 검토와 분리해 현재 제출 diff의 wiring, 회귀 범위, evidence·문서 연결을 확인한다. 계획 검토는 [계획 리뷰](../spec/internal/evidence/c11-4-anonymous-grid-border-overflow-precomparison-2026-10-21/plan-review-20.md), 코드 경로 검토는 [구현 리뷰](./c11-4-anonymous-grid-border-overflow-implementation-review-20.md)다.

1. **fixture 의존성이 기본 feature가 될 수 있음** — `spinon-image`와 live V8 의존성은 `c04-runtime-gpu-fixture-test`에만 연결했다. workspace 기본 build는 기존 default configuration으로 검사했다.
2. **Rust 모듈 파일이 컴파일 그래프에서 빠질 수 있음** — FFI fixture 모듈, runtime layout fixture 등록, 기존 anonymous text/image test 모듈에서 각각 새 파일 경로를 등록했다.
3. **scene ID가 다른 fixture handler에 가려질 수 있음** — C11.4 source routing에서 새 두 scene을 기존 scene 분기보다 앞서 분기하며 ID가 서로 다름을 확인했다.
4. **공용 test harness의 scene 수 상수가 낡을 수 있음** — simulator capture scene 수를 169에서 171로 늘리고 모든 ID의 유일성 검사를 유지했다.
5. **선택 실행에서 다른 수백 개 scene까지 돌아갈 수 있음** — `--scene-id`는 두 신규 scene으로 제한되며 AVD/iOS 각 결과 JSON은 `sceneCount=2`다.
6. **실행 근거가 서로 다른 build에서 올 수 있음** — Android installed APK SHA가 local APK와 일치하고 iOS 결과 JSON에 Simulator 앱 SHA를 보존했다. 빌드 로그는 같은 evidence root에 있다.
7. **공유 Android assertion의 text-index 변경이 기존 scene을 깨뜨릴 수 있음** — 기존 두-image/세-image forward·reverse 네 장면을 AVD에서 다시 돌려 모두 frame, trace, draw 통과를 확인했다.
8. **같은 기존 회귀가 iOS에서만 실패할 수 있음** — 같은 네 장면이 iPhone 17 Pro/iOS 26.2 Simulator에서도 모두 통과했다.
9. **새 text run 두 개를 기존 단일 text 기대값과 혼동할 수 있음** — Android/iOS validator는 `textIndices` 배열과 각 node id를 사용하고 두 direct run의 frame/trace를 확인한다.
10. **contribution 검사에서 row 축이 column 기대값으로 검사될 수 있음** — native verifier는 axis마다 intrinsic span 목록을 별도로 지정해 columns의 `[1,2]`와 rows의 빈 목록을 구분한다.
11. **2회 min-content iteration 기대값이 잘못 재사용될 수 있음** — 신규 span-1 text contribution 분기만 별도 처리하고 기존 scene 분기 회귀를 양쪽 simulator에서 실행했다.
12. **native 오류 로그가 사후에 묻힐 수 있음** — script는 선택 run ID의 `SPINON_C114_GRID_ASSERTION_ERROR`를 최종 trace 검사 전 실패 처리한다.
13. **새 pixel test가 모든 C11.4 scene에 끼어들 수 있음** — Android PixelCopy와 iOS Metal expected sample은 신규 두 scene ID에서만 선택된다.
14. **pixel matrix 실행 allowlist에서 빠질 수 있음** — Android surface matrix scene allowlist와 iOS pixel-run scene 분기에 두 scene이 명시돼 있다.
15. **image attachment report가 track-trace JSON을 깨뜨릴 수 있음** — FFI summary는 image report를 trace JSON보다 앞에 붙인다. 두 플랫폼 run이 complete trace chunk를 파싱했다.
16. **draw assertion이 layout image 수만 검사할 수 있음** — script는 `runtime_image_count=2`와 `image_draw_count=2`가 함께 기록된 draw 행을 요구한다.
17. **문서가 physical Android나 compositor 이후 표시를 주장할 수 있음** — evidence README는 API 37 AVD, iOS compositor 전 drawable을 구분하고 물리 기기 검증을 부인한다.
18. **계획 문서가 구현 완료 상태와 맞지 않을 수 있음** — 계획은 simulator/구현 완료, 리뷰·PR 작업 진행으로 갱신했다. 공식 `spec/STATUS.md`와 C11.4 완료 집계는 건드리지 않았다.
19. **PR 화면 이미지 링크가 바뀌거나 깨질 수 있음** — PR 본문은 커밋 SHA를 고정한 저장소 screenshot URL 두 개를 사용하고, 제출 후 Agent Browser에서 두 이미지 표시 상태를 확인한다.
20. **roadmap 변경이 Pages 배포나 별도 공식 완료 처리로 번질 수 있음** — 후속 반영은 기존 Tailnet preview 절차로 한정하고 Pages workflow/`gh-pages`와 공식 완료 집계는 변경하지 않는다.

최종 PR 생성 뒤 19번의 실제 표시 상태를 Agent Browser로 확인하고, 20번의 Tailnet 파일 hash·HTTP 결과를 반영 근거에 추가한다. 그 확인 전까지 이 문서의 해당 두 항목은 제출 전 gate로 남긴다. 현재 diff 검토에서 코드·evidence 경로를 끊는 blocker는 발견하지 못했다.
