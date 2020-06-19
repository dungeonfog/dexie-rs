use js_sys::{Function, Promise};
use wasm_bindgen::prelude::*;

#[wasm_bindgen(module = "dexie")]
extern "C" {
    #[wasm_bindgen(js_name = default)]
    pub type Dexie;

    #[wasm_bindgen(constructor, js_class = default)]
    pub fn new(name: JsValue) -> Dexie;

    #[wasm_bindgen(method, js_class = default)]
    pub fn close(this: &Dexie);

    #[wasm_bindgen(method, js_class = default)]
    pub fn open(this: &Dexie) -> Promise;

    #[wasm_bindgen(method, js_class = default)]
    pub fn version(this: &Dexie, version_number: u32) -> Version;

    #[wasm_bindgen(method, js_class = default)]
    pub fn table(this: &Dexie, store_name: JsValue) -> Table;

    #[wasm_bindgen(method, js_class = default)]
    pub fn delete(this: &Dexie) -> Promise;

    #[wasm_bindgen(method, js_class = default)]
    pub fn transaction(this: &Dexie, mode: JsValue, tables: JsValue, callback: JsValue) -> Promise;

    #[wasm_bindgen(method)]
    pub fn on(this: &Dexie, event_type: &str, subscriberFunction: &Closure<dyn FnMut(JsValue, JsValue)>) -> Dexie;

    pub type Version;

    #[wasm_bindgen(method)]
    pub fn stores(this: &Version, schema_definition: JsValue) -> Version;

    #[wasm_bindgen(method)]
    pub fn upgrade(this: &Version, upgrade_function: Function) -> Version;

    pub type Table;

    #[wasm_bindgen(method)]
    pub fn add(this: &Table, item: JsValue) -> Promise;

    #[wasm_bindgen(method)]
    pub fn put(this: &Table, item: JsValue) -> Promise;

    #[wasm_bindgen(method)]
    pub fn get(this: &Table, primary_key: JsValue) -> Promise;

    #[wasm_bindgen(method, js_name = get)]
    pub fn get_with_filter(this: &Table, primary_key: JsValue, filter: JsValue) -> Promise;

    #[wasm_bindgen(method)]
    pub fn delete(this: &Table, primary_key: JsValue) -> Promise;

    #[wasm_bindgen(method)]
    pub fn clear(this: &Table) -> Promise;

    #[wasm_bindgen(method)]
    pub fn filter(this: &Table, filter_function: Function) -> Collection;

    #[wasm_bindgen(method, js_name = toCollection)]
    pub fn to_collection(this: &Table) -> Collection;
    
    #[wasm_bindgen(method, js_name = toArray)]
    pub fn to_array(this: &Table) -> Promise;

    #[wasm_bindgen(method)]
    pub fn count(this: &Table) -> Promise;

    #[wasm_bindgen(method, js_name = where)]
    pub fn where_with_object(this: &Table, criterias: JsValue) -> Collection;

    #[wasm_bindgen(method, js_name = where)]
    pub fn where_with_string(this: &Table, key_path_array: JsValue) -> WhereClause;

    #[wasm_bindgen(method, js_name = bulkAdd)]
    pub fn bulk_add(this: &Table, items: JsValue) -> Promise;

    #[wasm_bindgen(method, js_name = bulkPut)]
    pub fn bulk_put(this: &Table, items: JsValue) -> Promise;
    
    #[wasm_bindgen(method, js_name = bulkDelete)]
    pub fn bulk_delete(this: &Table, keys: JsValue) -> Promise;

    #[wasm_bindgen(method, js_name = bulkGet)]
    pub fn bulk_get(this: &Table, keys: JsValue) -> Promise;

    #[wasm_bindgen(method, js_name = orderBy)]
    pub fn order_by(this: &Table, key: JsValue) -> Collection;

    #[wasm_bindgen(method)]
    pub fn limit(this: &Table, limit: JsValue) -> Collection;

    #[wasm_bindgen(method)]
    pub fn reverse(this: &Table) -> Collection;

    pub type Collection;

    #[wasm_bindgen(method)]
    pub fn and(this: &Collection, filter: Function) -> Collection;

    #[wasm_bindgen(method)]
    pub fn clone(this: &Collection) -> Collection;

    #[wasm_bindgen(method)]
    pub fn count(this: &Collection) -> Promise;

    #[wasm_bindgen(method)]
    pub fn delete(this: &Collection) -> Promise;

    #[wasm_bindgen(method)]
    pub fn distinct(this: &Collection) -> Collection;

    #[wasm_bindgen(method)]
    pub fn each(this: &Collection, callback: Function) -> Promise;

    #[wasm_bindgen(method, js_name = eachKey)]
    pub fn each_key(this: &Collection, callback: Function) -> Promise;

    #[wasm_bindgen(method, js_name = eachPrimaryKey)]
    pub fn each_primary_key(this: &Collection, callback: Function) -> Promise;

    #[wasm_bindgen(method, js_name = eachUniqueKey)]
    pub fn each_unique_key(this: &Collection, callback: Function) -> Promise;

    #[wasm_bindgen(method)]
    pub fn filter(this: &Collection, filter: Function) -> Collection;

    #[wasm_bindgen(method)]
    pub fn first(this: &Collection) -> Promise;

    #[wasm_bindgen(method)]
    pub fn last(this: &Collection) -> Promise;

    #[wasm_bindgen(method)]
    pub fn keys(this: &Collection) -> Promise;

    #[wasm_bindgen(method, js_name = primaryKeys)]
    pub fn primary_keys(this: &Collection) -> Promise;

    #[wasm_bindgen(method, js_name = uniqueKeys)]
    pub fn unique_keys(this: &Collection) -> Promise;

    #[wasm_bindgen(method)]
    pub fn limit(this: &Collection, count: JsValue) -> Collection;
    
    #[wasm_bindgen(method)]
    pub fn modify(this: &Collection, changes: JsValue) -> Promise;
    
    #[wasm_bindgen(method, js_name = modify)]
    pub fn modify_fn(this: &Collection, changes: Function) -> Promise;

    #[wasm_bindgen(method)]
    pub fn offset(this: &Collection, count: JsValue) -> Collection;

    #[wasm_bindgen(method)]
    pub fn or(this: &Collection, index_or_primary_key: JsValue) -> WhereClause;

    #[wasm_bindgen(method)]
    pub fn reverse(this: &Collection) -> Collection;

    #[wasm_bindgen(method, js_name = sortBy)]
    pub fn sort_by(this: &Collection, key_path: JsValue) -> Promise;

    #[wasm_bindgen(method, js_name = toArray)]
    pub fn to_array(this: &Collection) -> Promise;

    #[wasm_bindgen(method)]
    pub fn until(this: &Collection, filter: Function, include_stop_entry: JsValue) -> Collection;

    pub type WhereClause;

    #[wasm_bindgen(method)]
    pub fn above(this: &WhereClause, lower_bound: JsValue) -> Collection;

    #[wasm_bindgen(method, js_name = aboveOrEqual)]
    pub fn above_or_equal(this: &WhereClause, lower_bound: JsValue) -> Collection;

    #[wasm_bindgen(method)]
    pub fn below(this: &WhereClause, upper_bound: JsValue) -> Collection;

    #[wasm_bindgen(method, js_name = belowOrEqual)]
    pub fn below_or_equal(this: &WhereClause, upper_bound: JsValue) -> Collection;

    #[wasm_bindgen(method, js_name = anyOf)]
    pub fn any_of(this: &WhereClause, array: JsValue) -> Collection;

    #[wasm_bindgen(method, js_name = anyOfIgnoreCase)]
    pub fn any_of_ignore_case(this: &WhereClause, array: JsValue) -> Collection;

    #[wasm_bindgen(method)]
    pub fn between(this: &WhereClause, lower_bound: JsValue, upper_bound: JsValue, include_lower: JsValue, include_upper: JsValue) -> Collection;

    #[wasm_bindgen(method)]
    pub fn equals(this: &WhereClause, key: JsValue) -> Collection;

    #[wasm_bindgen(method, js_name = equalsIgnoreCase)]
    pub fn equals_ignore_case(this: &WhereClause, key: JsValue) -> Collection;

    #[wasm_bindgen(method, js_name = noneOf)]
    pub fn none_of(this: &WhereClause, keys: JsValue) -> Collection;

    #[wasm_bindgen(method, js_name = notEqual)]
    pub fn not_equal(this: &WhereClause, key: JsValue) -> Collection;

    #[wasm_bindgen(method, js_name = startsWith)]
    pub fn starts_with(this: &WhereClause, prefix: JsValue) -> Collection;

    #[wasm_bindgen(method, js_name = startsWithIgnoreCase)]
    pub fn starts_with_ignore_case(this: &WhereClause, prefix: JsValue) -> Collection;

    #[wasm_bindgen(method, js_name = startsWithAnyOf)]
    pub fn starts_with_any_of(this: &WhereClause, array: JsValue) -> Collection;

    #[wasm_bindgen(method, js_name = startsWithAnyOfIgnoreCase)]
    pub fn starts_with_any_of_ignore_case(this: &WhereClause, array: JsValue) -> Collection;

    #[wasm_bindgen(method, js_name = inAnyRange)]
    pub fn in_any_range(this: &WhereClause, ranges: JsValue, options: JsValue) -> Collection;

    pub type Transaction;

    #[wasm_bindgen(method)]
    pub fn table(this: &Transaction, name: JsValue) -> Table;

    #[wasm_bindgen(method)]
    pub fn abort(this: &Transaction);
}

#[wasm_bindgen(module = "dexie-observable")]
extern "C" {
    #[no_mangle]
    #[wasm_bindgen(js_name = default)]
    pub fn observable(db: &Dexie); // don't actually call this, this is only so that dexie-observable is included by wasm-bindgen
}
