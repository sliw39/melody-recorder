use std::collections::HashMap;
use std::iter::Rev;
use std::slice::Iter;


pub struct SeqData<T> {
    _items: HashMap<String, usize>,
    _vec: Vec<T>
}

impl<T> SeqData<T> {
    pub fn new() -> Self {
        Self {
            _items: HashMap::new(),
            _vec: Vec::new()
        }
    }

    pub fn add(&mut self, key: &str, data: T) {
        self._vec.push(data);
        self._items.insert(String::from(key), self._vec.len() - 1);
    }

    pub fn iter_forward(&self, key: &str) -> Option<Iter<T>> {
        let iopt = self._items.get(key);
        if iopt.is_none() {
            return None
        }
        let mut iter = self._vec.iter();
        for _ in 0..(*iopt.unwrap()) {
            match iter.next() {
                None => return None,
                Some(_) => continue
            }
        }
        Some(iter)
    }

    pub fn iter_backward(&self, key: &str) -> Option<Rev<Iter<T>>> {
        let iopt = self._items.get(key);
        if iopt.is_none() {
            return None
        }
        let mut iter = self._vec.iter().rev();
        for _ in 0..(self._vec.len() - *iopt.unwrap() -1) {
            match iter.next() {
                None => return None,
                Some(_) => continue
            }
        }
        Some(iter)
    }

    pub fn get(&self, key: &str) -> Option<&T> {
        self._items.get(&String::from(key)).and_then(|i| self._vec.get(*i))
    }
}

#[cfg(test)]
mod tests {

    use super::SeqData;

    #[test]
    fn test_get() {
        let mut seq = SeqData::new();
        for i in 0..9 {
            seq.add(format!("test{}", i).as_str(), format!("test{} value", i));
        }

        let data = seq.get("test6").unwrap();
        assert_eq!(String::from("test6 value"), *data);
    }

    #[test]
    fn test_get_not_exists() {
        let seq: SeqData<String> = SeqData::new();

        let data = seq.get("fake");
        assert!(data.is_none());
    }

    #[test]
    fn test_iter_forward() {
        let mut seq: SeqData<String> = SeqData::new();
        for i in 0..9 {
            seq.add(format!("test{}", i).as_str(), format!("test{} value", i));
        }

        let mut data = seq.iter_forward("test5").unwrap();
        let mut str = Vec::new();
        while let Some(value) = data.next() {
            str.push(value.as_str());
        }
        assert_eq!("test5 value;test6 value;test7 value;test8 value", str.join(";"));
    }

    
    #[test]
    fn test_iter_forward_not_exists() {
        let mut seq = SeqData::new();
        seq.add("test0", String::from("test0 value"));

        let test = seq.iter_forward("test5_fake");
        assert!(test.is_none());
    }

    #[test]
    fn test_iter_backward() {
        let mut seq: SeqData<String> = SeqData::new();
        for i in 0..9 {
            seq.add(format!("test{}", i).as_str(), format!("test{} value", i));
        }

        let mut data = seq.iter_backward("test5").unwrap();
        let mut str = Vec::new();
        while let Some(value) = data.next() {
            str.push(value.as_str());
        }
        assert_eq!("test5 value;test4 value;test3 value;test2 value;test1 value;test0 value", str.join(";"));
    }

    
    #[test]
    fn test_iter_backward_not_exists() {
        let mut seq = SeqData::new();
        seq.add("test0", String::from("test0 value"));

        let test = seq.iter_backward("test5_fake");
        assert!(test.is_none());
    }
 
}