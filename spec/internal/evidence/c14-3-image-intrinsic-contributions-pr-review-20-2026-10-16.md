# C14.3 계획 PR #215 변경 적대 검토 20개 관점

대상: [PR #215](https://github.com/ohah/spinon/pull/215)의 계획·상태·oracle fixture·기대행렬·화면 evidence 변경. 구현 PR 검토와 혼합하지 않는다.

## 결과

20개 관점을 적용했고 미해결 P1/P2는 없다. GitHub 확인 시점에 base는 `main`, PR은 `MERGEABLE`, `gh pr checks`는 `no checks reported`를 반환했다. 따라서 필수 CI green으로 표현하지 않는다. Agent Browser에서 PR 본문 screenshot image가 `complete=true`, `1280×823`으로 로드됐다.

## 관점별 공격과 결과

1. **불필요한 코드 변경** — 구현이 검토 없이 섞였는지 확인했다. 변경은 계획, 상태/인덱스 문서, fixture, oracle, review evidence뿐이다.
2. **로드맵 상태 조작** — oracle 수집만으로 C14.3 또는 상위 C14를 완료 처리했는지 확인했다. C14.3와 C14/C14.1/C14.2 모두 unchecked다.
3. **상위 계획 정합성** — 부모 C14 계획이 새 child plan을 연결하는지 확인했다. C14.3 행의 범위·선행 항목·미완료 상태가 갱신됐다.
4. **범위 선언 정합성** — status/index가 이미지 paint나 공개 CSS support까지 주장하는지 확인했다. Chrome geometry만 기록하고 native image paint/clip과 공개 지원은 제외했다.
5. **의존성 우회** — 새 계산이 snapshot ownership이나 stale revision 경계를 피하도록 계획됐는지 확인했다. 기존 immutable C14.1/C14.2 input·source gate를 요구한다.
6. **oracle/version 증거** — 파일의 버전과 browser가 실행한 버전이 다른지 확인했다. 전체 version/platform은 high-entropy UA에서 보존했고 reduced UA도 함께 적었다.
7. **PNG drift** — oracle data가 다른 입력을 가리키는지 확인했다. asset 경로, natural size, SHA-256이 README·expected JSON에 일치한다.
8. **미로드 성공 오판** — 빈 image를 natural-size case로 받아들이는지 확인했다. raw observation은 case 13/13의 `loaded=true`, natural `24×16`을 보존한다.
9. **expected/raw divergence** — canonical matrix가 observation과 다른 크기를 적는지 대조했다. 13개 expected image used size가 raw geometry와 모두 일치한다.
10. **CSS precision** — computed string rounding을 exact oracle로 썼는지 확인했다. unrounded `getBoundingClientRect()` CSS px 값과 `0.05px` tolerance를 사용한다.
11. **percentage basis 오해** — border-box를 content-box로 오인해 reference가 잘못됐는지 확인했다. definite cases는 200×120 content basis를 고정한다.
12. **cyclic/definite 경로 합침** — zero basis를 definite case의 값으로 대체할 수 있는지 확인했다. cyclic min-content는 별도 case로 basis/image `0×0`을 보존한다.
13. **Flex 변형** — wrapping·stretch·grow/shrink가 oracle에 섞이는지 확인했다. 단일 item의 `flex:0 0 auto`, row/nowrap, `align-items:flex-start`로 제한했다.
14. **Grid 범위 팽창** — 미구현 intrinsic track/auto-repeat/spanning을 지원한다고 오인시킬지 확인했다. fixed 200×120 tracks 안의 단일 item만 수집한다.
15. **border 밖 크기** — 콘텐츠 영역 초과와 border edge 초과를 혼동하는지 다시 계산했다. 12px content, 22px 부모 border box, 5px image offset에서 edge 밖 7px로 적었다.
16. **screenshot provenance** — screenshot이 제품 화면처럼 보이거나 PR에서 깨지는지 확인했다. Agent Browser로 PR 이미지 URL을 열어 실제 1280×823 PNG load를 확인했고 oracle capture임을 표기했다.
17. **text 측정 침투** — browser label이 Android/iOS 기대 layout을 바꿀 수 있는지 확인했다. app fixture에서는 text node를 제외한다고 명시한다.
18. **플랫폼 검증 허위 표기** — 미실행 AVD/iOS 결과를 통과한 것처럼 썼는지 확인했다. PR 본문과 상태 문서 모두 제품 Simulator 검증이 아직 없다고 분리한다.
19. **검증/라벨 누락** — 계획 PR을 구현 완료처럼 분류하거나 evidence를 숨겼는지 확인했다. `planning`, `area: layout`, `area: docs`, `area: testing` 라벨과 plan review·oracle 링크가 있다.
20. **merge gate** — CI 부재를 green으로 처리하거나 잘못된 base로 병합할 수 있는지 확인했다. `main` base와 mergeability를 확인했고 checks 부재를 그대로 기록한다. rebase 병합 직전에 상태를 다시 조회한다.

이 검토는 PR #215 계획 diff에 한정된다. C14.3 구현 후에는 별도 구현 실패 관점 20개와 해당 구현 PR의 새 20개 변경 검토가 필요하다.
