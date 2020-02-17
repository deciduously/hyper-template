// templates.rs
// Typed structs for each template in /templates/

use askama::Template;

#[derive(Default, Template)]
#[template(path = "skel.html")]
pub struct SkelTemplate {}

#[derive(Default, Template)]
#[template(path = "404.html")]
pub struct FourOhFourTemplate {}

#[derive(Default, Template)]
#[template(path = "index.html")]
pub struct IndexTemplate {}
