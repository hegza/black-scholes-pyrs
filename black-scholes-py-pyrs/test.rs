use std::collections::HashMap;
use std::*;

use black_scholes_dp::*;
use black_scholes_ndp::*;
struct MyTest {}

impl MyTest {
    fn test_non_divident_paying(&self) {
        self.assertEqual(
            euro_vanilla_call(50, 100, 1, 0.05, 0.25),
            0.027352509369436617,
        );
        self.assertEqual(euro_vanilla_put(50, 100, 1, 0.05, 0.25), 45.15029495944084);
        self.assertEqual(
            euro_vanilla(50, 100, 1, 0.05, 0.25, "put"),
            45.15029495944084,
        );
    }
    fn test_divident_paying(&self) {
        /*pass*/
    }
}
fn main() {
    unittest.main();
}
