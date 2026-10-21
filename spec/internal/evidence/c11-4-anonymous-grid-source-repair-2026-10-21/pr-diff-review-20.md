# PR diff 적대 검토 20개

범위는 `text_paragraphs.rs`가 선언·호출하는 누락 내부 모듈과 검증 근거다. 각 관점에서 독립된 입력·실패 경로를 확인했다.

1. **모듈 경로 불일치** — `#[path = "text_paragraphs/dom_traversal.rs"]`의 상대 경로에 파일이 놓였는가? 예, clean snapshot에서 동일 위치에 추가한 뒤 Cargo가 모듈을 읽었다.
2. **빈 파일 또는 선언만 복구** — 세 함수 본문이 모두 있는가? `collect_host_handles`, `collect_hidden_layout_nodes`, `is_hidden_by_ancestor`가 정의되어 있다.
3. **가시성 경계** — 다른 패키지에 불필요하게 공개됐는가? 세 함수 모두 `pub(super)`이고 내부 helper 계약을 유지한다.
4. **타입/import 연결** — `HostParent`, `NodeId`, `LayoutStyle` 등의 실제 타입과 연결되는가? `cargo check --locked -p spinon-layout`이 깨끗한 스냅샷에서 통과했다.
5. **깨끗한 체크아웃 재현** — 미추적 로컬 파일에 의존하는가? `origin/main` archive만으로는 실패했고, 누락 모듈을 추가한 archive에서는 통과했다.
6. **루트 포함 여부** — 수집 시작 노드가 결과에서 빠지는가? 첫 pop 시 `handles`에 삽입한다.
7. **넓은 트리 방문** — 한 자식 경로만 따라가거나 형제를 건너뛰는가? 각 노드의 전체 child iterator를 stack에 추가한다.
8. **중복 ID와 순환 입력** — 무한 순회하거나 중복 값을 덮어쓰는가? 중복 `NodeId` 삽입을 즉시 `DuplicateNode`로 거부한다.
9. **stale handle** — snapshot에 없는 handle을 역참조하는가? `snapshot.node` 실패를 `InvalidHostDocumentRoot`로 변환한다.
10. **text leaf 경계** — 텍스트 leaf의 정상적인 자식 없음과 비-text 노드의 비정상 자식 없음이 구분되는가? Text만 자식 없는 leaf로 허용한다.
11. **빈 element child list** — 빈 element를 잘못 leaf 오류로 처리하는가? `Some(empty)` child iterator는 정상적으로 순회를 끝낸다.
12. **숨김 element style 누락** — `display:none` 하위에서 없는 스타일을 조용히 통과하는가? 각 element마다 `MissingStyle`을 반환한다.
13. **숨김 root 제외** — 숨김 subtree 시작 element가 제외 집합에서 빠지는가? descendants 확장 전에 현재 element를 삽입한다.
14. **숨김 깊은 자손** — 2단계 이상 element 자손이 남는가? 매 element에서 element child들을 stack에 반복 추가한다.
15. **text 노드 분류** — 텍스트를 layout element 제외 집합에 넣는가? child 확장 시 element만 선택하므로 텍스트 노드는 제외 대상이 아니다.
16. **현재 노드 display 확인** — candidate 자체가 `display:none`일 때 놓치는가? ancestor loop의 첫 반복에서 현재 handle style을 검사한다.
17. **숨김 조상 전파** — 중간 조상이 숨김일 때 표시 자손을 잘못 보이는가? root까지 각 parent의 style을 검사하고 첫 `None`에서 true를 반환한다.
18. **지정 root 경계** — root 밖의 조상 상태가 하위 결과를 바꾸는가? root style을 확인한 뒤 parent 조회 전에 종료한다.
19. **잘린 parent chain** — `HostParent::Root` 또는 parent 없음에서 무한 반복하는가? 둘 다 false로 종료한다.
20. **공식 상태와 검증 주장** — 수정만으로 C11.4 전체 완료·실기기·GPU 호환을 과장하는가? `spec/STATUS.md`는 C11.4 미완료를 유지하고, evidence는 Android API 37 AVD만 기록한다. 새 모듈에 `unwrap`/unsafe 없이 오류를 `Result`로 전파한다.

**결론:** clean checkout 빌드를 막던 파일 누락을 확인하고 이 PR에 추가했다. 상태 대장과 증거 범위는 C11.4의 미완료 상태를 유지한다. 포맷, 229개 layout 테스트, workspace Clippy, Android AVD의 5개 장면에서 추가 실패는 발견되지 않았다. 이 검토는 전체 CSS Grid 적합성이나 물리 GPU 렌더링 검증을 의미하지 않는다.
