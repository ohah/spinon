# C15.2b 정적 로드맵 PR 변경 검토 30개

대상은 `spec/public/roadmap.html`에 추가한 C15.2b 요약 한 항목이다. 코드를 다시 검토하는 대신 공개 요약의 사실 일치, 용어, 상태 경계, 링크와 기존 내용 보존을 각각 확인했다.

| # | 검토 관점 | 확인 결과 |
| --- | --- | --- |
| 1 | 새 요약이 C15.2a보다 앞서 나타나는가 | 2026-10-17 C15.2b 항목은 기존 C15.2a 바로 뒤, C14.1 뒤에 배치돼 순서가 유지된다. |
| 2 | 구현 PR 번호가 실제 병합본을 가리키는가 | PR #242는 `MERGED`이고 제목·merge commit은 GitHub에서 확인했다. |
| 3 | run 별 font family/size/line-height가 실제 변경과 일치하는가 | 계약 0070과 C15.2b evidence가 기록한 입력 의미만 요약했다. |
| 4 | mixed line의 line-height가 Chrome oracle과 일치하는가 | 고정 fixture 값 24px을 기록했다. |
| 5 | 공통 baseline 수치가 잘못 반올림되는가 | Chrome과 Rust fixture의 baseline 19px을 기록했다. |
| 6 | inline fragment의 서로 다른 y를 한 값으로 축약하는가 | 요약은 개별 Ahem/Roboto y를 주장하지 않고 연결한 evidence로 상세 비교를 제공한다. |
| 7 | anonymous flow에서 Block y가 segment y와 섞이는가 | Block y=10px, trailing anonymous segment y=20px로 구분해 기록했다. |
| 8 | parent geometry가 정확한가 | 고정 oracle parent `30×40px`을 그대로 기록했다. |
| 9 | source order를 무시하고 일반 Block layout으로만 설명하는가 | direct Block 앞뒤 anonymous paragraph를 같은 Taffy flow에 둔다고 명시했다. |
| 10 | 검증 결과 수가 실제 focused logs와 다른가 | text-layout 17/17와 C15 layout 16/16은 저장된 focused log와 일치한다. |
| 11 | workspace 총계에 filtered 테스트 수를 잘못 넣는가 | 전체 954 passed·0 failed·4 ignored는 workspace log와 일치한다. |
| 12 | Clippy/rustfmt 결과가 실행되지 않은 상태에서 표기되는가 | `clippy.log` 및 같은 구현 PR의 formatter 실행 근거를 확인했다. |
| 13 | Android ARM64 cross compile을 AVD 앱 실행으로 부풀리는가 | 문구에 Rust target compile이라고 한정했다. |
| 14 | iOS target compile을 Simulator 앱 실행으로 부풀리는가 | 앱 빌드·실행을 검증하지 않았다고 같은 문장에 적었다. |
| 15 | glyph paint 결과를 geometry test에서 추정하는가 | 실제 glyph paint 미검증을 분명히 적었다. |
| 16 | border 밖 ink나 clip을 검증한 것처럼 쓰는가 | overflow clip 미검증을 적었다. |
| 17 | runtime text admission이 이미 동작하는가 | projection/renderer가 계속 text를 거부한다고 명시했다. |
| 18 | C15 checkbox 또는 official completion이 바뀌는가 | 정적 요약은 진행 기록이며 C15/C15.2 미완료를 유지한다. |
| 19 | C16 font feature 완료를 암시하는가 | CSS cascade, font loading/fallback, runtime paint를 따로 완료 주장하지 않는다. |
| 20 | 화면 증거가 Spinon 앱 출력이라고 오해되는가 | 새 요약은 Chrome screenshot이나 앱 screenshot이라고 말하지 않고 oracle 기반 수치로 한정한다. |
| 21 | 새 evidence README route가 Rspress output 위치와 맞는가 | `/spinon/internal/evidence/.../README.html`은 앞선 C15.2a Tailnet route convention과 같다. |
| 22 | 구현 검토 링크가 실제 문서로 이어지는가 | `implementation-review-20.html`의 source Markdown 파일이 있다. |
| 23 | PR 변경 검토 링크가 실제 문서로 이어지는가 | `pr-delta-review-30.html`의 source Markdown 파일이 있다. |
| 24 | 병합 PR 링크가 branch URL 또는 잘못된 번호를 가리키는가 | 링크는 canonical `https://github.com/ohah/spinon/pull/242`다. |
| 25 | 로드맵 페이지가 구현 상태 대장을 대신하는가 | official state는 기존처럼 `spec/STATUS.md`에 둔다고 보존하고, 이 note에는 완료 표시가 없다. |
| 26 | 전체 완료 수를 수동으로 잘못 갱신하는가 | 이 변경은 개별 C15 요약만 추가하며 roadmap counter/code를 수정하지 않는다. |
| 27 | 기존 C15.2a 기록을 C15.2b 수치로 덮는가 | 변경은 새 paragraph 하나를 삽입하고 C15.2a text를 보존한다. |
| 28 | 삽입 지점 주변의 C14.1/C19.2 내용이 제거되는가 | C14.1 marker를 기준으로 앞에만 삽입; 기존 뒤·앞 문구는 diff에서 보존됐다. |
| 29 | HTML 링크에 잘못된 경로 prefix나 Markdown 확장자가 남는가 | 세 evidence 링크는 현재 Tailnet mirror 구조의 `.html` route이며 repository source link는 GitHub canonical URL이다. |
| 30 | 아직 동기화하지 않은 Tailnet 화면을 최신이라고 보고하는가 | 요약은 repository source이며 Tailnet 반영 완료를 주장하지 않는다. Tailnet 제공 검증은 별도 동기화 뒤 기록한다. |

## 판정

요약은 고정 geometry evidence와 미완료 경계를 유지한다. 이 PR은 아직 Tailnet server filesystem을 수정하지 않으며, server 반영과 Agent Browser 확인은 main 병합 뒤 별도 evidence PR에서 한다.
