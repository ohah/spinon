use spinon_core::TaskPriority;
use std::collections::VecDeque;

const AGING_INTERVAL: usize = 8;
const WEIGHTED_CYCLE: [usize; 13] = [0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 1, 1, 2];
const HORIZON: usize = 4_096;

#[derive(Clone, Copy, Debug)]
struct Task {
    priority: TaskPriority,
    sequence: usize,
    enqueued_at: usize,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum Policy {
    StrictPriority,
    Aging { interval: usize },
    WeightedRoundRobin,
}

#[derive(Debug, Eq, PartialEq)]
struct Metrics {
    service_counts: [usize; 3],
    first_background_turn: Option<usize>,
    maximum_same_priority_run: usize,
}

#[test]
fn compares_starvation_and_service_shares_under_identical_sustained_arrivals() {
    let strict = simulate(Policy::StrictPriority, HORIZON);
    let aging = simulate(
        Policy::Aging {
            interval: AGING_INTERVAL,
        },
        HORIZON,
    );
    let weighted = simulate(Policy::WeightedRoundRobin, HORIZON);

    println!("strict_priority={strict:?}");
    println!("aging_interval_{AGING_INTERVAL}={aging:?}");
    println!("weighted_8_4_1={weighted:?}");

    assert_eq!(strict.service_counts, [HORIZON, 0, 0]);
    assert_eq!(strict.first_background_turn, None);
    assert_eq!(strict.maximum_same_priority_run, HORIZON);

    assert_eq!(aging.service_counts, [3_405, 451, 240]);
    assert_eq!(aging.first_background_turn, Some(17));
    assert_eq!(aging.maximum_same_priority_run, 8);

    assert_eq!(weighted.service_counts, [2_521, 1_260, 315]);
    assert_eq!(weighted.first_background_turn, Some(13));
    assert_eq!(weighted.maximum_same_priority_run, 8);
}

#[test]
fn aging_with_an_interval_longer_than_the_horizon_is_a_starvation_negative_control() {
    let disabled_aging = simulate(
        Policy::Aging {
            interval: HORIZON + 1,
        },
        HORIZON,
    );

    assert_eq!(disabled_aging.service_counts, [HORIZON, 0, 0]);
    assert_eq!(disabled_aging.first_background_turn, None);
}

#[test]
fn weighted_round_robin_skips_empty_classes_and_keeps_its_cycle_position() {
    let mut queues: [VecDeque<Task>; 3] = std::array::from_fn(|_| VecDeque::new());
    let mut next_sequence = 0;
    enqueue(
        &mut queues,
        &mut next_sequence,
        TaskPriority::UserBlocking,
        0,
    );
    enqueue(&mut queues, &mut next_sequence, TaskPriority::Background, 0);

    let mut cursor = 0;
    let mut selected = Vec::new();
    for turn in 0..9 {
        let (queue_index, selected_slot) = weighted_queue(&queues, cursor);
        cursor = (selected_slot + 1) % WEIGHTED_CYCLE.len();
        let task = queues[queue_index]
            .pop_front()
            .expect("가중 순환이 선택한 큐에는 작업이 있어야 합니다");
        selected.push(task.priority);
        enqueue(&mut queues, &mut next_sequence, task.priority, turn + 1);
    }

    assert_eq!(
        selected,
        [
            TaskPriority::UserBlocking,
            TaskPriority::UserBlocking,
            TaskPriority::UserBlocking,
            TaskPriority::UserBlocking,
            TaskPriority::UserBlocking,
            TaskPriority::UserBlocking,
            TaskPriority::UserBlocking,
            TaskPriority::UserBlocking,
            TaskPriority::Background,
        ]
    );
}

fn simulate(policy: Policy, horizon: usize) -> Metrics {
    let mut queues: [VecDeque<Task>; 3] = std::array::from_fn(|_| VecDeque::new());
    let mut next_sequence = 0;
    for priority in [
        TaskPriority::Background,
        TaskPriority::UserVisible,
        TaskPriority::UserBlocking,
    ] {
        enqueue(&mut queues, &mut next_sequence, priority, 0);
    }

    let mut metrics = Metrics {
        service_counts: [0; 3],
        first_background_turn: None,
        maximum_same_priority_run: 0,
    };
    let mut weighted_cursor = 0;
    let mut previous_priority = None;
    let mut same_priority_run = 0;

    for turn in 0..horizon {
        let queue_index = match policy {
            Policy::StrictPriority => first_nonempty_queue(&queues),
            Policy::Aging { interval } => aging_queue(&queues, turn, interval),
            Policy::WeightedRoundRobin => {
                let (index, selected_slot) = weighted_queue(&queues, weighted_cursor);
                weighted_cursor = (selected_slot + 1) % WEIGHTED_CYCLE.len();
                index
            }
        };
        let task = queues[queue_index]
            .pop_front()
            .expect("선택된 큐에는 작업이 있어야 합니다");
        let priority_index = priority_index(task.priority);
        metrics.service_counts[priority_index] += 1;
        if task.priority == TaskPriority::Background && metrics.first_background_turn.is_none() {
            metrics.first_background_turn = Some(turn + 1);
        }

        if previous_priority == Some(task.priority) {
            same_priority_run += 1;
        } else {
            previous_priority = Some(task.priority);
            same_priority_run = 1;
        }
        metrics.maximum_same_priority_run =
            metrics.maximum_same_priority_run.max(same_priority_run);

        enqueue(&mut queues, &mut next_sequence, task.priority, turn + 1);
    }

    metrics
}

fn enqueue(
    queues: &mut [VecDeque<Task>; 3],
    next_sequence: &mut usize,
    priority: TaskPriority,
    enqueued_at: usize,
) {
    let queue_index = priority_index(priority);
    queues[queue_index].push_back(Task {
        priority,
        sequence: *next_sequence,
        enqueued_at,
    });
    *next_sequence += 1;
}

fn first_nonempty_queue(queues: &[VecDeque<Task>; 3]) -> usize {
    queues
        .iter()
        .position(|queue| !queue.is_empty())
        .expect("세 우선순위 큐가 계속 runnable 상태여야 합니다")
}

fn aging_queue(queues: &[VecDeque<Task>; 3], turn: usize, interval: usize) -> usize {
    assert!(interval > 0, "aging interval은 0보다 커야 합니다");

    queues
        .iter()
        .enumerate()
        .filter_map(|(queue_index, queue)| {
            queue.front().map(|task| {
                let rank = priority_index(task.priority);
                let waited = turn.saturating_sub(task.enqueued_at);
                let effective_rank = rank.saturating_sub(waited / interval);
                ((effective_rank, task.sequence), queue_index)
            })
        })
        .min_by_key(|(key, _)| *key)
        .map(|(_, queue_index)| queue_index)
        .expect("세 우선순위 큐가 계속 runnable 상태여야 합니다")
}

fn weighted_queue(queues: &[VecDeque<Task>; 3], cursor: usize) -> (usize, usize) {
    (0..WEIGHTED_CYCLE.len())
        .map(|offset| {
            let slot = (cursor + offset) % WEIGHTED_CYCLE.len();
            (WEIGHTED_CYCLE[slot], slot)
        })
        .find(|(queue_index, _)| !queues[*queue_index].is_empty())
        .expect("세 우선순위 큐가 계속 runnable 상태여야 합니다")
}

const fn priority_index(priority: TaskPriority) -> usize {
    match priority {
        TaskPriority::UserBlocking => 0,
        TaskPriority::UserVisible => 1,
        TaskPriority::Background => 2,
    }
}
