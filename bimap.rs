/**
Author: Havvy

This is an implementation of a bimap done using two hashmaps. I cannot speak
to its speed or size or any characteristics you care about if you care about
code quality. Use this when you need a bimap, and performance doesn't matter
yet. Unless another bimap implementation comes outs by somebody more studied.
*/

use std;
import std::map::hashmap;

type hashbimap<K, V> = {kv: hashmap<K, V>, vk: hashmap<V, K>};

impl hashbimap_map<K, V> of std::map::map<K, V> for hashbimap<K, V> {
    fn size () -> uint { self.kv.size() }

    // TODO Fix this.
    fn insert (+K: K, +V: V) -> bool {
        self.vk.insert(V, K);
        self.kv.insert(K, V)
    }

    fn contains_key (K: K) -> bool { self.kv.contains_key(K) }
    fn get (K: K) -> V { self.kv.get(K) }
    fn [] (K: K) -> V { self.kv[K] }
    fn find (K: K) -> option<V> { self.kv.find(K) }

    fn remove (K: K) -> option<V> {
        let v = self.kv.remove(K);
        alt v {
            none { ret v }
            some(vv) {
                self.vk.remove(vv);
                ret v;
            }
        }
    }

    fn each (mapfn: fn(K, V) -> bool) { self.kv.each(mapfn) }
    fn each_key (mapfn: fn(K) -> bool) { self.kv.each_key(mapfn) }
    fn each_value (mapfn: fn(V) -> bool) { self.vk.each_key(mapfn) }

    fn clear () { self.kv.clear(); self.vk.clear() }
}

iface bimap<K, V> { fn getKey<K, V> (V) -> K; }

impl hashbimap_bimap<K, V> of bimap<K,V> for hashbimap<K, V> {
    fn getKey<K, V> (V: V) -> K { self.vk.get(V) }
}

fn bimap<K: const, clone, V: const, clone> (
    key_hasher: fn@ (K) -> uint,
    key_eqler: fn@ (K, K) -> bool,
    val_hasher: fn@ (V) -> uint,
    val_eqler: fn@ (V, V) -> bool) -> hashbimap<K, V> {

    {
        kv: hashmap::<K, V>(key_hasher, key_eqler), 
        vk: hashmap::<V, K>(val_hasher, val_eqler) 
    }
}

#[cfg(test)]
mod test {
    fn checkRep<K, V> (bimap: hashbimap<K, V>) -> bool {
        assert bimap.vk.size() == bimap.kv.size();

        // For each key, 
        bimap.kv.each(|K, V| -> bool {
            assert bimap.getKey(V) == K
        });
    }

    #[test]
    fn testCreation () {
        let bimap: hashbimap<int, str> = bimap(int::hash, int::eq,
            str::hash, str::eq);
        checkRep(bimap);

        bimap.insert(0, "abc");
        checkRep(bimap);

        bimap.insert(1, "def");
        checkRep(bimap);

        assert bimap.get(0) == "abc";
        assert bimap.get(1) == "def";
    }
}