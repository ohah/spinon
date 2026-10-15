export class HostProjectionError extends Error {
  constructor(code, message) {
    super(message);
    this.name = "HostProjectionError";
    this.code = code;
  }
}

export function assertOwnerId(ownerId) {
  if (typeof ownerId !== "string" || ownerId.length === 0) {
    throw new HostProjectionError("InvalidOwnerId", "OwnerId는 빈 문자열이 아닌 문자열이어야 합니다");
  }
}

export function assertNodeName(name) {
  if (typeof name !== "string" || !/^[a-z][a-z0-9-]*$/.test(name)) {
    throw new HostProjectionError("InvalidNodeName", `지원하지 않는 요소 이름: ${String(name)}`);
  }
}
