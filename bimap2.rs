use std;
import std::map::hashmap;

type hashbimap<K, V> = {kv: hashmap<K, V>, vk: hashmap<V, K>};

iface bimap<K: copy, V: const> { fn getKey<K, V> (V) -> K; }

impl hashbimap_bimap<K, V> of bimap<K,V> for hashbimap<K, V> {
    fn getKey<K: copy, V: const> (V: V) -> K { 
        //let vk: hashmap<V, K> = self.vk(V);
        self.vk.get(V)
    }
}

// Works
fn getKey<K: copy, V: const> (self: hashbimap<K, V>, V: V) -> K { self.vk.get(V) }