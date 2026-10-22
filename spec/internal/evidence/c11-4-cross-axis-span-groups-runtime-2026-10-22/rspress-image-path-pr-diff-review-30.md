# C11.4 runtime evidence RSPress 이미지 경로 PR diff 실패 관점 검토 30개

**대상:** runtime evidence README의 Android·iOS 임베드 경로, evidence SHA-256 목록, 이 변경의 검토 기록. 첫 RSPress 시도에서 발견한 정적 경로 해석 실패를 고친 최종 diff를 검토했다.

| # | 실패 관점 | diff 판정 |
|---:|---|---|
| 1 | 최초 build 오류가 PNG 파일 부재를 가리키는가, Markdown import 해석을 가리키는가? | Rspack은 파일명으로 시작하는 상대 경로를 모듈로 찾지 못했다. 두 PNG는 source와 선택 root에 모두 있었다. |
| 2 | 수정이 실제로 실패한 Android 경로를 바꾸는가? | Android screenshot URL 앞에 명시적 `./`를 추가했다. |
| 3 | 수정이 실제로 실패한 iOS 경로를 바꾸는가? | iOS screenshot URL 앞에 명시적 `./`를 추가했다. |
| 4 | 이미지가 아닌 다운로드 링크나 다른 evidence 링크를 불필요하게 바꾸는가? | 두 Markdown 이미지 URL만 경로 표기를 고쳤다. |
| 5 | 경로 기준 폴더가 README가 있는 폴더에서 벗어나는가? | `./`가 같은 runtime evidence 디렉터리 내 자산을 가리킨다. |
| 6 | Android 이미지 파일명이 기존 캡처와 달라졌는가? | `definite-minmax-calc-cross-axis-span-groups-forward.png`를 유지한다. |
| 7 | iOS 이미지 파일명이 기존 캡처와 달라졌는가? | 동일 scene 이름의 기존 Simulator PNG를 유지한다. |
| 8 | Android 환경 설명이 alt text에서 빠지는가? | API 37 AVD 및 definite forward 장면 설명을 유지한다. |
| 9 | iOS 환경 설명이나 drawable 한계가 alt text에서 빠지는가? | iPhone 17 Pro/iOS 26.2와 제출 전 Metal 표본 설명을 유지한다. |
| 10 | 이미지 원본 바이트를 수정해 기록된 capture hash를 무효화하는가? | PNG 파일은 수정하지 않았다. |
| 11 | screenshot 경로와 SHA256SUMS 항목 사이가 어긋나는가? | 두 이미지의 기존 manifest 항목 및 hash는 그대로다. |
| 12 | README 본문을 수정하고 기존 README hash를 갱신하지 않는가? | 변경한 README의 SHA-256 항목을 다시 계산했다. |
| 13 | 새 PR diff 검토 파일을 evidence manifest에서 빠뜨리는가? | 이 검토 파일의 SHA-256 항목도 manifest에 추가한다. |
| 14 | manifest가 해시 계산 대상을 다른 알고리즘으로 바꾸는가? | 저장소의 기존 SHA-256 형식과 2-space separator를 유지한다. |
| 15 | 변경하지 않은 evidence 항목의 기존 checksum을 임의로 덮는가? | 변경 전 경로들에 대해 기존 값을 검증하고 두 개의 변경 문서만 갱신한다. |
| 16 | manifest 경로 순서가 불안정해져 diff가 불필요하게 커지는가? | 파일 경로 기준 정렬을 유지한다. |
| 17 | manifest가 자기 자신을 해시해 순환 의존을 만드는가? | `SHA256SUMS.txt` 자체는 목록에 넣지 않는다. |
| 18 | 동일한 README alias의 중복 RSPress 빌드 중 하나만 고쳐지는가? | 최종 선택 root 네 alias에 같은 README를 넣고 각 정적 경로를 빌드했다. |
| 19 | RSPress 빌드 결과가 이미지를 실제 `<img>`로 출력하는가? | 성공 빌드 HTML에 Android와 iOS 각각의 `<img>`가 생성됐다. |
| 20 | 두 이미지가 같은 처리된 자산 URL을 잘못 공유하는가? | 각 원본 hash에 대응하는 별도 `/spinon/static/image/...` 자산을 출력한다. |
| 21 | 고친 경로 뒤에도 module-resolution 오류가 남는가? | 두 번째 RSPress build는 `Pages rendered` 및 web/node 성공으로 끝났다. |
| 22 | 성공 build가 다른 RSPress 버전을 사용해 결과를 왜곡하는가? | 저장소 잠금 버전 RSPress `2.0.22`로 빌드했다. |
| 23 | PR이 unrelated evidence 디렉터리의 미추적 파일을 사이트에 섞는가? | pass47 선택 root는 main의 tracked diff만 적용하고 untracked workspace evidence는 제외한다. |
| 24 | 화면 표본 설명이 iOS compositor 이후 캡처를 주장하는가? | README의 제출 전 drawable 한계를 유지했다. |
| 25 | overflow screenshot이 CSS border stroke 검증으로 확대되는가? | README에서 border stroke 미검증 상태를 유지한다. |
| 26 | box screenshot이 glyph raster 동등성을 입증한다고 읽히는가? | glyph raster 미검증 상태를 유지한다. |
| 27 | `spec/STATUS.md`나 완료 집계에 구현 완료를 추가하는가? | 상태 대장과 `77/213`은 이 PR에서 변경하지 않았다. |
| 28 | public roadmap의 PR #510 링크·근거 경로와 runtime README의 이미지 경로가 서로 어긋나는가? | 공개 노트가 연결한 README 경로는 같고 해당 README 화면 자산을 빌드한다. |
| 29 | 실행 실패 산출물이 성공 로그를 대체하거나 감춰지는가? | 최초 실패와 이미지 경로 수정 후 성공 로그를 별도 파일로 보존한다. |
| 30 | 변경이 runtime 구현이나 simulator 입력을 바꾸는가? | 수정 범위는 evidence 문서, checksum 목록, PR diff 검토에 한정된다. |

**결론:** 두 embedded image 경로의 `./` 누락이 build 실패 원인이었다. 보정 후 RSPress `2.0.22`가 공개 경로용 runtime README를 렌더링했고 Android·iOS PNG를 이미지 요소로 출력했다. 전체 문서 build의 기존 dead-link 한계와 simulator 증거 범위는 바뀌지 않는다.
