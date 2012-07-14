This is an implementation of a bimap done using two hashmaps.

I cannot speak to its speed or size or any characteristics you care about if 
you care about code metrics. Use this when you need a bimap, and performance 
doesn't matter. 

If another bimap implementation comes outs by somebody else,
it'll probably be better than this. If you want to make a better one, feel
free to decide upon iface characteristics. If you want to improve this one,
send a pull request.

## Iface: bimap

+ get_key(V) -> K;
+ find_key(V) -> option<K>;
+ remove_key(V) -> option<K>;

These three methods are the same as their map counterparts, except they return
or remove entries based on known values.

In this library, only hashbimap implements bimap.

## Type: hashbimap

- Implements std::map::map
- Implements bimap::bimap
- { kv: hashmap<K, V>, vk: hashmap<V, K> }

Basic implementation of a bimap using two hashmaps. Not very efficient, but
real easy to write.

## Functions

fn bimap<K: const copy, V: const copy> (fn (K) -> uint, fn (K, K) -> bool,
fn (V) -> uint, fn (V, V) -> bool) -> hashbimap<K, V>

The functions in order, are hash_key, eql_key, hash_value, eql_value.