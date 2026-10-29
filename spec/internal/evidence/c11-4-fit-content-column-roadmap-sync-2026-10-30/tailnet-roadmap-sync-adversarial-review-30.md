# C11.4 Tailnet 로드맵 동기화 적대 검토 30개

검토 대상은 병합 뒤 `spec/STATUS.md`, C11.4 집중 계획, 공개 roadmap, 선택된 Tailnet 문서 미리보기 및 이 동기화 근거다. 2026-10-30 기준 소스·manifest·원격 route hash·Agent Browser 표시를 대조했다.

| # | 실패 관점 | 확인 결과 |
|---:|---|---|
| 1 | 제한 slice를 전체 C11 완료로 오인 | Roadmap과 STATUS가 전체 C11 미완료 한계를 명시한다. 통과. |
| 2 | C11.4 checkbox를 조기 완료 표시 | 로컬과 Tailnet STATUS checkbox 모두 unchecked다. 통과. |
| 3 | 전체 완료 수를 72에서 증가 | 세 문서와 렌더링된 페이지가 72/213을 유지한다. 통과. |
| 4 | 병합 전 PR 상태를 완료로 둠 | GitHub 조회는 PR #811 `MERGED`다. 통과. |
| 5 | 잘못된 merge SHA 표기 | GitHub merge commit과 문서의 `8b2e3088c84ae4a3df1fd56c9e75eba150ce1f59`가 같다. 통과. |
| 6 | 병합 일시 누락·불일치 | GitHub `mergedAt` 및 문서의 2026-10-29T17:19:06Z가 같다. 통과. |
| 7 | 빈 check rollup을 CI 성공으로 표현 | STATUS와 roadmap이 check rollup 및 required check 조회가 비었다고 쓴다. 통과. |
| 8 | 필수 check의 부재를 통과 판정으로 확대 | 문서가 CI 성공으로 세지 않는다고 제한한다. 통과. |
| 9 | Android scene 수 과장 | Roadmap은 해당 slice의 4/4만 기록한다. 통과. |
| 10 | iOS scene 수 과장 | Roadmap은 해당 slice의 4/4만 기록한다. 통과. |
| 11 | 표면 표본 수나 정확성을 잘못 기재 | 14/14 표본은 구현 근거와 일치한다. 통과. |
| 12 | Android AVD를 실기기·hardware GPU로 오인 | AVD software 경로이며 실기기/hardware GPU를 확인하지 않았다고 표시한다. 통과. |
| 13 | iOS Metal drawable을 compositor 이후 pixel로 오인 | 제출 전 표본으로 한정하고 compositor 이후를 미검증으로 표시한다. 통과. |
| 14 | fixture-only 결과를 일반 author CSS 지원으로 확대 | 일반 author CSS admission 제외 범위를 표시한다. 통과. |
| 15 | 고정 scene 결과를 전체 WPT로 확대 | 전체 WPT 미검증 범위를 표시한다. 통과. |
| 16 | 전체 Grid intrinsic sizing 또는 C11.4를 완료 처리 | 공개 항목은 제한 runtime slice만 설명한다. 통과. |
| 17 | Roadmap source가 구현 PR merge 정보를 빠뜨림 | HTML과 렌더된 페이지에 #811 및 merge SHA가 보인다. 통과. |
| 18 | 상태 대장에 계획·증거·review 연결이 끊김 | STATUS 링크가 계획과 precomparison/runtime/review 문서를 가리킨다. 통과. |
| 19 | 집중 계획에 stale pending 상태 | Agent Browser에서 pending/unmerged 문구가 검출되지 않았다. 통과. |
| 20 | Android 화면 링크가 잘못되거나 깨짐 | Tailnet README image가 complete, natural size 1080×2400이다. 통과. |
| 21 | iOS 화면 링크가 잘못되거나 깨짐 | Tailnet README image가 complete, natural size 1206×2622이다. 통과. |
| 22 | 문서 build의 별도 root에서 정적 자산 누락 | 최종 manifest의 required route에 `icon.svg`가 포함된다. 통과. |
| 23 | 최종 선택 build가 핵심 route를 빠뜨림 | manifest의 92개 중 required route 목록에 roadmap, STATUS, plan, evidence와 이미지가 있다. 통과. |
| 24 | 선택 build 산출물 수·크기 불일치 | pass02 manifest는 92개, 7,176,017 bytes다. 통과. |
| 25 | preflight가 기존 파일을 백업하지 않고 덮어씀 | 교체 전 4개 대상, 총 2,022,171 bytes를 보관했다. 통과. |
| 26 | 기존 Tailnet route의 의도하지 않은 삭제 | preflight, copy, remote verify 모두 no-deletes를 기록한다. 통과. |
| 27 | 무관한 404 페이지가 변경됨 | pass01 preflight는 복사되지 않았고 pass02에서 404.html을 제외했다. 통과. |
| 28 | 원격 복사 뒤 대상 hash가 로컬과 달라짐 | 92/92 remote HTTP 200 및 SHA-256 검증이 일치한다. 통과. |
| 29 | 공개 배포나 다른 preview target을 잘못 갱신 | 기존 SpinonPreview `/spinon/` 대상만 갱신했고 GitHub Pages 실행은 하지 않았다. 통과. |
| 30 | 검증용 브라우저가 남거나 사용자 Chrome을 종료 | 해당 Agent Browser session을 닫았고 `pgrep -fl agent-browser` 결과가 비었다. 다른 Chrome 프로세스는 종료하지 않았다. 통과. |

## 판정

확인된 blocking 결함은 없다. 초기 최소 root 빌드는 `icon.svg` 누락으로 실패했고 첫 preflight에는 무관한 `404.html`이 잡혔으나, 두 경우 모두 원격 복사 전에 감지해 pass02에 반영했다. 최종 동기화·표시 근거는 `README.md`, pass02 manifest 및 `agent-browser-verification-pass02.json`에 있다. 이 검토는 GitHub Pages 배포나 전체 문서 내용의 독립적인 의미 검증을 뜻하지 않는다.
