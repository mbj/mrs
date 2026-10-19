//! Real IO adds a syscall per read, which an in-memory source does not, so
//! `bytes/read` matters as much as the timing.

use core::future::Future;
use core::pin::pin;
use core::sync::atomic::{AtomicUsize, Ordering};
use core::task::{Context, Poll, Waker};
use std::alloc::{GlobalAlloc, Layout, System};
use std::collections::VecDeque;
use std::time::Instant;

struct Counting;

static ALLOCATIONS: AtomicUsize = AtomicUsize::new(0);

unsafe impl GlobalAlloc for Counting {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        ALLOCATIONS.fetch_add(1, Ordering::Relaxed);

        unsafe { System.alloc(layout) }
    }

    unsafe fn dealloc(&self, pointer: *mut u8, layout: Layout) {
        unsafe { System.dealloc(pointer, layout) }
    }
}

#[global_allocator]
static ALLOCATOR: Counting = Counting;

use pg_client::protocol::{self, Frame, Reader, ReceiveBuffer, ReceiveBufferSize};

const VOLUME: usize = 64 * 1024 * 1024;

const SAMPLES: usize = 5;

struct Source<'a> {
    bytes: &'a [u8],
    position: usize,
    reads: usize,
}

impl<'a> Source<'a> {
    fn new(bytes: &'a [u8]) -> Self {
        Self {
            bytes,
            position: 0,
            reads: 0,
        }
    }

    fn exhausted(&self) -> bool {
        self.position == self.bytes.len()
    }
}

impl Reader for Source<'_> {
    async fn read<'a>(&'a mut self, buffer: &'a mut Vec<u8>) -> std::io::Result<usize> {
        let count = (self.bytes.len() - self.position).min(buffer.capacity() - buffer.len());

        buffer.extend_from_slice(&self.bytes[self.position..self.position + count]);

        self.position += count;
        self.reads += 1;

        Ok(count)
    }
}

fn block_on<F: Future>(future: F) -> F::Output {
    match pin!(future).poll(&mut Context::from_waker(Waker::noop())) {
        Poll::Ready(value) => value,
        Poll::Pending => panic!("benchmark source never pends"),
    }
}

fn stream(count: usize, body: usize) -> Vec<u8> {
    let body = vec![0x2a; body];
    let mut bytes = Vec::with_capacity(count * (body.len() + protocol::HEADER_SIZE));

    for _ in 0..count {
        protocol::encode(b'D', &body, &mut bytes);
    }

    bytes
}

const MIXED_VOLUME: usize = 128 * 1024 * 1024;

const NARROW: usize = 100;
const WIDE: usize = 96 * 1024;
const JUMBO: usize = 1024 * 1024;

struct Random(u64);

impl Random {
    fn next(&mut self) -> u64 {
        self.0 = self
            .0
            .wrapping_mul(6_364_136_223_846_793_005)
            .wrapping_add(1_442_695_040_888_963_407);

        self.0 >> 33
    }
}

fn mixed(weights: &[(u64, usize)]) -> (Vec<u8>, usize) {
    let bodies: Vec<Vec<u8>> = weights.iter().map(|(_, size)| vec![0x2a; *size]).collect();

    let mut random = Random(0x5eed);
    let mut bytes = Vec::with_capacity(MIXED_VOLUME + JUMBO);
    let mut count = 0;

    while bytes.len() < MIXED_VOLUME {
        let mut pick = random.next() % 10_000;
        let mut index = 0;

        for (weight, _) in weights {
            if pick < *weight {
                break;
            }

            pick -= *weight;
            index += 1;
        }

        protocol::encode(b'D', &bodies[index.min(bodies.len() - 1)], &mut bytes);

        count += 1;
    }

    (bytes, count)
}

#[derive(Clone, Copy)]
struct Retention {
    label: &'static str,
    rate: u64,
    window: usize,
}

struct Sample {
    seconds: f64,
    reads: usize,
    allocations: usize,
}

