
/**
This is an implementation of a bimap done using two hashmaps. I cannot speak
to its speed or size or any characteristics you care about if you care about
code metrics. Use this when you need a bimap, and performance doesn't matter.
*/

extern mod std;
use std::map;
use std::map::Map;
use std::map::HashMap;
use cmp::Eq;
use to_bytes::IterBytes;
use hash::Hash;

type hashbimap<K, V> = {kv: HashMap<K, V>, vk: HashMap<V, K>};

impl<K: Const Copy Eq IterBytes Hash, V: Const Copy Eq IterBytes Hash> hashbimap<K, V> : std::map::Map<K, V> {
    pure fn size () -> uint { self.kv.size() }

    fn insert (+K: K, +V: V) -> bool {
        self.vk.insert(V, K);
        self.kv.insert(K, V)
    }

    fn contains_key (K: K) -> bool { self.kv.contains_key(K) }
    fn contains_key_ref(K: &K) -> bool { self.kv.contains_key_ref(K) }

    fn get (K: K) -> V { self.kv.get(K) }
    pure fn find (K: K) -> Option<V> { self.kv.find(K) }

    fn remove (K: K) -> bool {
        let v = self.kv.find(K);
        match v {
            None => { return false; }
            Some(vv) => {
                self.kv.remove(K);
                self.vk.remove(vv);
                return true;
            }
        }
    }

    pure fn each (mapfn: fn(K, V) -> bool) { self.kv.each(mapfn) }
    pure fn each_key (mapfn: fn(K) -> bool) { self.kv.each_key(mapfn) }
    pure fn each_value (mapfn: fn(V) -> bool) { self.vk.each_key(mapfn) }
    pure fn each_ref (mapfn: fn(&K, &V) -> bool) { self.kv.each_ref(mapfn) }
    pure fn each_key_ref (mapfn: fn(&K) -> bool) { self.kv.each_key_ref(mapfn) }
    pure fn each_value_ref (mapfn: fn(&V) -> bool) { self.vk.each_key_ref(mapfn) }

    fn clear () { self.kv.clear(); self.vk.clear() }
}

trait bimap<K: Const Copy, V: Const Copy> {
    fn contains_value (V) -> bool;
    fn get_key (V) -> K;
    fn find_key (V) -> Option<K>;
    fn remove_key (V) -> bool;
}

impl<K: Const Copy Eq IterBytes Hash, V: Const Copy Eq IterBytes Hash> hashbimap<K, V> : bimap<K,V> {
    fn contains_value (V: V) -> bool { self.vk.contains_key(V) }
    fn get_key (V: V) -> K { self.vk.get(V) }
    fn find_key (V: V) -> Option<K> { self.vk.find(V) }
    fn remove_key (V: V) -> bool {
        let k = self.vk.find(V);
        match k {
            None => { return false }
            Some(kk) => {
                self.vk.remove(V);
                self.kv.remove(kk);
                return true;
            }
        }
    }
}

impl<K: Copy Eq IterBytes Hash, V: Copy> hashbimap<K, V> : ops::Index<K, V> {
    pure fn index(+index: K) -> V {
        self.kv[index]
    }
}

#[doc="Cannot have impl as ret type in iface"]
fn reverse<K, V> (self: hashbimap<K, V>) -> hashbimap<V, K> { 
    {kv: self.vk, vk: self.kv}
}

fn bimap<K: Const Copy Eq IterBytes Hash, V: Const Copy Eq IterBytes Hash> () -> hashbimap<K, V> {
    {
        kv: HashMap::<K, V>(), 
        vk: HashMap::<V, K>() 
    }
}

#[cfg(test)]
mod test {
    fn checkRep<K: Const Copy Eq IterBytes Hash, V: Const Copy Eq IterBytes Hash> (+bimap: hashbimap<K, V>) {
        assert bimap.vk.size() == bimap.kv.size();

        // For each key, the value matches the key.
        bimap.kv.each(|K, V| { assert bimap.get_key(V) == K; true });
    }

    #[test]
    fn test_creation () {
        let bimap: hashbimap<int, @str> = bimap::<int, @str>();
        checkRep(bimap);

        bimap.insert(0, @"abc");
        checkRep(bimap);

        bimap.insert(1, @"def");
        checkRep(bimap);

        assert bimap.get(0) == @"abc";
        assert bimap.get(1) == @"def";

        assert bimap.get_key(@"abc") == 0;
        assert bimap.get_key(@"def") == 1;
    }

    /* Tests from stdlib/map, co-opted into testing bimap */

