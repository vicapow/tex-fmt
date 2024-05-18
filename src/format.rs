use crate::indent::*;
use crate::logging::*;
use crate::subs::*;
use crate::wrap::*;
use crate::Cli;
use log::Level::Error;

const MAX_PASS: usize = 10;

fn apply_passes(
    file: &str,
    filename: &str,
    args: &Cli,
    logs: &mut Vec<Log>,
) -> String {
    let mut new_file = apply_indent(file, filename, args, logs, Some(1));
    let mut finished = false;
    let mut pass = 2;

    while needs_wrap(&new_file) && !finished && pass < MAX_PASS + 2 {
        let old_file = new_file.clone();
        new_file = wrap(&new_file, filename, logs, Some(pass));
        new_file = remove_trailing_spaces(&new_file);
        new_file = apply_indent(&new_file, filename, args, logs, Some(pass));
        pass += 1;
        if new_file == old_file {
            finished = true;
        }
    }

    // check indents return to zero
    if new_file.lines().last().unwrap().starts_with(' ') {
        //log::error!("");
        record_log(
            logs,
            Error,
            None,
            filename.to_string(),
            None,
            None,
            "Indent does not return to zero at end of file.".to_string(),
        );
    }

    new_file
}

pub fn format_file(
    file: &str,
    filename: &str,
    args: &Cli,
    logs: &mut Vec<Log>,
) -> String {
    let mut new_file = remove_extra_newlines(file);
    new_file = begin_end_environments_new_line(&new_file);
    new_file = remove_tabs(&new_file);
    new_file = remove_trailing_spaces(&new_file);
    new_file = apply_passes(&new_file, filename, args, logs);
    new_file
}
