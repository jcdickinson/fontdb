use criterion::{black_box, criterion_group, criterion_main, Criterion};
use std::time::Instant;

pub fn get_system_fonts(c: &mut Criterion) {
    c.bench_function("load system fonts", move |b| {
        b.iter_custom(move |iters| {
            let start = Instant::now();
            let mut len = 1;
            for _ in 0..iters {
                let mut db = fontdb::Database::new();
                black_box(db.load_system_fonts());
                len = db.len() as u32;
            }
            let elapsed = start.elapsed();

            // Consider two approaches. Approach A results in 100 fonts, which
            // is an incomplete set of fonts on the system. Approach B results in
            // 200 fonts, which is the complete set of fonts. Approach B *will*
            // be slower than A by virtue of simply loading more fonts. This
            // approximately levels the playing field.
            elapsed / len.max(1)
        })
    });
}

criterion_group!(benches, get_system_fonts);
criterion_main!(benches);
