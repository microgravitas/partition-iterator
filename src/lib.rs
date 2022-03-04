mod seqit;
mod partit;

pub trait Partitions : Iterator {
   
    fn kpartitions(self, k:u32) -> partit::KPartitions<Self>
        where Self: Sized {
        partit::kpartitions(self, k)
    }
}


#[cfg(test)]
mod test {
    use super::seqit;
    use super::partit;
    use super::Partitions;

    #[test]
    fn seq_count() {
        // Make sure the number of elements is equal to the
        // Stirling number of the second kind

        // n = 5
        for (k, res) in std::iter::zip(1..=5, vec![1,15,25,10,1]) {
            let c = seqit::IncSeqIterator::new(5,k).count();
            assert_eq!(c, res);
        }

        // n = 6
        for (k, res) in std::iter::zip(1..=6, vec![1,31,90,65,15,1]) {
            let c = seqit::IncSeqIterator::new(6,k).count();
            assert_eq!(c, res);
        }        

        let c = seqit::IncSeqIterator::new(10,5).count();
        assert_eq!(c, 42525); 
    }

    #[test]
    fn kpartition_test() {
        let ls = vec![1,2,3,4,5];
        let it = ls.iter().kpartitions(3);
    }    

    #[test]
    fn kpartition_test2() {
        let ls = vec![1,2,3,4,5];
        let it = partit::kpartitions(ls.iter(), 3);
        for a in it {
            println!("{:?}", a);
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
