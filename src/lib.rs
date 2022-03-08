mod seqit;
mod partit;

pub use crate::partit::PartitionIter;


#[cfg(test)]
mod test {
    use super::seqit;
    use super::PartitionIter;


    #[test]
    fn seq_count() {
        // Make sure the number of elements is equal to the
        // Stirling number of the second kind

        let mut stirling = Vec::new();
        stirling.push(vec![]);  // n = 0
        stirling.push(vec![1]); // n = 1
        stirling.push(vec![1,1]); // n = 2
        stirling.push(vec![1,3,1]); // n = 3
        stirling.push(vec![1,7,6,1]); // n = 4
        stirling.push(vec![1,15,25,10,1]); // n = 5
        stirling.push(vec![1,31,90,65,15,1]); // n = 6
        stirling.push(vec![1,63,301,350,140,21,1]); // n = 7

        for n in 1..=7 {
            for (k, res) in std::iter::zip(1..=n, &stirling[n]) {
                let c = seqit::IncSeqIterator::new(n as u32, k as u32).count();
                assert_eq!(c, *res);
            }
        }

        let c = seqit::IncSeqIterator::new(10,5).count();
        assert_eq!(c, 42525); 
    }

    #[test]
    fn kpartition_test() {
        let mut stirling = Vec::new();
        stirling.push(vec![]);  // n = 0
        stirling.push(vec![1]); // n = 1
        stirling.push(vec![1,1]); // n = 2
        stirling.push(vec![1,3,1]); // n = 3
        stirling.push(vec![1,7,6,1]); // n = 4
        stirling.push(vec![1,15,25,10,1]); // n = 5
        stirling.push(vec![1,31,90,65,15,1]); // n = 6
        stirling.push(vec![1,63,301,350,140,21,1]); // n = 7

        for n in 1..=7 {
            let ls:Vec<_> = (1..=n).collect();
            for (k, res) in std::iter::zip(1..=n, &stirling[n]) {
                let c = ls.iter().kpartitions(k as u32).count();
                assert_eq!(c, *res);
            }
        }
    }           

    #[test]
    fn partition_test() {
        let bell = vec![1, 1, 2, 5, 15, 52, 203, 877, 4140];
        for (n, res) in bell.iter().enumerate() {
            let ls:Vec<_> = (1..=n).collect();
            let c = ls.iter().partitions().count();
            assert_eq!(c, *res, "n={} c={} res={}", n, c, *res);
        }
    }          
}

// pub trait PartitionTools: Iterator {
//     fn kpartitions<T>(self, k:u32) -> TupleCombinations<Self, T>
//         where Self: Sized + Clone,
//               Self::Item: Clone,
//               T: adaptors::HasCombination<Self>,
//     {
//         adaptors::tuple_combinations(self)
//     }
// }

