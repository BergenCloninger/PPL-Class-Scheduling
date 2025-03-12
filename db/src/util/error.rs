use worker::*;

/// Checks if a `D1Result` wrote any rows.
#[allow(dead_code)]
pub fn check_written(input: &D1Result) -> Option<bool> {
	let metadata: D1ResultMeta = input.meta().ok()??;

	Some(metadata.rows_written? > 0)
}