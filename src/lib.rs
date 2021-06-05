//! Implementations of the Cool-lex (http://webhome.cs.uvic.ca/~ruskey/Publications/Coollex/CoolComb.html) algorithms.

pub mod linked_list;

#[cfg(test)]
mod tests {
    use crate::linked_list::*;
    //
    // Examples of usage
    //
    #[test]
    fn example_direct_use() {
        let mut alg: Algorithm = Algorithm::new(3, 2); // two '1' and three '0', that is n=5,k=2
        loop {
            println!("{}", alg); //prints the current combination as a bit-string
            if !alg.has_more() {
                break;
            }
            alg.next_combination();
        }
    }
    #[test]
    fn example_iterator_use() {
        let elements = vec!['A', 'B', 'C', 'D', 'E'];
        let combs = CoollexLinkedList::combinations(elements.len(), 2); //k=2
        for comb in combs {
            for i in comb {
                print!("{}", elements[i]);
            }
            println!()
        }
    }

    //
    // Testing methods
    //
    #[test]
    fn one() {
        test_linked_list(33, 6);
        test_linked_list(9, 9);
        test_linked_list(10, 4);
        test_linked_list(15, 7);
        test_linked_list(15, 6);
    }
    fn test_linked_list(n: usize, k: usize) {
        // value at given index denotes how many times this element appeared in a combination
        let mut hits = vec![0; n];

        let combs = CoollexLinkedList::combinations(n, k);
        let mut num_comb = 0; // total number of combinations yielded by the algorithm
        for comb in combs {
            let mut num_elem = 0; // number of elements in this combination
            for i in comb {
                hits[i] += 1;
                num_elem += 1;
            }
            assert_eq!(k, num_elem, "number of elements in a combination");

            num_comb += 1;
        }

        assert_eq!(
            calc_num_comb(n as u64, k as u64),
            num_comb,
            "number of combinations"
        );

        let occur = calc_num_comb((n - 1) as u64, (k - 1) as u64);
        for hit in hits {
            assert_eq!(
                occur, hit,
                "number of combinations where each element appears"
            );
        }
    }

    //
    // Helper methods
    //
    fn calc_num_comb(n: u64, k: u64) -> u64 {
        assert!(k <= n, "k={}, n={}", k, n);
        if k == 0 {
            return 0;
        }
        multiply_all(n, n - k + 1) / multiply_all(k, 1)
    }
    fn multiply_all(mut n2: u64, n1: u64) -> u64 {
        let mut r = n2;
        loop {
            n2 -= 1;
            if n2 < n1 {
                break;
            }
            r *= n2;
        }
        r
    }
}
