pub struct Matrix<T>{
    content: Vec<T>,
    row_len: usize,
    col_len: usize
}

impl<T> Matrix<T>{
    pub fn from_iter<It>(iter: It, row_len: usize,col_len: usize) -> Result<Self,()>
    where It: Iterator, Vec<T>: FromIterator<<It as Iterator>::Item>
    {
        let content:Vec<T> = iter.collect();
        if content.len() == row_len * col_len{
            return Ok(Matrix { content, row_len, col_len });
        }
        return Err(());
    }

    pub fn get(&self, r: usize, c: usize) -> Option<&T>{
        if (self.row_len > r) && (self.col_len > c){
            let idx = (r*self.col_len) + c;
            return Some(&self.content[idx]);
        }
        return None;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new() {
        //  0  1  2  3
        //  4  5  6  7
        //  8  9 10 11
        let iter = 0..12;
        let mat = Matrix::from_iter(iter,3,4).unwrap();
        assert_eq!(5, *mat.get(1,1).unwrap());
        assert_eq!(6, *mat.get(1,2).unwrap());

        
    }
}