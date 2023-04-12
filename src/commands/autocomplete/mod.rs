use {
	crate::state::{Context, StateContainer},
	schnosebot::global_map::GlobalMap,
};

// Provides autocompletion for map names on certain commands using some fuzzy finding algorithm
// I found on the interent. :)
pub async fn map_name<'a>(
	ctx: Context<'a>,
	input: &'a str,
) -> impl futures::Stream<Item = String> + 'a {
	futures::stream::iter(
		GlobalMap::fuzzy_match(input, ctx.maps())
			.into_iter()
			.map(|map| map.name),
	)
}
