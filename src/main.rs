mod vm;

use std::{env, fs};
use std::io::Read;

fn main() {
	if let Some(fname) = env::args().nth(1) {
		if let Ok(prog) = fs::read_to_string(&fname) {
			let mut progsplit = prog.splitn(2, "\n::=\n");
			if let Some(rulesrc) = progsplit.next() {
				if let Some(textsrc) = progsplit.next() {
					let mut textsrc = textsrc.to_string();
					if textsrc.contains("@@") {
						let mut input = String::new();
						let stdin = std::io::stdin();
						stdin.lock().read_to_string(&mut input).expect("failed to read stdin");
						textsrc = textsrc.replace("@@", &input);
					}
					if let Ok(rules) = vm::Rules::parse(rulesrc) {
						println!("{}", rules.run(&textsrc));
					}
				}
			}
		}
	}
}
