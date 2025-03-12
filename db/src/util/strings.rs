/// Transforms a slice of strings into a comma seperated list.
/// 
/// Returns an empty string if the input slice is empty.
///
/// # Examples
///
/// From slice:
/// ```
/// let value: String = to_str_list(&["a", "b", "c"]);
///
/// assert_eq!("a,b,c", value);
/// ```
/// 
/// From `Vec`:
/// ```
/// let list: Vec<&str> = vec!["a", "b", "c"];
///
/// assert_eq!("a,b,c", to_str_list(&list[..]));
/// ```
#[allow(dead_code)]
pub fn to_str_list(input: &[&str]) -> String {
	if input.len() == 0 {
		return "".to_owned();
	}

	let mut out: String = input[0].to_owned();

	for string in &input[1..] {
		out.push(',');
		out.push_str(string);
	}

	out
}