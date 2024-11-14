use std::collections::HashMap;

enum Error {
    CantAddElement
}

#[derive(serde::Serialize, serde::Deserialize, Debug)]
pub struct Data {
    dict: HashMap<u8, Vec<usize>>,
    elements: Vec<u8>
}

impl Data {
    pub fn new() -> Self {
        return Data { dict: HashMap::new(), elements: Vec::new() };
    }

    pub fn add_element(&mut self, s: &u8) {
        if !self.dict.contains_key(s) {
            self.elements.push(1);
            self.dict.insert(*s, Vec::from([self.elements.len() - 1]));
        } else {
            let indexs = self.dict.get_mut(s).unwrap();
            
            let last_index = *indexs.last().unwrap();
            if last_index == self.elements.len() - 1 {
                let elem = self.elements.get_mut(last_index).unwrap();
                *elem += 1;
            } else {
                self.elements.push(1);
                indexs.push(self.elements.len() - 1);
            }
        }
    }

    pub fn encode(self) -> Vec<u8> {
        return bincode::serialize(&self).unwrap();
    }
}