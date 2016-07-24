extern crate pencil;

use pencil::Pencil;
use pencil::{Request, PencilResult, Response};
use pencil::method::Get;


fn hello(req: &mut Request) -> PencilResult {
	let mut response = String::from("All headers:\n");
	for header in req.headers().iter() {
		response.push_str(&format!("{}: {}\n",
			header.name(),
			header.value_string()
		));
	}

	let mut retval = Response::from(response);
	retval.set_content_type("text/plain");
	return Ok(retval)
}


fn main() {
	let mut app = Pencil::new("/dev/null");
	app.route("/", &[Get], "hello", hello);
	app.run("127.0.0.1:5000");
}
