use dearx_edit_model::TestData;

fn main() {
    let test_data = TestData{ value: 0  };
    let _new_test_data = test_data.with_value(1);

    println!("Hello, world!");
}