fn sample(bytes: &[u8], size: ReceiveBufferSize, retention: Retention) -> Sample {
    let mut source = Source::new(bytes);
    let mut buffer = ReceiveBuffer::new(size);
    let mut held: VecDeque<Frame> = VecDeque::with_capacity(retention.window.min(1024) + 1);
    let mut random = Random(0xd15ea5e);

    let allocations = ALLOCATIONS.load(Ordering::Relaxed);
    let start = Instant::now();

    while !source.exhausted() {
        let frame = block_on(buffer.read_frame(&mut source)).unwrap();

        if random.next() % 10_000 < retention.rate {
            held.push_back(frame);

            if held.len() > retention.window {
                held.pop_front();
            }
        } else {
            core::hint::black_box(frame);
        }
    }

    let seconds = start.elapsed().as_secs_f64();
    let allocations = ALLOCATIONS.load(Ordering::Relaxed) - allocations;

    drop(held);

    Sample {
        seconds,
        reads: source.reads,
        allocations,
    }
}

fn median(values: &mut [f64]) -> f64 {
    values.sort_by(f64::total_cmp);

    values[values.len() / 2]
}

fn report(
    shape: &str,
    bytes: &[u8],
    count: usize,
    label: &str,
    size: ReceiveBufferSize,
    retention: Retention,
) {
    let warmup = sample(bytes, size, retention);

    let mut seconds: Vec<f64> = (0..SAMPLES)
        .map(|_| sample(bytes, size, retention).seconds)
        .collect();

    let low = seconds.iter().copied().fold(f64::MAX, f64::min);
    let high = seconds.iter().copied().fold(f64::MIN, f64::max);
    let median = median(&mut seconds);

    println!(
        "{shape:>9}  {label:>9}  {:>7}  {:>10.1}  {:>7.0}%  {:>11.0}  {:>10.2}  {:>9.0}",
        retention.label,
        median * 1e9 / count as f64,
        (high - low) / median * 100.0,
        bytes.len() as f64 / warmup.reads as f64,
        warmup.allocations as f64 / warmup.reads as f64,
        bytes.len() as f64 / median / (1024.0 * 1024.0),
    );
}

fn header() {
    println!(
        "\n{:>9}  {:>9}  {:>7}  {:>10}  {:>8}  {:>11}  {:>10}  {:>9}",
        "body", "buffer", "hold", "ns/frame", "spread", "bytes/read", "alloc/read", "MiB/s"
    );
}

const NONE: Retention = Retention {
    label: "none",
    rate: 0,
    window: 0,
};

/// What production looks like.
const RARE: Retention = Retention {
    label: "0.1%",
    rate: 10,
    window: 4,
};

const SOME: Retention = Retention {
    label: "1%",
    rate: 100,
    window: 16,
};

const OFTEN: Retention = Retention {
    label: "10%",
    rate: 1_000,
    window: 64,
};

const ALL: Retention = Retention {
    label: "all",
    rate: 10_000,
    window: usize::MAX,
};

fn main() {
    let sizes = [
        (
            "8K/512K",
            ReceiveBufferSize::new(8 * 1024, 512 * 1024).unwrap(),
        ),
        ("64K/512K", ReceiveBufferSize::DEFAULT),
        (
            "256K/1M",
            ReceiveBufferSize::new(256 * 1024, 1024 * 1024).unwrap(),
        ),
    ];

    header();

    for body in [100, 16 * 1024] {
        let count = (VOLUME / (body + protocol::HEADER_SIZE)).clamp(64, 200_000);
        let bytes = stream(count, body);
        let shape = body.to_string();

        for (label, size) in sizes {
            for retention in [NONE, RARE, ALL] {
                report(&shape, &bytes, count, label, size, retention);
            }
        }
    }

    let shapes = [
        ("no-jumbo", mixed(&[(9_500, NARROW), (500, WIDE)])),
        ("jumbo", mixed(&[(9_500, NARROW), (495, WIDE), (5, JUMBO)])),
    ];

    let budgets = [
        (
            "64K/64K",
            ReceiveBufferSize::new(64 * 1024, 64 * 1024).unwrap(),
        ),
        (
            "64K/128K",
            ReceiveBufferSize::new(64 * 1024, 128 * 1024).unwrap(),
        ),
        (
            "64K/2M",
            ReceiveBufferSize::new(64 * 1024, 2 * 1024 * 1024).unwrap(),
        ),
    ];

    for (shape, (bytes, count)) in &shapes {
        header();

        for (label, size) in budgets {
            for retention in [NONE, RARE, SOME, OFTEN, ALL] {
                report(shape, bytes, *count, label, size, retention);
            }
        }
    }
}
