# C15 계획 PR 변경 적대 검토 20개 — 2026-10-17

검토 대상은 C15 계획 PR의 전체 변경분이다. 계획 자체의 실패 관점 20개를 다시 세지 않고, 이번 PR diff에서 생길 수 있는 상태·근거·범위·재현성 문제를 별도로 확인했다.

| # | 실패 관점 | diff에서 확인한 방어와 결과 |
|---:|---|---|
| 1 | 계획 추가만으로 C15를 완료 처리 | `spec/STATUS.md`에서 C15는 `[ ]`로 남고 계획·Chrome 사전 비교 링크만 연결한다. 통과. |
| 2 | C15 계획을 상위 C16 글꼴 속성 완료로 오인 | C15 line box는 내부 fixture scalar를 쓰고 CSS `line-height` parsing/cascade는 C16이라고 명시했다. 일반 font cascade·`@font-face`·fallback도 C16에 남기고 status를 미완료로 둔다. 통과. |
| 3 | C19.3 painter를 C15 전제 없이 완료로 표시 | 계획 첫머리와 완료 판정이 C15/C16 결과 및 실제 painter 연결을 요구한다. 통과. |
| 4 | 공개 적합성 문서에 아직 구현되지 않은 CSS 지원을 추가 | `spec/0001-conformance.md`는 자료를 Chrome DOM oracle로 한정하고 Spinon·Simulator 검증 아님을 명시한다. 통과. |
| 5 | STATUS에서 사전 비교 근거 경로가 끊김 | C15 계획 및 precomparison README 상대 경로가 격리 worktree에서 존재함을 확인했다. 통과. |
| 6 | C15 계획의 Chrome 결과가 Spinon 측정으로 읽힘 | HTML은 브라우저 DOM geometry를 관찰하며 README·STATUS가 runtime 미실행을 명시한다. 통과. |
| 7 | 버전 고정 기록이 Chrome 실행 환경을 완전히 특정한다고 과장 | full version, OS, arch, locale, viewport, DPR를 기록하고 executable SHA/CDP revision 부재와 구현 전 gate를 남겼다. 통과. |
| 8 | 사전 비교에 폰트 저작권·출처가 빠짐 | Ahem source/공개 도메인 근거와 Roboto upstream/OFL을 README 및 font license file로 보존한다. 통과. |
| 9 | HTML·폰트 입력이 사후 교체되어 기대값과 분리 | 관찰 README에 HTML·font file hash를 적었고 저장 폰트 SHA-256을 재계산해 세 파일과 일치시켰다. 통과. |
| 10 | PR 본문에 넣을 관찰 화면을 찾을 수 없음 | Chrome screenshot이 evidence directory에 있으며 README에서 상대 링크와 SHA-256을 제공한다. 통과. |
| 11 | Ahem만으로 proportional shaping을 주장 | Ahem은 line box/baseline 전용이며 Roboto 변수 폰트의 두 advance가 별도 관찰값으로 기록된다. 통과. |
| 12 | 동일한 글자 수의 문자열 비교가 synthetic measure를 걸러내지 못함 | Roboto `iiii`/`WWWW`가 19.4531px/70.9844px로 다르며 이를 명시적 negative control로 둔다. 통과. |
| 13 | 공백 collapse·wrap·unbreakable overflow·break-all을 한 결과로 뭉침 | 서로 다른 fixture ID와 폭·line count·advance를 plan 및 observation 표에 대응시켰다. 통과. |
| 14 | line box의 element union rect가 inline fragmentation을 숨김 | inline span의 `getClientRects()` 결과 두 개를 각각 21×12px로 기록하고 plan도 fragment 단위 snapshot을 요구한다. 통과. |
| 15 | `0.05px` geometry 허용치가 screenshot raster 차이를 통과 처리 | geometry·glyph coverage·Roboto raster antialias 판정을 분리하고 비교 방법을 plan에 적었다. 통과. |
| 16 | Chrome WPT oracle 결과가 Spinon 적합성 결과로 표시 | WPT path/commit을 구현 전에 선택하고 Chrome oracle을 Spinon WPT pass로 부르지 않는 경계를 둔다. 통과. |
| 17 | Parley 공개 최신판 조사가 dependency 승인으로 오인 | `=0.12.0` 고정은 계획 제안이며 lockfile·API·font ownership·reshaping을 구현 전 source에서 재확인한다. 통과. |
| 18 | Taffy flex-wrap을 inline layout 해결로 채택 | 현재 `CalcLayoutTree` 경계와 definite content width adapter를 조사하고 flex-wrap simulation을 제외했다. 통과. |
| 19 | simulator 또는 물리 기기 확인을 이미 실행했다고 오해 | Android API 37 AVD/iOS 26.2 Simulator는 미래 gate이며 현재 실행 근거 없음과 physical device 미검증을 구분한다. 통과. |
| 20 | PR 검토 문서나 fixture 보조 파일이 상태 대장 완료 근거로 승격 | 계획 검토와 이번 PR diff 검토를 별도 파일로 두고 C15/C16/C19 체크를 유지한다. 모든 C15 링크는 저장소 내 실제 파일을 가리킨다. 통과. |

이번 검토에서 수정이 필요한 P1/P2를 찾지 못했다. `git diff --check`, 변경 파일의 C15 링크 및 fixture font hash를 확인했다. Chrome executable SHA/CDP revision, WPT revision 선택, Parley 잠금 API 확인, Spinon runtime 및 양 simulator 실행은 구현 전/구현 PR의 gate로 남는다.
