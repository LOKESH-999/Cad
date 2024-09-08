use std::collections::{HashMap,HashSet};
use std::sync::Mutex;

pub struct MFreq{
    heap:Vec<String>,
    ch:HashMap<String,i32>,
    unique:HashSet<String>,
    freq:usize
}

impl MFreq{
    pub fn new (freq:usize)->Self{
        MFreq { heap: Vec::with_capacity(freq+2), ch: HashMap::new(), unique: HashSet::new(), freq: freq }
    }
    pub fn add(&mut self,val:String){
        self.ch.entry(val.clone()).and_modify(|x| *x+=1).or_insert(1);
        if !self.unique.contains(&val){
            self.heap.push(val.clone());
            self.unique.insert(val);
        }
        self.arrange();
        if self.heap.len()>self.freq{
            let a=self.heap.pop().unwrap();
            self.unique.remove(&a);
        }
    }
    pub fn arrange(&mut self){
        let l=self.heap.len();
        let mut idx=l-1;
        while idx>0 {
            if self.ch.get(&self.heap[idx/2]).unwrap()<self.ch.get(&self.heap[idx]).unwrap(){
                self.heap.swap(idx, idx/2)
            }
            idx/=2;
        }
        while idx*2<l-1 {
            if self.ch.get(&self.heap[idx*2+1]).unwrap()>self.ch.get(&self.heap[idx]).unwrap(){
                self.heap.swap(idx, idx*2+1)
            }if self.ch.get(&self.heap[idx*2]).unwrap()>self.ch.get(&self.heap[idx]).unwrap() {
                self.heap.swap(idx, idx*2)
            }
            idx+=1;
        }
        if l>1 && self.ch.get(&self.heap[l-2]).unwrap()<self.ch.get(&self.heap[l-1]).unwrap(){
            self.heap.swap(l-1, l-2)
        }
    }
    pub fn get(&self)->Vec<String>{
        self.heap.clone()
    }
}

pub struct Desc{
    pub d:Mutex<MFreq>
}
pub struct Br{
    pub d:Mutex<MFreq>
}

// impl !Copy for MFreq {}