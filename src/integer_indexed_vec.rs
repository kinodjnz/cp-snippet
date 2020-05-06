use cargo_snippet::snippet;

#[snippet("_integer_indexed_vec_struct")]
#[derive(Debug, Clone)]
struct IntegerIndexedVec<T> {
    v: Vec<T>,
    offset: usize,
}

#[snippet("_integer_indexed_vec_impl", include = "_integer_indexed_vec_struct")]
impl<T> IntegerIndexedVec<T> {
    fn new(v: Vec<T>, offset: usize) -> Self {
        IntegerIndexedVec {
            v,
            offset,
        }
    }

    #[allow(dead_code)]
    fn iter(&self) -> std::slice::Iter<T> {
        self.v.iter()
    }
}

#[snippet("_integer_indexed_vec_into_iterator", include = "_integer_indexed_vec_impl")]
impl<T> IntoIterator for IntegerIndexedVec<T> {
    type Item = T;
    type IntoIter = std::vec::IntoIter<T>;

    #[warn(dead_code)]
    fn into_iter(self) -> std::vec::IntoIter<T> {
        self.v.into_iter()
    }
}

#[snippet("_integer_indexed_vec_index", include = "_integer_indexed_vec_into_iterator")]
impl<T> std::ops::Index<i32> for IntegerIndexedVec<T> {
    type Output = T;

    #[warn(dead_code)]
    fn index(&self, index: i32) -> &Self::Output {
        let actual_index = index + (self.offset as i32);
        &self.v[actual_index as usize]
    }
}

#[snippet("_integer_indexed_vec_index_mut", include = "_integer_indexed_vec_index")]
impl<T> std::ops::IndexMut<i32> for IntegerIndexedVec<T> {
    #[warn(dead_code)]
    fn index_mut(&mut self, index: i32) -> &mut T {
        let actual_index = index + (self.offset as i32);
        &mut self.v[actual_index as usize]
    }
}

#[snippet("_integer_indexed_vec_trait", include = "_integer_indexed_vec_index_mut")]
trait IntoIntegerIndexedVec {
    type Output;

    #[warn(dead_code)]
    fn into_i32_indexed_vec(self, offset: usize) -> IntegerIndexedVec<Self::Output>;
}

#[snippet("i32_indexed_vec", include = "_integer_indexed_vec_trait")]
impl<T> IntoIntegerIndexedVec for Vec<T> {
    type Output = T;

    #[warn(dead_code)]
    fn into_i32_indexed_vec(self, offset: usize) -> IntegerIndexedVec<T> {
        IntegerIndexedVec::new(self, offset)
    }
}

#[test]
fn test_i32_indexed_vec() {
    let v = std::iter::successors(Some(0), |&n| Some(n+1))
        .take(10)
        .collect::<Vec<_>>().into_i32_indexed_vec(5);
    assert_eq!(v[-5], 0);
    assert_eq!(v[4], 9);
}
