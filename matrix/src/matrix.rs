

pub struct Matrix<T>{
    content: Vec<Vec<T>>,
    row_len: usize,
    col_len: usize
}

impl<T:Default+Clone> Matrix<T>{
    // pub fn from_iter<It>(iter: It, row_len: usize,col_len: usize) -> Result<Self,()>
    // where It: Iterator, Vec<T>: FromIterator<<It as Iterator>::Item>
    // {
    //     let content:Vec<T> = iter.collect();
    //     if content.len() == row_len * col_len{
    //         return Ok(Matrix { content, row_len, col_len });
    //     }
    //     return Err(());
    // }
    pub fn new(row_len: usize, col_len: usize) -> Self{
        let content = vec![vec![T::default(); col_len]; row_len];
        Self { content, row_len, col_len }
    }

    pub fn from_vv(table: Vec<Vec<T>>) -> Result<Self,()>{
        let row_len = table.len();

        let col_len = table[0].len();
        // Verify all rows are same length
        for i in 1..row_len{
            let row = &table[i];
            if col_len != row.len(){
                return Err(());
            }
        }
        Ok(Self { content: table, row_len, col_len })
    }

    pub fn get(&self, r: usize, c: usize) -> Option<&T>{
        if (self.row_len > r) && (self.col_len > c){
            return Some(&self.content[r][c]);
        }
        return None;
    }

    pub fn row_len(&self) -> usize{
        self.row_len
    }

    pub fn col_len(&self) -> usize{
        self.col_len
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
        let table = vec![
            vec![0,1,2,3],
            vec![4,5,6,7],
            vec![8,9,10,11]
        ];
        let mat = Matrix::from_vv(table).unwrap();
        assert_eq!(5, *mat.get(1,1).unwrap());
        assert_eq!(6, *mat.get(1,2).unwrap());
        assert_eq!(3, mat.row_len());
        assert_eq!(4, mat.col_len());        
    }
}