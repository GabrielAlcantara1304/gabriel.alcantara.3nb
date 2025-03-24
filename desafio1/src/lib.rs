pub unsafe fn multiply_array(ptr: *const i32, len: usize) -> i32 {
    let mut product = 1;
    for i in 0..len  {
        product *= unsafe { *ptr.offset(i as isize) }; 
        //precisou colocar unsafe pra rodar o teste
    }
    product
}
//o loop começa com 1, "for i in 1..len" isso faz com que o primeiro elemento do array
//seja ignorado e é somado 3 * 4
//se mudar para "for i in 0..len" o teste funcionará
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_multiply_array() {
        let arr = [2, 3, 4];
        let product = unsafe { multiply_array(arr.as_ptr(), arr.len()) };
        assert_eq!(product, 24);
    }
}
