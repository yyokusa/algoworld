// https://en.wikipedia.org/wiki/Heap%27s_algorithm
// procedure generate(k : integer, A : array of any):
//     if k = 1 then
//         output(A)
//     else
//         // Generate permutations with k-th unaltered
//         // Initially k = length(A)
//         generate(k - 1, A)

//         // Generate permutations for k-th swapped with each k-1 initial
//         for i := 0; i < k-1; i += 1 do
//             // Swap choice dependent on parity of k (even or odd)
//             if k is even then
//                 swap(A[i], A[k-1]) // zero-indexed, the k-th is at k-1
//             else
//                 swap(A[0], A[k-1])
//             end if
//             generate(k - 1, A)
//         end for
//     end if

fn permutations(k: u32, mut a: Vec<u32>) {
    if k == 1 {
        println!("{:?}", a);
    } else {
        permutations(k - 1, a.clone());
        for i in 0..k-1 {
            if k % 2 == 0 {
                a.swap(i as usize, (k - 1) as usize);
            } else {
                a.swap(0, (k - 1) as usize);
            }
            permutations(k - 1, a.clone());
        }
    }
}


fn main() {
    let a = vec![1, 2, 3, 4];
    permutations(a.len() as u32, a);
}