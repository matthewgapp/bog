use rand::prelude::*;
use std::fs::OpenOptions;
use std::io::Write;
use std::path::PathBuf;
use structopt::StructOpt;

// POV: You're using Python in 2024 🤡
#[derive(StructOpt)]
#[structopt(
    name = "pyspewer",
    about = "haha python go brrr 🐍 (generates cursed code snippets)"
)]
struct Opt {
    /// Where to dump our masterpiece 📝
    #[structopt(parse(from_os_str))]
    output: PathBuf,

    /// How many crimes against Python we're committing today
    #[structopt(short, long, default_value = "5")]
    count: usize,
}

fn main() {
    // Time to create some skill issues
    let opt = Opt::from_args();

    // YOLO error handling because we're building Different™️
    let mut file = OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(true)
        .open(&opt.output)
        .expect("fs machine broke 💀");

    let snippets = generate_snippets(opt.count);
    for snippet in snippets {
        writeln!(file, "{}\n", snippet).expect("write_ln go bye bye");
    }
}

fn generate_snippets(count: usize) -> Vec<String> {
    let mut rng = rand::thread_rng();
    let mut snippets = Vec::with_capacity(count); // we optimize in this house 😤

    for _ in 0..count {
        let snippet = match rng.gen_range(0..5) {
            0 => generate_function_snippet(&mut rng), // functions are just spicy variables
            1 => generate_list_comprehension(&mut rng), // list comprehension because we're ~fancy~
            2 => generate_class_snippet(&mut rng),    // OOP because we hate ourselves
            3 => generate_loop_snippet(&mut rng),     // loops go brrr
            _ => generate_conditional_snippet(&mut rng), // if statements are just spicy match
        };
        snippets.push(snippet);
    }
    snippets
}

fn generate_function_snippet(rng: &mut ThreadRng) -> String {
    let function_names = [
        "yeet_data",
        "stonks_calculator",
        "text_yeeter",
        "number_go_brrr",
    ];
    let name = function_names[rng.gen_range(0..function_names.len())];
    format!(
        "def {}(data): # type hints? we don't do that here 🤪\n    return [x * {} for x in data if x > {}]  # because why not",
        name,
        rng.gen_range(2..10),
        rng.gen_range(0..5)
    )
}

fn generate_list_comprehension(rng: &mut ThreadRng) -> String {
    format!(
        "# POV: You're trying to be clever\nnumbers = [x ** {} for x in range({}, {}) if x % {} == 0]  # big brain time 🧠",
        rng.gen_range(2..4),
        rng.gen_range(0..10),
        rng.gen_range(20..50),
        rng.gen_range(2..5)
    )
}

fn generate_class_snippet(rng: &mut ThreadRng) -> String {
    let class_names = ["DataYeeter", "NumberGoBrrr", "TextChonker", "MathDoer"];
    let name = class_names[rng.gen_range(0..class_names.len())];
    format!(
        "class {}: # because everything needs to be an object 🙄\n    def __init__(self):\n        self.value = {}  # magic number go brrr\n\n    def yeet(self, x):\n        return x * self.value  # mathematics™️",
        name,
        rng.gen_range(1..10)
    )
}

fn generate_loop_snippet(rng: &mut ThreadRng) -> String {
    format!(
        "# Time complexity? Never heard of her 💅\ntotal = 0\nfor i in range({}):\n    if i % {} == 0:\n        total += i ** {}  # numbers go zoom",
        rng.gen_range(10..20),
        rng.gen_range(2..5),
        rng.gen_range(2..4)
    )
}

fn generate_conditional_snippet(rng: &mut ThreadRng) -> String {
    format!(
        "# POV: You're writing Java in Python\nvalue = {}\nif value > {}:  # big if true\n    result = value * {}\nelif value < {}:  # elif because 'else if' was too readable\n    result = value / {}\nelse:  # what even is pattern matching\n    result = value",
        rng.gen_range(1..100),
        rng.gen_range(50..80),
        rng.gen_range(2..5),
        rng.gen_range(20..40),
        rng.gen_range(2..4)
    )
}
