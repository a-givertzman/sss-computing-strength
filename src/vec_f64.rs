
///
#[derive(Debug, Clone, PartialEq)]
pub struct VecF64 {
    pub data: Vec<f64>,
}

impl VecF64 {
    ///
    pub fn new(data: Vec<f64>) -> Self {
        Self {data}
    }

    ///
    pub fn len(&self) -> usize {
        self.data.len()
    }

    ///
    pub fn iter(&self) -> core::slice::Iter<'_, f64> {
        self.data.iter()
    }

    ///
    pub fn iter_mut(&mut self) -> core::slice::IterMut<'_, f64> {
        self.data.iter_mut()
    }

    ///
    pub fn into_iter(self) -> <Vec<f64> as IntoIterator>::IntoIter {
        self.data.into_iter()
    }

    ///сумма сверху $res_i = res_{i-1} + src_i, res_0 = 0$
    pub fn sum_above(&self) -> Self {
        let mut data: Vec<f64> = vec![0.];
        self.data
            .iter()
            .for_each(|&t| data.push(*data.iter().last().unwrap_or(&0.) + t));
        Self::new(data)
    }

    ///интегральная сумма $res_i = res_{i-1} + src_{i-1} + src_i, res_0 = 0$
    pub fn integral_sum(&self) -> Self {
        let mut data = vec![0.];
        for i in 1..(self.data.len()) {
            data.push(data[i - 1] + self.data[i - 1] + self.data[i]);
        }
        Self::new(data)
    }

    ///
    pub fn add(&mut self, rhs: f64) {
        self.data.iter_mut()
           .for_each(|mut v| {*v += rhs;} )
    }

    ///
    pub fn sub(&mut self, rhs: f64) {
        self.data.iter_mut()
           .for_each(|mut v| {*v -= rhs;} )
    }

    ///
    pub fn div(&mut self, rhs: f64) {
        assert_ne!(rhs, 0.);
        self.data.iter_mut()
           .for_each(|mut v| {*v /= rhs;} )
    }

    ///
    pub fn mul(&mut self, rhs: f64) {
        self.data.iter_mut()
            .for_each(|mut v| {*v *= rhs;} )
    }
}

impl std::ops::Index<usize> for VecF64 {
    type Output = f64;
    ///
    fn index(&self, index: usize) -> &Self::Output {
        &self.data[index]
    }
}

impl std::ops::Add for VecF64 {
    type Output = Self;    
    ///
    fn add(self, rhs: Self) -> Self {
        VecF64 {
            data: self.data.into_iter()
            .zip(rhs.data.into_iter())
            .map(|(v1, v2)| v1 + v2)
            .collect()
        }
    }
}

impl std::ops::Sub for VecF64 {
    type Output = Self;
    ///
    fn sub(self, rhs: Self) -> Self {
        Self::new(self.data.into_iter()
            .zip(rhs.data.into_iter())
            .map(|(v1, v2)| {
                v1 - v2
            })
            .collect())
    }
}

impl std::ops::Div for VecF64 {
    type Output = Self;
    ///
    fn div(self, rhs: Self) -> Self {
        Self::new(self.data.into_iter()
            .zip(rhs.data.into_iter())
            .map(|(v1, v2)| {
                assert_ne!(v2, 0.);
                v1 / v2
            })
            .collect())
    }
}

impl std::ops::Mul for VecF64 {
    type Output = Self;
    ///
    fn mul(self, rhs: Self) -> Self {
        Self::new(self.data.into_iter()
            .zip(rhs.data.into_iter())
            .map(|(v1, v2)| v1 * v2)
            .collect())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sum_above() {
        let v1 = VecF64::new(vec![1., 2., 3.,]).sum_above();
        let v2 = VecF64::new(vec![0., 1., 3., 6.,]);
        assert_eq!(v1, v2);   
    }

    #[test]
    fn integral_sum() {
        let v1 = VecF64::new(vec![0., 1., 2., 3.,]).integral_sum();
        let v2 = VecF64::new(vec![0., 1., 4., 9.,]);
        assert_eq!(v1, v2);   
    }

    #[test]
    pub fn add() {
        let mut v1 = VecF64::new(vec![0., 1.,]);
        v1.add(-1.);
        let v2 = VecF64::new(vec![-1., 0.,]);
        assert_eq!(v1, v2);   
        let v1 = VecF64::new(vec![0., 1.,]);
        assert_eq!(v1 + v2, VecF64::new(vec![-1., 1.,]));   
    }

    #[test]
    pub fn sub() {
        let mut v1 = VecF64::new(vec![0., 1.,]);
        v1.sub(1.);
        let v2 = VecF64::new(vec![-1., 0.,]);
        assert_eq!(v1, v2);   
        let v1 = VecF64::new(vec![0., 1.,]);
        assert_eq!(v1 - v2, VecF64::new(vec![1., 1.,]));  
    }

    #[test]
    pub fn div() {
        let mut v1 = VecF64::new(vec![-4., 4.,]);
        v1.div(-2.);
        let v2 = VecF64::new(vec![2., -2.,]);
        assert_eq!(v1, v2);   
        let v1 = VecF64::new(vec![-4., 4.,]);
        assert_eq!(v1 / v2, VecF64::new(vec![-2., -2.,])); 
    }

    #[test]
    pub fn mul() {
        let mut v1 = VecF64::new(vec![0., 1.,]);
        v1.mul(-2.);
        let v2 = VecF64::new(vec![0., -2.,]);
        assert_eq!(v1, v2);   
        let v1 = VecF64::new(vec![0., 1.,]);
        assert_eq!(v1 * v2, VecF64::new(vec![0., -2.,])); 
    }
}
