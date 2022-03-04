use crate::seqit::IncSeqIterator;

pub struct KPartitions<I> 
    where I: Iterator   {
    n: u32,
    k: u32,
    data: Vec<I::Item>,
    seqs: IncSeqIterator
}

pub fn kpartitions<I>(iter: I, k:u32) -> KPartitions<I> 
    where I: Iterator 
{
    let data:Vec<I::Item> = iter.collect(); // TODO: make lazy
    let n = data.len() as u32;
    KPartitions{ n, k, data, seqs: IncSeqIterator::new(n, k) }
}

impl<I,T> Iterator for KPartitions<I>
    where I: Iterator<Item = T>,
          T: Clone
{
    type Item = Vec<Vec<T>>;

    fn next(&mut self) -> Option<Self::Item> {
        match self.seqs.next() {
            Some(seq) => {
                let mut res:Self::Item = vec![vec![]; self.k as usize];
                for i in 0..(self.n as usize) {
                    res[seq[i] as usize].push(self.data[i].clone());
                }
                Some(res)
            }
            None => None,
        }
    }
}
