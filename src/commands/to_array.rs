use crate::object::Value;
use crate::prelude::*;

pub fn to_array(args: CommandArgs) -> Result<OutputStream, ShellError> {
    let out = args.input.collect();
    Ok(out
        .map(|vec: Vec<_>| single_output(Value::List(vec)))
        .flatten_stream()
        .boxed())
}
