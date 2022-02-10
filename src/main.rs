use clap::Parser;
use rand::distributions::WeightedIndex;
use rand::prelude::*;

#[derive(Parser, Debug)]
struct Args {
    /// Length of the keysmash to generate
    #[clap(short, long)]
    length: Option<usize>,
    /// Optional lexicon of characters to use
    #[clap(long)]
    choices: Option<String>,
}

fn main() {
    let args = Args::parse();
    let mut rng = thread_rng();
    let len = args.length.unwrap_or_else(|| rng.gen_range(5..15));
    println!("args: {:?}, len {}", args, len);

    let result: String = if let Some(choices) = args.choices {
        let indexes = (0..len)
            .map(|_| rng.gen_range(0..choices.len()))
            .collect::<Vec<usize>>();
        indexes
            .iter()
            .map(|x| choices.chars().nth(*x).unwrap())
            .collect()
    } else {
        // Taken from https://scholarsbank.uoregon.edu/xmlui/bitstream/handle/1794/24149/Final%20Thesis-Ygartua.pdf
        // We're keeping the assumption that key smashes are not a markov process
        let choices: [char; 15] = [
            'd', 'j', 's', 'f', 'k', 'h', 'g', 'a', 'l', 'n', 'b', 'm', 'e', 'w', 'r',
        ];
        let weights = [
            2990, 2877, 2594, 2562, 2246, 2017, 1630, 1313, 1223, 1043, 662, 450, 349, 283, 262,
        ];

        let dist = WeightedIndex::new(&weights).unwrap();

        (0..len).map(|_| choices[dist.sample(&mut rng)]).collect()
    };
    println!("{}", result);
}
