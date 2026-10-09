const SIGNAL_STEPS = ['SIGINT', 'SIGTERM', 'SIGKILL'];

export function parseGeteventLine(line) {
  const match = line.match(/^\[\s*(\d+)\.(\d{1,9})\]\s+EV_(\w+)\s+(\S+)\s+(\S+)/);
  if (!match) return null;

  const [, seconds, fraction, type, code, rawValue] = match;
  const nanosecondFraction = BigInt((fraction + '000000000').slice(0, 9));
  const timestampNs = BigInt(seconds) * 1_000_000_000n + nanosecondFraction;
  let value;
  if (rawValue.toLowerCase() === 'ffffffff' || rawValue === '-1') value = -1;
  else if (/^[0-9a-f]+$/i.test(rawValue)) value = Number.parseInt(rawValue, 16);
  else value = rawValue;

  return { timestampNs, type, code, rawValue, value };
}

export class RawContactCounter {
  #activeBySlot = new Map();
  #currentSlot = 0;
  #pendingCompletions = 0;
  #completed = 0;
  #overlapReports = 0;
  #protocolErrors = [];

  get completedContacts() { return this.#completed; }
  get activeContacts() { return this.#activeBySlot.size; }
  get pendingCompletions() { return this.#pendingCompletions; }
  get overlapReports() { return this.#overlapReports; }
  get protocolErrors() { return [...this.#protocolErrors]; }

  consume(line) {
    const event = parseGeteventLine(line);
    if (!event) return { event: null, firstContact: false, completedContacts: this.#completed };

    let firstContact = false;
    if (event.type === 'ABS' && event.code === 'ABS_MT_SLOT' && typeof event.value === 'number') {
      this.#currentSlot = event.value;
    } else if (event.type === 'ABS' && event.code === 'ABS_MT_TRACKING_ID' && typeof event.value === 'number') {
      if (event.value === -1) {
        if (this.#activeBySlot.delete(this.#currentSlot)) this.#pendingCompletions += 1;
        else this.#protocolErrors.push(`release_without_active_contact:${this.#currentSlot}:${event.timestampNs}`);
      } else {
        firstContact = true;
        if (this.#activeBySlot.has(this.#currentSlot)) {
          this.#protocolErrors.push(`tracking_id_replaced_without_release:${this.#currentSlot}:${event.timestampNs}`);
          this.#activeBySlot.delete(this.#currentSlot);
        }
        this.#activeBySlot.set(this.#currentSlot, event.value);
        if (this.#activeBySlot.size > 1) this.#overlapReports += 1;
      }
    } else if (event.type === 'SYN' && event.code === 'SYN_REPORT' && this.#pendingCompletions > 0) {
      this.#completed += this.#pendingCompletions;
      this.#pendingCompletions = 0;
    }

    return { event, firstContact, completedContacts: this.#completed };
  }
}

export class CaptureDeadline {
  #firstContactAt = null;

  constructor({ startAt, noContactMs = 60_000, afterFirstContactMs = 60_000 }) {
    this.startAt = startAt;
    this.noContactMs = noContactMs;
    this.afterFirstContactMs = afterFirstContactMs;
  }

  noteContact(at) {
    if (this.#firstContactAt === null) this.#firstContactAt = at;
  }

  get firstContactAt() { return this.#firstContactAt; }

  reasonAt(now) {
    if (this.#firstContactAt === null) {
      return now - this.startAt >= this.noContactMs ? 'prestart_no_contact' : null;
    }
    return now - this.#firstContactAt >= this.afterFirstContactMs ? 'timeout_after_first_contact' : null;
  }
}

export function captureWindowIsValid({
  stopReason,
  completedContacts,
  contactLimit,
  captureError,
  activeContacts,
  pendingReleaseFrames,
  protocolErrors,
}) {
  return stopReason === 'contact_limit'
    && completedContacts >= contactLimit
    && !captureError
    && activeContacts === 0
    && pendingReleaseFrames === 0
    && protocolErrors.length === 0;
}

function sameIdentity(expected, actual) {
  return actual !== null
    && expected.pid === actual.pid
    && expected.parentPid === actual.parentPid
    && expected.startedAt === actual.startedAt
    && expected.command === actual.command;
}

export async function stopChildBounded(child, expectedIdentity, {
  readIdentity,
  sendSignal = (signal) => child.kill(signal),
  waitForExit,
  stepTimeoutMs = 2_000,
} = {}) {
  if (typeof readIdentity !== 'function' || typeof waitForExit !== 'function') {
    throw new TypeError('process identity and bounded wait functions are required');
  }

  const escalation = [];
  for (const signal of SIGNAL_STEPS) {
    if (await waitForExit(child, 0)) {
      return { exited: true, escalation, exitedBeforeSignal: escalation.length === 0 };
    }

    const actualIdentity = await readIdentity(expectedIdentity.pid);
    if (!sameIdentity(expectedIdentity, actualIdentity)) {
      throw new Error(`collector process identity changed; refusing ${signal}`);
    }

    sendSignal(signal);
    escalation.push(signal);
    if (await waitForExit(child, stepTimeoutMs)) {
      return { exited: true, escalation, exitedBeforeSignal: false };
    }
  }

  throw new Error('collector did not exit after SIGKILL bounded wait');
}

export const signalSteps = SIGNAL_STEPS;
