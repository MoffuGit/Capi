use convex::Value as ConvexValue;
use serde_json::Value as JsonValue;

use super::subscripton::convert_json_value_to_convex_value;

pub struct Mutation {
    args: JsonValue,
    name: String,
}

impl Mutation {
    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn convex_args(&self) -> ConvexValue {
        convert_json_value_to_convex_value(self.args.clone())
    }
}
