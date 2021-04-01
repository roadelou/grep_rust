// Used to read the input line by line.
use std::io::{self, BufRead};

fn main() {
    // We start by locking stdin and reading all the lines. We return the lines
    // that match at least one of our arguments.
    for matched_line in io::stdin().lock().lines().filter_map(
        // Each line returned is a Result<String>.
        |line_result| line_result.map_or(
            // If we encountered an error, we skip the line.
            None,
            // If we didn't get an EOF, we inspect the string. We return a
            // boolean telling if the string contains any word of interest. We
            // skip 1 argument, argv[0] is the name of the program.
            |line| std::env::args().skip(1).fold(
                // By default the line is not matched.
                None,
                |accumulator, argument| accumulator.or(
                    // If the accumulator is still None, we try to match the
                    // next argument.
                    line.contains(&argument).then(
                        // We return a copy of the line if it contains the
                        // argument, None otherwise. We can't move the value of
                        // line here because it will be needed for future calls
                        // to fold. Well, actually it won't because of the or,
                        // but the compiler cannot deduce that.
                        || line.clone()
                    )
                )
            )
        )
    ) {
        // We print the line that matched one of the input words.
        println!{"{}", matched_line};
    }
}
