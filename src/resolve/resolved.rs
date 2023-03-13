use std::collections::HashMap;
use std::collections::hash_map::Entry;


use super::Reference;
use super::ResolverError;

pub enum Type { 
    Struct(ReferenceMap)
}

pub type ReferenceMap = HashMap<String, Reference>; 

/// Tries to add a `key`/`val` pair to `map`, but adds an error to `errs` if the key already exists
pub fn try_add_reference(map: &mut ReferenceMap, errs: &mut Vec<ResolverError>, key: String, val: Reference) { 
    match map.entry(key) { 
        Entry::Occupied(entry) => { 
            errs.push(ResolverError::Duplicate { 
                original: entry.get().location, 
                redefined_at: val.location,  
            })
        }, 
        Entry::Vacant(entry) => { 
            entry.insert(val);
        }
    }
}
