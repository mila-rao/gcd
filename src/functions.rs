fn build_verctor_all_types_specified() -> Vec<i16> {
    let mut v: Vec<i16> = Vec::<i16>::new();
    v.push(10i16);
    v.push(20.i16);
    v
}



fn build_vector() -> Vec<i16> {
    let mut v = Vec::new();
    v.push(10);
    v.push(20);
    v
}

/* 

Important points:
- Functions require type specification when it is ambiguous. 
- Types are inferred wherever possible (see second function)
- Rust offers flexibity for inference. While it requires type specification
to avoid type errors and cannot be fully dynamically types, types need not be specified 
where inference is possible. 
- Functions can be generic. For example the same Sum function can be used for both signed
and unsigned integer types. 

*/ 
