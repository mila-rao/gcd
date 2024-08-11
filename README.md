# RUST LESSONS

## Primitive Types
1. i8 - i128 - Signed integers of a given bit width
2. u8 - u128 - Unsigned integers of a given bit width
3. f32, f64 - Floating point numbers with single and double precision
4. bool - Boolean
5. char - Unicode character, 32 bits width
6. String - utf-8 string, dynamically sized
7. (char, u8, i32) - Tuple; mixed types allowed
8. () - Unit; empty tuple
9. Vec<f64> - Vector of varying length; all elements are of same type
10. [f64, 4] - Array of fixed length; all elements are of same type
11. Struct S {x: i64, y: i64} - Named-field struct
12. Struct T (i64, char) - Tuple like struct
13. Struct E - Unit like struct; has no fields
14. enum Attend { OnTime, u32 } - Enumeration of algebraic data type fields
15. mut - Mutable variable
16. &str - Reference to a string field; non-owing pointer
17. Option<&str> - Optional value; type specified or None
18. Result<u64, Error> Result of an operation can have only two types
19. fn(&str) -> - Pointer to a function