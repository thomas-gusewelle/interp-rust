// #[cfg(test)]
// mod test {
//     use crate::object::object::Object;
//
//     #[test]
//     fn test_eval_integer_expression() {
//         struct Test {
//             input: Vec<u8>,
//             expected: Object,
//         }
//         let tests = vec![
//             Test {
//                 input: "5".into(),
//                 expected: Object::Integer(5),
//             },
//             Test {
//                 input: "10".into(),
//                 expected: Object::Integer(10),
//             },
//         ];
//
//         for test in tests.into() {
//             let evaluated = test_eval(test.input);
//         }
//     }
// }
