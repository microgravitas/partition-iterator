use std::cmp::min;

use crate::seqit::IncSeqIterator;

pub struct Partitions<I>
    where I: Iterator {
    n: u32,
    k: u32,
    data: Vec<I::Item>,
    seqs: IncSeqIterator
}

pub struct KPartitions<I> 
    where I: Iterator   {
    n: u32,
    k: u32,
    data: Vec<I::Item>,
    seqs: IncSeqIterator
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


impl<I,T> Iterator for Partitions<I>
    where I: Iterator<Item = T>,
          T: Clone
{
    type Item = Vec<Vec<T>>;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            if let Some(seq) = self.seqs.next() {
                let mut res:Self::Item = vec![vec![]; self.k as usize];
                for i in 0..(self.n as usize) {
                    res[seq[i] as usize].push(self.data[i].clone());
                }
                return Some(res)
            }

            self.k += 1;
            if self.k > self.n {
                return None
            }
            self.seqs = IncSeqIterator::new(self.n, self.k);
        }
    }
}

pub trait PartitionIter<I> where I: Iterator {
    fn kpartitions(self, k:u32) -> KPartitions<I>;
    fn partitions(self) -> Partitions<I>;
}

impl<I> PartitionIter<I> for I where I: Iterator + Sized
{
    fn kpartitions(self, k:u32) -> KPartitions<I> {
        let data:Vec<I::Item> = self.collect(); // TODO: make lazy
        let n = data.len() as u32;
        KPartitions{ n, k, data, seqs: IncSeqIterator::new(n, k) }
    }

    fn partitions(self) -> Partitions<I> {
        let data:Vec<I::Item> = self.collect(); // TODO: make lazy
        let n = data.len() as u32;
        let k = min(1, n); // k = 0 makes only sense for n = 0
        Partitions{ n, k, data, seqs: IncSeqIterator::new(n, k) }
    }    
}



