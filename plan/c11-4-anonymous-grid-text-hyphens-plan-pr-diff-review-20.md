# C11.4 익명 Grid 텍스트 `hyphens` 계획 PR 변경 실패 관점 20개

이 검토는 계획 자체의 20개 관점과 별도이며, 계획·색인·사전 비교 근거의 PR 변경분을 공격적으로 확인한다.

| # | 실패 관점 | 검토 결과 |
| --- | --- | --- |
| 1 | 문서 전용 변경이 제품 코드 구현을 완료했다고 암시하는가 | 새 계획, CSS oracle, Ahem 계산, 화면만 추가한다. 구현 미시작을 반복해 명시한다. **통과** |
| 2 | 공식 로드맵 카운터를 계획 PR에서 올리는가 | `spec/STATUS.md`와 C11 상위 체크 상태를 수정하지 않고 `72/213`을 유지한다. **통과** |
| 3 | C11.4 후보 계획이 C11 완료 판정으로 오인되는가 | C11/C11.4 미완료를 계획, 색인, 사전 근거에 표시한다. **통과** |
| 4 | 계획 위험 검토와 PR 변경 검토가 같은 산출물인가 | 별도 20항 계획 검토와 이 20항 PR diff 검토를 둔다. **통과** |
| 5 | 동작 설명이 비규범 자료에만 의존하는가 | CSS Text 3 §5.3 W3C 원문 링크와 Chrome 관찰을 분리한다. **통과** |
| 6 | Chrome 결과가 Spinon 구현 결과로 표현되는가 | 근거 제목·브라우저 버전·한계에서 사전 비교임을 명시한다. **통과** |
| 7 | 고정 HTML의 positive scene 수가 설명과 맞는가 | 저장소에서 `<section class="case">` 19개를 확인했다. **통과** |
| 8 | 계획의 21 runtime case 수가 근거와 맞는가 | Chrome positive 19개와 `auto`/invalid 거부 control 2개로 분리해 센다. **통과** |
| 9 | Ahem 기대값이 font resource 변경에 묶이지 않는가 | Ahem SHA-256과 UPM/advance 계산을 기록하고 현재 파일 hash를 재확인했다. **통과** |
| 10 | label 또는 parent Grid stretch가 target row를 오염시키는가 | 외부 label을 block 흐름에 두고 잘못된 16px 측정은 폐기했다고 명시한다. **통과** |
| 11 | 초기값·inherited·max/min-content·visible hyphen·multi-soft-hyphen controls가 빠지는가 | 고정 표에 각각의 통제 장면과 기대값이 있다. **통과** |
| 12 | HTML fixture의 기대 geometry가 Ahem 계산과 모순되는가 | `none` 40px, `manual` 30px min-content와 fixed 30px 1/2-line 구분을 대조했다. **통과** |
| 13 | PR screenshot이 제품 text painting 증거처럼 보이는가 | PNG는 장면 위치 보조 자료이며 숫자는 DOM geometry, glyph pixel 증거가 아님을 밝힌다. **통과** |
| 14 | 무효 authored keyword가 파서에서 사라져 `manual` 기본값으로 잘못 성공하는가 | parsing 전 원문을 분류하고 미선언과 구분되지 않으면 layout 전에 거부하도록 계획 관문을 보강했다. **통과** |
| 15 | fixture alias가 cascade·상속·CSS-wide keyword를 우회하는가 | 실제 computed custom property를 읽고 selector/order/priority 및 collision·`@property`·`all` 경계를 구현 관문으로 둔다. **통과** |
| 16 | UTF-8 변환 뒤 Node/source mapping이나 revision이 손상되는가 | source NodeId, UTF-8 byte/UTF-16 range, document/style revision 검증을 요구한다. **통과** |
| 17 | 계획 문서가 이미 병합된 C11.4 작업을 미구현이라고 표시하는가 | #974, #909, #867의 merged 상태를 GitHub에서 확인하고 stale plan 문구를 갱신했다. **통과** |
| 18 | 추가한 상대 링크가 존재하지 않거나 잘못된 파일을 가리키는가 | 새 계획·HTML·근거 링크의 대상 파일이 저장소에 존재한다. **통과** |
| 19 | 브라우저·runtime·simulator 검증을 실행한 것처럼 PR 범위를 부풀리는가 | 이 변경은 기존 Agent Browser 사전 관찰을 문서화하며 제품 tests/runtime/simulator는 수행하지 않았다고 구분한다. **통과** |
| 20 | 사용자 WIP나 무관한 파일이 PR에 섞이는가 | stage된 변경은 계획·두 문서 색인·고정 HTML/PNG/README와 두 검토 문서뿐이다. **통과** |

체크는 `git diff --cached --check`, 고정 자원 SHA-256, scene 수, 관련 병합 PR 상태를 확인했다. 제품 테스트는 실행하지 않았다. PR 본문의 screenshot raw link는 branch push 뒤 Agent Browser로 열리는지 확인한 후 병합 관문으로 남긴다.
