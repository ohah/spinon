/// 내부 layout 경로에서 전달하는 제한된 writing mode 값입니다.
///
/// 이 타입은 CSS parsing이나 일반 writing-mode 지원 범위를 정의하지 않습니다.
/// 지원하지 않는 computed 값은 값을 소비하는 profile에서 명시적으로 거부해야 합니다.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum WritingMode {
    /// 위에서 아래로 block이 진행하고 inline은 가로로 진행합니다.
    HorizontalTb,
    /// 오른쪽에서 왼쪽으로 block이 진행하고 inline은 세로로 진행합니다.
    VerticalRl,
}

/// 내부 layout 경로에서 전달하는 제한된 text orientation 값입니다.
///
/// 이 타입은 CSS parsing이나 glyph painting을 정의하지 않습니다.
/// 지원하지 않는 computed 값은 값을 소비하는 profile에서 명시적으로 거부해야 합니다.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum TextOrientation {
    /// 세로 writing mode에서 문맥에 따라 glyph 방향을 정하는 값입니다.
    Mixed,
}
