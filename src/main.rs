use rand::prelude::*;
use std::fs::OpenOptions;
use std::io::Write;
use std::path::PathBuf;
use structopt::StructOpt;

#[derive(StructOpt)]
#[structopt(name = "pyspewer", about = "Generates random Python code snippets")]
struct Opt {
    /// Output file path
    #[structopt(parse(from_os_str))]
    output: PathBuf,

    /// Number of snippets to generate
    #[structopt(short, long, default_value = "5")]
    count: usize,
}

fn main() {
    let opt = Opt::from_args();

    let mut file = OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(true)
        .open(&opt.output)
        .expect("Failed to open output file");

    let snippets = generate_snippets(opt.count);
    for snippet in snippets {
        writeln!(file, "{}\n", snippet).expect("Failed to write to file");
    }
}

fn generate_snippets(count: usize) -> Vec<String> {
    let mut rng = rand::thread_rng();
    let mut snippets = Vec::new();

    for _ in 0..count {
        let snippet = match rng.gen_range(0..5) {
            0 => generate_function_snippet(&mut rng),
            1 => generate_list_comprehension(&mut rng),
            2 => generate_class_snippet(&mut rng),
            3 => generate_loop_snippet(&mut rng),
            _ => generate_conditional_snippet(&mut rng),
        };
        snippets.push(snippet);
    }

    snippets
}

fn generate_function_snippet(rng: &mut ThreadRng) -> String {
    let function_names = [
        "process_data",
        "calculate_sum",
        "transform_text",
        "analyze_numbers",
    ];
    let name = function_names[rng.gen_range(0..function_names.len())];
    format!(
        "def {}(data):\n    return [x * {} for x in data if x > {}]",
        name,
        rng.gen_range(2..10),
        rng.gen_range(0..5)
    )
}

fn generate_list_comprehension(rng: &mut ThreadRng) -> String {
    format!(
        "numbers = [x ** {} for x in range({}, {}) if x % {} == 0]",
        rng.gen_range(2..4),
        rng.gen_range(0..10),
        rng.gen_range(20..50),
        rng.gen_range(2..5)
    )
}

fn generate_class_snippet(rng: &mut ThreadRng) -> String {
    let class_names = [
        "DataProcessor",
        "NumberCruncher",
        "TextAnalyzer",
        "Calculator",
    ];
    let name = class_names[rng.gen_range(0..class_names.len())];
    format!(
        "class {}:\n    def __init__(self):\n        self.value = {}\n\n    def process(self, x):\n        return x * self.value",
        name,
        rng.gen_range(1..10)
    )
}

fn generate_loop_snippet(rng: &mut ThreadRng) -> String {
    format!(
        "total = 0\nfor i in range({}):\n    if i % {} == 0:\n        total += i ** {}",
        rng.gen_range(10..20),
        rng.gen_range(2..5),
        rng.gen_range(2..4)
    )
}

fn generate_conditional_snippet(rng: &mut ThreadRng) -> String {
    format!(
        "value = {}\nif value > {}:\n    result = value * {}\nelif value < {}:\n    result = value / {}\nelse:\n    result = value",
        rng.gen_range(1..100),
        rng.gen_range(50..80),
        rng.gen_range(2..5),
        rng.gen_range(20..40),
        rng.gen_range(2..4)
    )
}