    #[test]
    fn test_simple() {
        #debug("*** starting test_simple");
        fn eq_uint(&&x: uint, &&y: uint) -> bool { return x == y; }
        fn uint_id(&&x: uint) -> uint { x }
        #debug("uint -> uint");
        let hbm_uu: hashbimap<uint, uint> = bimap::<uint, uint>();
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
        let ten: @str = @"ten";
        let eleven: @str = @"eleven";
        let twelve: @str = @"twelve";
        #debug("str -> uint");
        let hbm_su: hashbimap<@str, uint> = 
            bimap::<@str, uint>();
        assert (hbm_su.insert(@"ten", 12u));
        assert (hbm_su.insert(eleven, 13u));
        assert (hbm_su.insert(@"twelve", 14u));
        assert (hbm_su.get(eleven) == 13u);
        assert (hbm_su.get(@"eleven") == 13u);
        assert (hbm_su.get(@"twelve") == 14u);
        assert (hbm_su.get(@"ten") == 12u);
        assert (!hbm_su.insert(@"twelve", 14u));
        assert (hbm_su.get(@"twelve") == 14u);
        assert (!hbm_su.insert(@"twelve", 12u));
        assert (hbm_su.get(@"twelve") == 12u);
        #debug("uint -> str");
        let hbm_us: hashbimap<uint, @str> =
            bimap::<uint, @str>();
        assert (hbm_us.insert(10u, @"twelve"));
        assert (hbm_us.insert(11u, @"thirteen"));
        assert (hbm_us.insert(12u, @"fourteen"));
        assert (hbm_us.get(11u) == @"thirteen");
        assert (hbm_us.get(12u) == @"fourteen");
        assert (hbm_us.get(10u) == @"twelve");
        assert (!hbm_us.insert(12u, @"fourteen"));
        assert (hbm_us.get(12u) == @"fourteen");
        assert (!hbm_us.insert(12u, @"twelve"));
        assert (hbm_us.get(12u) == @"twelve");
        #debug("str -> str");
        let hbm_ss: hashbimap<@str, @str> =
            bimap::<@str, @str>();
        assert (hbm_ss.insert(ten, @"twelve"));
        assert (hbm_ss.insert(eleven, @"thirteen"));
        assert (hbm_ss.insert(twelve, @"fourteen"));
        assert (hbm_ss.get(@"eleven") == @"thirteen");
        assert (hbm_ss.get(@"twelve") == @"fourteen");
        assert (hbm_ss.get(@"ten") == @"twelve");
        assert (!hbm_ss.insert(@"twelve", @"fourteen"));
        assert (hbm_ss.get(@"twelve") == @"fourteen");
        assert (!hbm_ss.insert(@"twelve", @"twelve"));
        assert (hbm_ss.get(@"twelve") == @"twelve");
        #debug("*** finished test_simple");
    }

    /**
    * Force map growth
    */
    #[test]
    fn test_growth() {
        #debug("*** starting test_growth");
        let num_to_insert: uint = 64u;
        fn eq_uint(&&x: uint, &&y: uint) -> bool { return x == y; }
        fn uint_id(&&x: uint) -> uint { x }
        #debug("uint -> uint");
        let hbm_uu: hashbimap<uint, uint> = bimap::<uint, uint>();
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
        let hbm_ss: hashbimap<@~str, @~str> =
            bimap::<@~str, @~str>();
        i = 0u;
        while i < num_to_insert {
            assert hbm_ss.insert(@uint::to_str(i, 2u), 
                @uint::to_str(i * i, 2u));
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
                   *hbm_ss.get(@uint::to_str(i, 2u)));
            assert (hbm_ss.get(@uint::to_str(i, 2u)) == @uint::to_str(i * i, 2u));
            i += 1u;
        }
        assert (hbm_ss.insert(@uint::to_str(num_to_insert, 2u),
                             @uint::to_str(17u, 2u)));
        assert (hbm_ss.get(@uint::to_str(num_to_insert, 2u)) == @uint::to_str(17u, 2u));
        #debug("-----");
        i = 0u;
        while i < num_to_insert {
            #debug("get(\"%s\") = \"%s\"",
                   uint::to_str(i, 2u),
                   *hbm_ss.get(@uint::to_str(i, 2u)));
            assert (hbm_ss.get(@uint::to_str(i, 2u)) == @uint::to_str(i * i, 2u));
            i += 1u;
        }
        #debug("*** finished test_growth");
    }

    #[test]
    fn test_removal() {
        #debug("*** starting test_removal");
        let num_to_insert: uint = 64u;
        fn eq(&&x: uint, &&y: uint) -> bool { return x == y; }
        fn hash(&&u: uint) -> uint {
            // This hash function intentionally causes collisions between
            // consecutive integer pairs.

            return u / 2u * 2u;
        }
        assert (hash(0u) == hash(1u));
        assert (hash(2u) == hash(3u));
        assert (hash(0u) != hash(2u));
        let hbm: hashbimap<uint, uint> = 
            bimap::<uint, uint>();
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
            assert v;
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
        let key = @"k";
        let map = bimap::<@str, @str>();
        assert (!map.contains_key(key));
        map.insert(key, @"val");
        assert (map.contains_key(key));
    }

    #[test]
    fn test_find() {
        let key = @"k";
        let map = bimap::<@str, @str>();
        assert (option::is_none(&map.find(key)));
        map.insert(key, @"val");
        assert (option::get(&map.find(key)) == @"val");
    }

    #[test]
    fn test_clear() {
        let key = @"k";
        let map = bimap::<@str, @str>();
        map.insert(key, @"val");
        assert (map.size() == 1);
        assert (map.contains_key(key));
        map.clear();
        assert (map.size() == 0);
        assert (!map.contains_key(key));
    }
}