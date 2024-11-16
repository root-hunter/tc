use rustc_hash::FxHashMap;

enum Error {
    CantAddElement
}

pub trait EngineAllowedTypes{} 
impl EngineAllowedTypes for u8 {}
impl EngineAllowedTypes for String {}

pub trait Engine<T> 
where 
    T: EngineAllowedTypes + serde::Deserialize<'static> + std::cmp::Eq + std::hash::Hash,
{
    fn new(size: usize) -> Data<T> {
        return Data { size, dict: FxHashMap::<T, Vec<DataElement>>::default(), data_length: 0 };
    }

    fn from_slice_u8(buffer: &[T]) -> Self;
    fn from_slice_vec(buffer: Vec<T>) -> Self;

    fn add_element(&mut self, s: &T);

    fn to_str(self) -> String;
    fn to_bytes(self) -> Vec<u8>;

    fn exists_elem_by_index(&mut self, key: T, index: usize) -> bool;
}

pub trait AllowedExportTypes {}

impl AllowedExportTypes for u8 {}
impl AllowedExportTypes for u16 {}
impl AllowedExportTypes for usize {}

#[derive(serde::Serialize, serde::Deserialize, Debug)]
enum DataExportType {
    U8,
    U16,
    USIZE
}

pub trait ExportBehavior<T, D, E> 
where 
    T: EngineAllowedTypes + std::cmp::Eq + std::hash::Hash,
    D: AllowedExportTypes + std::cmp::Eq + std::hash::Hash,
    E: AllowedExportTypes + std::cmp::Eq + std::hash::Hash,
{
    //fn new(size: usize, dict: HashMap<T, Vec<usize>>, elements: Vec<E>) -> DataExport<T, D, E>;
    fn to_bytes(self) -> Vec<u8>;
}

#[derive(serde::Serialize, serde::Deserialize, Debug)]
pub struct DataExport<T, D, E>
where 
    T: EngineAllowedTypes + std::cmp::Eq + std::hash::Hash,
    D: AllowedExportTypes + std::cmp::Eq + std::hash::Hash,
    E: AllowedExportTypes + std::cmp::Eq + std::hash::Hash,
{
    dict_type: DataExportType,
    elements_type: DataExportType,
    size: usize,
    dict: FxHashMap<T, Vec<D>>,
    elements: Vec<E>
}

impl ExportBehavior<String, u16, u8> for DataExport<String, u16, u8> {
    // fn new(size: usize, dict: HashMap<String, Vec<usize>>) -> DataExport<String, u16, u8> {
    //     let mut dictionary = HashMap::<String, Vec<u16>>::new();
        
    //     for key in dict.keys() {
    //         let value = dict.get(key).unwrap();
    //         let mut v = Vec::<u16>::new();

    //         for e in value {
    //             let value = *e as u16;
    //             v.push(value);
    //         }

    //         dictionary.insert(key.clone(), v);
    //     }
        
    //     return DataExport{
    //         dict: dictionary,
    //         size,
    //         elements,
    //         dict_type: DataExportType::U16,
    //         elements_type: DataExportType::U8,
    //     };
    // }
 
    fn to_bytes(self) -> Vec<u8> {
        return bincode::serialize(&self).unwrap();
    }
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, Copy)]
pub struct DataElement
{
    index: usize,
    count: usize
}

impl DataElement {
    pub fn inc(&mut self) {
        self.count += 1;
    }    
}

#[derive(serde::Serialize, serde::Deserialize, Debug)]
pub struct Data<T>
where 
    T: EngineAllowedTypes + std::cmp::Eq + std::hash::Hash,
{
    pub size: usize,
    pub data_length: usize,
    dict: FxHashMap<T, Vec<DataElement>>,
}

impl Engine<u8> for Data<u8>
 {
    fn from_slice_u8(buffer: &[u8]) -> Self {
        return Data { size: buffer.len(), dict: FxHashMap::default(), data_length: 0 };
    }

    fn from_slice_vec(buffer: Vec<u8>) -> Self {
        return Data { size: buffer.len(), dict: FxHashMap::default(), data_length: 0 };
    }

    fn exists_elem_by_index(&mut self, key: u8, index: usize) -> bool {
        let symbol_elements = self.dict.get(&key).unwrap();

        for elem in symbol_elements {
            if elem.index == index {
                return true;
            }
        }
        return false;
    }

    fn add_element(&mut self, s: &u8) {
        let index = self.data_length;

        let exists = self.exists_elem_by_index(*s, index);
        if exists {
            let elem = self.dict.get_mut(s).unwrap().get_mut(index).unwrap();
            elem.inc();
            
            self.data_length += 1;
        } else {
            let elem = DataElement{
                index,
                count: 1
            };

            self.dict.insert(*s, Vec::from([elem]));
        }
    }

    fn to_bytes(self) -> Vec<u8> {
        return bincode::serialize(&self).unwrap();
    }
    
    fn to_str(self) -> String {
        let mut output = Vec::<u8>::new();
        output.reserve(self.size);

        for key in self.dict.keys() {
            let elements = self.dict.get(key).unwrap();
            for element in elements {
                let e = output.get_mut(element.index).unwrap();
                *e = *key;
            }
        }

        return String::from_utf8(output).unwrap();
    }
}

impl Engine<String> for Data<String>
 {
    fn from_slice_u8(buffer: &[String]) -> Self {
        return Data { size: buffer.len(), dict: FxHashMap::default(), data_length: 0 };
    }

    fn from_slice_vec(buffer: Vec<String>) -> Self {
        return Data { size: buffer.len(), dict: FxHashMap::default(), data_length: 0 };
    }

    fn add_element(&mut self, s: &String) {
        let index = self.data_length;

        if self.exists_elem_by_index(s.clone(), index) {
            if let Some(elements) = self.dict.get_mut(s) {
                if let Some(elem) = elements.get_mut(index) {
                    elem.inc();
                } else {
                    println!("ERROR ELEMENT");
                }
            } else {
                println!("ERROR ELEMENTS");
            }
            
        } else {
            let elem = DataElement{
                index,
                count: 1
            };

            self.dict.insert(s.clone(), Vec::from([elem]));
        }
    }

    fn to_bytes(self) -> Vec<u8> {
        return bincode::serialize(&self).unwrap();
    }
    
    fn to_str(self) -> String {
        let mut output = Vec::<String>::new();
        output.reserve(self.size);

        for key in self.dict.keys() {
            let elements = self.dict.get(key).unwrap();
            for element in elements {
                let e = output.get_mut(element.index).unwrap();
                *e = key.clone();
            }
        }

        return output.join("");
    }
    
    fn exists_elem_by_index(&mut self, key: String, index: usize) -> bool {
        
        if let Some(symbol_elements) = self.dict.get(&key) {
            for elem in symbol_elements {
                if elem.index == index {
                    return true;
                }
            }
        }

        return false;
    }
}