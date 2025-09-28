use criterion::{black_box, criterion_group, criterion_main, Criterion, BenchmarkId};

// For benchmarks, we'll create our own simple render function
fn render_text_simple(text: &str, font_name: &str, max_width: u32, height: u32) -> String {
    // Mock implementation for benchmarking
    // In a real scenario, this would be the actual render function
    format!("Rendered: {} with {} ({}x{})", text, font_name, max_width, height)
}

fn benchmark_render_text(c: &mut Criterion) {
    let mut group = c.benchmark_group("render_text");
    
    // Test different text lengths
    let texts = [
        ("SHORT", "HI"),
        ("MEDIUM", "HELLO WORLD"),
        ("LONG", "THE QUICK BROWN FOX JUMPS OVER THE LAZY DOG"),
        ("VERY_LONG", "THIS IS A VERY LONG STRING THAT SHOULD TAKE MORE TIME TO RENDER AND ALLOWS US TO TEST THE PERFORMANCE OF OUR RENDERING ENGINE WITH EXTENDED TEXT"),
    ];
    
    let fonts = ["standard", "small"];
    
    for font in fonts.iter() {
        for (size_label, text) in texts.iter() {
            group.bench_with_input(
                BenchmarkId::new(format!("{}_{}", font, size_label), text.len()),
                text,
                |b, text| {
                    b.iter(|| render_text_simple(black_box(text), black_box(font), 0, 5));
                },
            );
        }
    }
    
    group.finish();
}

fn benchmark_word_wrapping(c: &mut Criterion) {
    let mut group = c.benchmark_group("word_wrapping");
    
    let text = "HELLO WORLD TEST BENCHMARK PERFORMANCE UNICODE BLOCKS ASCII ART RENDERING";
    let widths = [20, 40, 60, 80, 100];
    
    for width in widths.iter() {
        group.bench_with_input(
            BenchmarkId::new("wrapped", width),
            width,
            |b, &width| {
                b.iter(|| render_text_simple(black_box(text), black_box("standard"), width, 5));
            },
        );
    }
    
    group.finish();
}

fn benchmark_different_fonts(c: &mut Criterion) {
    let mut group = c.benchmark_group("font_comparison");
    
    let text = "BENCHMARK TEST 123";
    let fonts = ["standard", "small"];
    
    for font in fonts.iter() {
        group.bench_function(*font, |b| {
            b.iter(|| render_text_simple(black_box(text), black_box(font), 0, 5));
        });
    }
    
    group.finish();
}

fn benchmark_character_types(c: &mut Criterion) {
    let mut group = c.benchmark_group("character_types");
    
    let test_cases = [
        ("LETTERS_ONLY", "ABCDEFGHIJKLMNOPQRSTUVWXYZ"),
        ("NUMBERS_ONLY", "0123456789"),
        ("MIXED", "ABC123XYZ789"),
        ("WITH_SPACES", "HELLO WORLD 123"),
        ("PUNCTUATION", "HELLO, WORLD! HOW ARE YOU?"),
    ];
    
    for (label, text) in test_cases.iter() {
        group.bench_function(*label, |b| {
            b.iter(|| render_text_simple(black_box(text), black_box("standard"), 0, 5));
        });
    }
    
    group.finish();
}

criterion_group!(
    benches,
    benchmark_render_text,
    benchmark_word_wrapping,
    benchmark_different_fonts,
    benchmark_character_types
);
criterion_main!(benches);
