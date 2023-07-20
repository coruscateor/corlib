

///
/// A key and a value, paried.
/// 
pub struct KeyValuePair<K, V>
    where K: PartialEq
{

    key: K,
    value: V

}

impl<K, V> KeyValuePair<K, V>
where K: PartialEq
{

    pub fn new(key: K, value: V) -> Self
    {

        Self
        {

            key,
            value

        }

    }

    pub fn get_key_ref(&self) -> &K
    {

        &self.key

    }

    pub fn get_value_ref(&self) -> &V
    {

        &self.value

    }

    pub fn get_value_mut(&mut self) -> &mut V
    {

        &mut self.value

    }
    
}

impl<K, V> PartialEq for KeyValuePair<K, V>
    where K: PartialEq
{

    fn eq(&self, other: &Self) -> bool
    {

        self.key == other.key

    }

}

