# PR #203 변경 적대 검토 20개 관점

검토 범위: `spec/public/roadmap.html`, Tailnet sync 기록·manifest·Agent Browser 화면, PR 본문과 이미지 검증. 기준은 PR base `efa1989f7f03b006c339e402e52bbf029278b5d4`; PR 본문은 Agent Browser로 확인한 공개 렌더 결과를 포함한다.

1. **공식 완료 수를 근거 없이 올렸는가?** 아니다. Tailnet 실제 counter는 `197개 표시 · 공식 완료 61/197`이며 이 변경에서 공식 체크를 편집하지 않았다.
2. **C13.3 전체 완료로 과장했는가?** 아니다. 로드맵은 C13.3 미완료를 유지하고 남은 iOS callback, 물리 복합 입력, 제품 경로와 기타 gates를 적는다.
3. **AVD 결과를 실기기·주변기기 결과로 바꿔 썼는가?** 아니다. 문구는 API 37 AVD uinput 경로이며 실제 복합 입력장치와 iOS는 미검증이라고 분리한다.
4. **지원 source 합집합을 event source로 혼동하는가?** 아니다. 합집합 `0x10200a`와 event source `0x2002`를 구별하고 pointer tool `mouse`를 함께 적었다.
5. **분류·전달 결과가 evidence와 맞는가?** 예. `MouseWheel`, target 7, normalized 64 CSS px, draw revision 2가 PR #202의 복합 AVD 기록과 일치한다.
6. **거부 경로가 admission 결과와 섞였는가?** 아니다. SurfaceView 바깥 입력은 queue 전에 거부되고 화면이 유지된 별도 control로 설명한다.
7. **pixel 근거의 측정 경계를 넓혀 주장했는가?** 아니다. 수치는 SurfaceView 정렬 crop의 root scrollport 내부/외부 pixel 비교로 한정했다.
8. **새 로드맵 링크가 실제 Tailnet 경로와 맞는가?** 예. `/spinon/spec/internal/evidence/c13-3-native-wheel-composite-avd-2026-10-16/README.html`을 Agent Browser에서 직접 열었다.
9. **문서 route에서 상대 이미지 링크가 깨졌는가?** 아니다. 네 임베드 이미지의 `naturalWidth`/`naturalHeight`가 모두 1080×2400이고 깨진 이미지 목록은 비었다.
10. **수정된 standalone roadmap과 Tailnet root 파일이 같은가?** 예. 저장소 `spec/public/roadmap.html`과 Tailnet `/roadmap.html` SHA-256이 모두 `9612f115a6c1b4fa78c704d9f2d80b1a8c6c6f1f5daa95fae12750c3033791bd`다.
11. **링크된 계획·상태 문서가 기존 preview에서 낡은 채 남았는가?** 관련 C13.3 plan, STATUS, internal index와 evidence route를 부분 빌드에서 생성해 manifest 대상에 넣었다.
12. **preview build 설정을 저장소 설정에 넣었는가?** 아니다. Rspress dead-link/image 예외는 임시 scoped config에만 두었으며 저장소의 docs config는 수정하지 않았다.
13. **제한 빌드를 전체 docs 검증이라고 표현했는가?** 아니다. PR 본문은 선택된 route의 scoped Rspress build라고 적고 전체 적합성으로 일반화하지 않는다.
14. **Tailnet 변경을 GitHub Pages 배포나 서버 교체로 표현했는가?** 아니다. local Tailnet static root의 파일 sync만 했고 Pages 배포와 서버 재시작은 없었다.
15. **복사 목록이 미리보기 전체를 덮어쓰거나 index를 바꾸는가?** 아니다. manifest는 103개 명시 경로이며 `index.html`은 대상이 아니고 삭제 목록도 비었다.
16. **교체 전 파일의 복구 근거가 있는가?** 예. 16개 교체 파일은 복사 전에 backup root에 저장했고 이전 SHA-256이 manifest에 남아 있다.
17. **manifest 밖 preview 파일을 수정하거나 삭제했는가?** 아니다. sync 스크립트는 manifest에 적힌 파일만 복사했으며 다른 파일 정리 명령은 사용하지 않았다.
18. **PR 본문 raw 이미지 URL이 공개 화면에서 깨지는가?** 아니다. Agent Browser에서 세 이미지가 각각 1280×633, `complete=true`, `naturalWidth>0`으로 표시됐고 깨진 이미지가 없었다.
19. **PR 내용과 라벨이 문서 변경의 성격을 드러내는가?** 예. `area: docs`와 `spike`를 붙이고 C13.3 미완료 상태, API 변경 없음, 남은 범위를 본문에 적었다.
20. **이 변경이 미검증 제품 계약을 닫았다고 주장하는가?** 아니다. 구현 완료를 선언하지 않으며 PR의 목적을 로드맵 요약·Tailnet preview 갱신으로 제한했다.

## 판정

20개 관점에서 이 PR의 범위 안에 차단할 문서 오류를 찾지 못했다. C13.3 product gates와 전체 docs dead-link 검증은 이 PR에서 완료되지 않았다고 구분했다. PR 본문 이미지는 GitHub 공개 렌더에서 별도로 확인했다.
