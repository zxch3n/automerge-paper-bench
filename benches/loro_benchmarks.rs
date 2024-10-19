use automerge_paper_bench::get_automerge_actions;
use criterion::{black_box, criterion_group, criterion_main, Criterion};
use loro::LoroDoc;
use std::time::Instant;

fn bench_loro(c: &mut Criterion) {
    let actions = get_automerge_actions();
    let start = Instant::now();
    let doc = LoroDoc::new();
    let text = doc.get_text("text");
    for _ in 0..100 {
        for action in &actions {
            if action.del > 0 {
                text.delete(action.pos, action.del).unwrap();
            }
            if !action.ins.is_empty() {
                text.insert(action.pos, &action.ins).unwrap();
            }
        }
    }
    doc.commit();
    println!("Apply time taken: {:?}", start.elapsed());
    let mut group = c.benchmark_group("New Snapshot");
    let snapshot = doc.export(loro::ExportMode::Snapshot).unwrap();
    println!("Snapshot size: {}", snapshot.len());
    bench(&mut group, snapshot);
    group.finish();

    let mut group = c.benchmark_group("Shallow Snapshot");
    let snapshot = doc
        .export(loro::ExportMode::shallow_snapshot_since(
            doc.oplog_frontiers().as_single().unwrap(),
        ))
        .unwrap();
    println!("Shallow Snapshot size: {}", snapshot.len());
    bench(&mut group, snapshot);
    group.finish();
}

fn bench(
    group: &mut criterion::BenchmarkGroup<'_, criterion::measurement::WallTime>,
    snapshot: Vec<u8>,
) {
    group.bench_function("Parse", |b| {
        b.iter(|| {
            let doc = LoroDoc::new();
            doc.import(black_box(&snapshot)).unwrap();
            black_box(doc);
        });
    });
    group.bench_function("Parse+ToString", |b| {
        b.iter(|| {
            let doc = LoroDoc::new();
            doc.import(black_box(&snapshot)).unwrap();
            black_box(doc.get_text("text").to_string());
            black_box(doc);
        });
    });
    group.bench_function("Parse+ToString+Edit", |b| {
        b.iter(|| {
            let doc = LoroDoc::new();
            doc.import(black_box(&snapshot)).unwrap();
            black_box(doc.get_text("text").to_string());
            doc.get_text("text").insert(0, "Hello, world!").unwrap();
            black_box(doc);
        });
    });
    group.bench_function("Parse+ToString+Edit+Export", |b| {
        b.iter(|| {
            let doc = LoroDoc::new();
            doc.import(black_box(&snapshot)).unwrap();
            black_box(doc.get_text("text").to_string());
            doc.get_text("text").insert(0, "Hello, world!").unwrap();
            black_box(doc.export(loro::ExportMode::Snapshot).unwrap());
        });
    });
}

criterion_group!(benches, bench_loro);
criterion_main!(benches);
