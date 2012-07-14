/**
Author: Havvy

This is an implementation of a bimap done using two hashmaps. I cannot speak
to its speed or size or any characteristics you care about if you care about
code quality. Use this when you need a bimap, and performance doesn't matter
yet. Unless another bimap implementation comes outs by somebody more studied.
*/

use std;
import std::map;
import std::map::hashmap;

type hashbimap<K, V> = {kv: hashmap<K, V>, vk: hashmap<V, K>};

impl hashbimap_map<K: const copy, V: const copy> of std::map::map<K, V> for
hashbimap<K, V> {
    fn size () -> uint { self.kv.size() }

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

iface bimap<K: const copy, V: const copy> { 
    fn get_key (V) -> K;
    fn find_key (V) -> option<K>;
    fn remove_key (V) -> option<K>;
}

impl hashbimap_bimap<K: const copy, V: const copy> of bimap<K,V> 
for hashbimap<K, V> {
    fn get_key (V: V) -> K { self.vk.get(V) }
    fn find_key (V: V) -> option<K> { self.vk.find(V) }
    fn remove_key (V: V) -> option<K> {
        let k = self.vk.remove(V);
        alt k {
            none { ret k }
            some(kk) {
                self.kv.remove(kk);
                ret k;
            }
        }
    }
}

#[doc="Cannot have impl as ret type in iface"]
fn reverse<K, V> (self: hashbimap<K, V>) -> hashbimap<V, K> { 
    {kv: self.vk, vk: self.kv}
}

fn bimap<K: const copy, V: const copy> (
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
    fn checkRep<K: const copy, V: const copy> (+bimap: hashbimap<K, V>) {
        assert bimap.vk.size() == bimap.kv.size();

        // For each key, the value matches the key.
        bimap.kv.each(|&&K, &&V| { assert bimap.get_key(V) == K; true });
    }

    #[test]
    fn test_creation () {
        let bimap: hashbimap<int, str> = bimap::<int, str>(int::hash, 
            int::eq, str::hash, str::eq);
        checkRep(bimap);

        bimap.insert(0, copy "abc");
        checkRep(bimap);

        bimap.insert(1, copy "def");
        checkRep(bimap);

        assert bimap.get(0) == "abc";
        assert bimap.get(1) == "def";

        assert bimap.get_key(copy "abc") == 0;
        assert bimap.get_key(copy "def") == 1;
    }

    /* Tests from stdlib/map, co-opted into testing bimap */

    #[test]
    fn test_simple() {
        #debug("*** starting test_simple");
        fn eq_uint(&&x: uint, &&y: uint) -> bool { ret x == y; }
        fn uint_id(&&x: uint) -> uint { x }
        let hasher_uint: map::hashfn<uint> = uint_id;
        let eqer_uint: map::eqfn<uint> = eq_uint;
        let hasher_str: map::hashfn<str> = str::hash;
        let eqer_str: map::eqfn<str> = str::eq;
        #debug("uint -> uint");
        let hbm_uu: hashbimap<uint, uint> = bimap::<uint, uint>(
            hasher_uint, eqer_uint, hasher_uint, eqer_uint);
        assert (hbm_uu.insert(10u, 12u));
        assert (hbm_uu.insert(11u, 13u));
        assert (hbm_uu.insert(12u, 14u));
        assert (hbm_uu.get(11u) == 13u);
        assert (hbm_uu.get_key(13u) == 11u);
        assert (hbm_uu.get(12u) == 14u);
        assert (hbm_uu.get_key(14u) == 12u);
        assert (hbm_uu.get(10u) == 12u);
        assert (hbm_uu.get_key(12u) == 10u);
        assert (!hbm_uu.insert(12u, 14u));
        assert (hbm_uu.get(12u) == 14u);
        assert (hbm_uu.get_key(14u) == 12u);
        assert (!hbm_uu.insert(12u, 12u));
        assert (hbm_uu.get(12u) == 12u);
        assert (hbm_uu.get_key(12u) == 12u);
        let ten: str = "ten";
        let eleven: str = "eleven";
        let twelve: str = "twelve";
        #debug("str -> uint");
        let hbm_su: hashbimap<str, uint> = 
            bimap::<str, uint>(hasher_str, eqer_str, hasher_uint, eqer_uint);
        assert (hbm_su.insert("ten", 12u));
        assert (hbm_su.insert(eleven, 13u));
        assert (hbm_su.insert("twelve", 14u));
        assert (hbm_su.get(eleven) == 13u);
        assert (hbm_su.get("eleven") == 13u);
        assert (hbm_su.get("twelve") == 14u);
        assert (hbm_su.get("ten") == 12u);
        assert (!hbm_su.insert("twelve", 14u));
        assert (hbm_su.get("twelve") == 14u);
        assert (!hbm_su.insert("twelve", 12u));
        assert (hbm_su.get("twelve") == 12u);
        #debug("uint -> str");
        let hbm_us: hashbimap<uint, str> =
            bimap::<uint, str>(hasher_uint, eqer_uint, hasher_str, eqer_str);
        assert (hbm_us.insert(10u, "twelve"));
        assert (hbm_us.insert(11u, "thirteen"));
        assert (hbm_us.insert(12u, "fourteen"));
        assert (str::eq(hbm_us.get(11u), "thirteen"));
        assert (str::eq(hbm_us.get(12u), "fourteen"));
        assert (str::eq(hbm_us.get(10u), "twelve"));
        assert (!hbm_us.insert(12u, "fourteen"));
        assert (str::eq(hbm_us.get(12u), "fourteen"));
        assert (!hbm_us.insert(12u, "twelve"));
        assert (str::eq(hbm_us.get(12u), "twelve"));
        #debug("str -> str");
        let hbm_ss: hashbimap<str, str> =
            bimap::<str, str>(hasher_str, eqer_str, hasher_str, eqer_str);
        assert (hbm_ss.insert(ten, "twelve"));
        assert (hbm_ss.insert(eleven, "thirteen"));
        assert (hbm_ss.insert(twelve, "fourteen"));
        assert (str::eq(hbm_ss.get("eleven"), "thirteen"));
        assert (str::eq(hbm_ss.get("twelve"), "fourteen"));
        assert (str::eq(hbm_ss.get("ten"), "twelve"));
        assert (!hbm_ss.insert("twelve", "fourteen"));
        assert (str::eq(hbm_ss.get("twelve"), "fourteen"));
        assert (!hbm_ss.insert("twelve", "twelve"));
        assert (str::eq(hbm_ss.get("twelve"), "twelve"));
        #debug("*** finished test_simple");
    }

    /**
    * Force map growth
    */
    #[test]
    fn test_growth() {
        #debug("*** starting test_growth");
        let num_to_insert: uint = 64u;
        fn eq_uint(&&x: uint, &&y: uint) -> bool { ret x == y; }
        fn uint_id(&&x: uint) -> uint { x }
        #debug("uint -> uint");
        let hasher_uint: map::hashfn<uint> = uint_id;
        let eqer_uint: map::eqfn<uint> = eq_uint;
        let hbm_uu: hashbimap<uint, uint> = bimap::<uint, uint>(
            hasher_uint, eqer_uint, hasher_uint, eqer_uint);
        let mut i: uint = 0u;
        while i < num_to_insert {
            assert (hbm_uu.insert(i, i * i));
            #debug("inserting %u -> %u", i, i*i);
            i += 1u;
        }
        #debug("-----");
        i = 0u;
        while i < num_to_insert {
            #debug("get(%u) = %u", i, hbm_uu.get(i));
            assert (hbm_uu.get(i) == i * i);
            i += 1u;
        }
        assert (hbm_uu.insert(num_to_insert, 17u));
        assert (hbm_uu.get(num_to_insert) == 17u);
        #debug("-----");
        i = 0u;
        while i < num_to_insert {
            #debug("get(%u) = %u", i, hbm_uu.get(i));
            assert (hbm_uu.get(i) == i * i);
            i += 1u;
        }
        #debug("str -> str");
        let hasher_str: map::hashfn<str> = str::hash;
        let eqer_str: map::eqfn<str> = str::eq;
        let hbm_ss: hashbimap<str, str> =
            bimap::<str, str>(hasher_str, eqer_str, hasher_str, eqer_str);
        i = 0u;
        while i < num_to_insert {
            assert hbm_ss.insert(uint::to_str(i, 2u), uint::to_str(i * i, 2u));
            #debug("inserting \"%s\" -> \"%s\"",
                   uint::to_str(i, 2u),
                   uint::to_str(i*i, 2u));
            i += 1u;
        }
        #debug("-----");
        i = 0u;
        while i < num_to_insert {
            #debug("get(\"%s\") = \"%s\"",
                   uint::to_str(i, 2u),
                   hbm_ss.get(uint::to_str(i, 2u)));
            assert (str::eq(hbm_ss.get(uint::to_str(i, 2u)),
                            uint::to_str(i * i, 2u)));
            i += 1u;
        }
        assert (hbm_ss.insert(uint::to_str(num_to_insert, 2u),
                             uint::to_str(17u, 2u)));
        assert (str::eq(hbm_ss.get(uint::to_str(num_to_insert, 2u)),
                        uint::to_str(17u, 2u)));
        #debug("-----");
        i = 0u;
        while i < num_to_insert {
            #debug("get(\"%s\") = \"%s\"",
                   uint::to_str(i, 2u),
                   hbm_ss.get(uint::to_str(i, 2u)));
            assert (str::eq(hbm_ss.get(uint::to_str(i, 2u)),
                            uint::to_str(i * i, 2u)));
            i += 1u;
        }
        #debug("*** finished test_growth");
    }

    #[test]
    fn test_removal() {
        #debug("*** starting test_removal");
        let num_to_insert: uint = 64u;
        fn eq(&&x: uint, &&y: uint) -> bool { ret x == y; }
        fn hash(&&u: uint) -> uint {
            // This hash function intentionally causes collisions between
            // consecutive integer pairs.

            ret u / 2u * 2u;
        }
        assert (hash(0u) == hash(1u));
        assert (hash(2u) == hash(3u));
        assert (hash(0u) != hash(2u));
        let hasher: map::hashfn<uint> = hash;
        let eqer: map::eqfn<uint> = eq;
        let hbm: hashbimap<uint, uint> = 
            bimap::<uint, uint>(hasher, eqer, hasher, eqer);
        let mut i: uint = 0u;
        while i < num_to_insert {
            assert (hbm.insert(i, i * i));
            #debug("inserting %u -> %u", i, i*i);
            i += 1u;
        }
        assert (hbm.size() == num_to_insert);
        #debug("-----");
        #debug("removing evens");
        i = 0u;
        while i < num_to_insert {
            let v = hbm.remove(i);
            alt v {
              option::some(u) { assert (u == i * i); }
              option::none { fail; }
            }
            i += 2u;
        }
        assert (hbm.size() == num_to_insert / 2u);
        #debug("-----");
        i = 1u;
        while i < num_to_insert {
            #debug("get(%u) = %u", i, hbm.get(i));
            assert (hbm.get(i) == i * i);
            i += 2u;
        }
        #debug("-----");
        i = 1u;
        while i < num_to_insert {
            #debug("get(%u) = %u", i, hbm.get(i));
            assert (hbm.get(i) == i * i);
            i += 2u;
        }
        #debug("-----");
        i = 0u;
        while i < num_to_insert {
            assert (hbm.insert(i, i * i));
            #debug("inserting %u -> %u", i, i*i);
            i += 2u;
        }
        assert (hbm.size() == num_to_insert);
        #debug("-----");
        i = 0u;
        while i < num_to_insert {
            #debug("get(%u) = %u", i, hbm.get(i));
            assert (hbm.get(i) == i * i);
            i += 1u;
        }
        #debug("-----");
        assert (hbm.size() == num_to_insert);
        i = 0u;
        while i < num_to_insert {
            #debug("get(%u) = %u", i, hbm.get(i));
            assert (hbm.get(i) == i * i);
            i += 1u;
        }
        #debug("*** finished test_removal");
    }

    #[test]
    fn test_contains_key() {
        let key = "k";
        let map = bimap::<str, str>(str::hash, str::eq, str::hash, str::eq);
        assert (!map.contains_key(key));
        map.insert(key, "val");
        assert (map.contains_key(key));
    }

    #[test]
    fn test_find() {
        let key = "k";
        let map = bimap::<str, str>(str::hash, str::eq, str::hash, str::eq);
        assert (option::is_none(map.find(key)));
        map.insert(key, "val");
        assert (option::get(map.find(key)) == "val");
    }

    #[test]
    fn test_clear() {
        let key = "k";
        let map = bimap::<str, str>(str::hash, str::eq, str::hash, str::eq);
        map.insert(key, "val");
        assert (map.size() == 1);
        assert (map.contains_key(key));
        map.clear();
        assert (map.size() == 0);
        assert (!map.contains_key(key));
    }
}