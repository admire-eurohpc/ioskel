use clap::Parser;
use rand::Rng;
use rand::{rngs::StdRng, SeedableRng};
use regex::Regex;
use std::fs::{File, OpenOptions};
use std::io::{BufReader, Read, Seek, Write};
use std::thread::sleep;
use std::time::Duration;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    /// compute_time of the IO start
    #[arg(short, long)]
    compute_time: Option<u64>,
    /// Size of IO
    #[arg(short, long)]
    size: Option<usize>,
    /// Iterations of IO
    #[arg(short, long)]
    iter: Option<u32>,
    #[arg(short, long)]
    transition_probability: Option<u32>,
}

#[derive(Debug)]
struct Configuration {
    compute_time: u64,
    size: usize,
    iter: u32,
    proba: u32,
}

impl Configuration {
    fn load_from_binary_name() -> (Option<u64>, Option<usize>, Option<u32>, Option<u32>) {
        let mut compute_time: Option<u64> = None;
        let mut size: Option<usize> = None;
        let mut iter: Option<u32> = None;
        let mut probability: Option<u32> = None;

        if let Ok(f) = File::open("/proc/self/cmdline") {
            let mut r = BufReader::new(f);
            let mut d = String::new();
            r.read_to_string(&mut d).unwrap();

            /* Now we have the command line in d
            we expect a name of ioskel.COMP.SIZE */
            for v in d.split('\0') {
                if v.contains("ioskel") {
                    let re =
                        Regex::new(r"ioskel\.([0-9]+)\.([0-9]+)\.([0-9]+)(?:\.([0-9]+))?").unwrap();

                    for cap in re.captures_iter(d.as_str()) {
                        if let Ok(ctime) = cap[1].parse::<u64>() {
                            compute_time = Some(ctime);
                        }

                        if let Ok(s) = cap[2].parse::<usize>() {
                            size = Some(s);
                        }

                        if let Ok(i) = cap[3].parse::<u32>() {
                            iter = Some(i);
                        }

                        if let Some(proba) = cap.get(4) {
                            if let Ok(proba) = proba.as_str().parse::<u32>() {
                                probability = Some(proba);
                            }
                        }
                    }
                }
            }
        }

        (compute_time, size, iter, probability)
    }

    fn new(cli: &Cli) -> Configuration {
        let (comp, s, i, p) = Configuration::load_from_binary_name();

        let mut compute_time = comp.unwrap_or(3);
        let mut size = s.unwrap_or(1024 * 1024);
        let mut iter = i.unwrap_or(10);
        let mut proba = p.unwrap_or(0);

        /* Now the CLI can override */
        compute_time = cli.compute_time.unwrap_or(compute_time);
        size = cli.size.unwrap_or(size);
        iter = cli.iter.unwrap_or(iter);
        proba = cli.transition_probability.unwrap_or(proba);

        Configuration {
            compute_time,
            size,
            iter,
            proba,
        }
    }

    fn outfile() -> String {
        let host = hostname::get().unwrap();
        let pid = std::process::id();

        format!("./ioskel-out-{}-{}.dat", host.to_string_lossy(), pid)
    }
}

fn main() {
    let cli = Cli::parse();
    let conf = Configuration::new(&cli);
    let mut rng = StdRng::seed_from_u64(1337);

    println!("{:?}", conf);

    let mut data: Vec<u8> = Vec::with_capacity(conf.size);
    for i in 0..conf.size {
        data.push((i % 255) as u8);
    }

    let mut outfile = OpenOptions::new()
        .read(true) // Open for reading.
        .write(true) // Open for writing.
        .create(true)
        .open(Configuration::outfile())
        .unwrap();

    let mut is_read = false;

    for i in 0..conf.iter {
        let act = if is_read { "READ" } else { "WRITE" };

        println!("Iteration {} , {}", i, act);

        if is_read && i > 0 {
            /* We go back at start to have someting to read */
            outfile.rewind().unwrap();
            outfile.read_exact(&mut data).unwrap();
        } else {
            outfile.write_all(&data).unwrap();
        }

        if conf.proba != 0 {
            let rnd = rng.gen::<u32>() % 100;
            if rnd <= conf.proba {
                is_read = !is_read;
            }
        }

        sleep(Duration::from_secs(conf.compute_time));
    }

    std::fs::remove_file(Configuration::outfile()).unwrap();
}
