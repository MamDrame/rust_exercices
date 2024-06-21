use std::fmt;

#[derive(Debug, Clone)]
pub struct Matrix(pub Vec<Vec<i32>>);

impl Matrix {
    pub fn new(slice: &[&[i32]]) -> Self {
        let mut result: Vec<Vec<i32>> = Vec::new();
        slice.iter().for_each(|&row| result.push(Vec::from(row)));
        Matrix(result)
    }
}

impl fmt::Display for Matrix {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let a = self.0.iter()
            .map(|row| format!("({})", row.iter().map(|x| x.to_string()).collect::<Vec<String>>().join(" ")))
            .collect::<Vec<String>>()
            .join("\n");
        
        write!(f, "{}", a)
    }
}
